use super::detect::ToolKind;
use super::distro::{DistroInfo, PackageManager};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub enum InstallEvent {
    Started(String),
    Line(String),
    Finished(bool),
    Failed(String),
}

pub fn install_tools(distro: &DistroInfo, tools: &[ToolKind], sender: Sender<InstallEvent>) {
    if distro.package_manager == PackageManager::Unknown {
        let _ = sender.send(InstallEvent::Failed(
            "Could not determine a package manager for this distribution".into(),
        ));
        return;
    }

    let mut has_rust_tool = false;
    let mut system_packages: Vec<&str> = Vec::new();

    for tool in tools {
        match tool {
            ToolKind::Rustc | ToolKind::Cargo => has_rust_tool = true,
            other => {
                let pkg = distro.package_manager.package_name_for(other.binary_name());
                if !system_packages.contains(&pkg) {
                    system_packages.push(pkg);
                }
            }
        }
    }

    if !system_packages.is_empty() {
        let commands = distro.package_manager.install_command(&system_packages);
        for cmd in commands {
            let _ = sender.send(InstallEvent::Started(cmd.clone()));
            run_privileged_command(&cmd, &sender);
        }
    }

    if has_rust_tool {
        let _ = sender.send(InstallEvent::Started(
            "Installing Rust via the official rustup installer".into(),
        ));
        run_rustup_install(&sender);
    }

    let _ = sender.send(InstallEvent::Finished(true));
}

fn run_privileged_command(cmd: &str, sender: &Sender<InstallEvent>) {
    let full = format!("{cmd}");
    let mut child = match Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&full)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => match Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(&full)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = sender.send(InstallEvent::Failed(format!(
                    "Could not start the privileged installation: {e}"
                )));
                return;
            }
        },
    };

    stream_output(&mut child, sender);
}

fn run_rustup_install(sender: &Sender<InstallEvent>) {
    let script_cmd = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";
    let mut child = match Command::new("sh")
        .arg("-c")
        .arg(script_cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            let _ = sender.send(InstallEvent::Failed(format!(
                "Could not start the Rust installer: {e}"
            )));
            return;
        }
    };
    stream_output(&mut child, sender);
}

fn stream_output(child: &mut std::process::Child, sender: &Sender<InstallEvent>) {
    use std::io::{BufRead, BufReader};

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let _ = sender.send(InstallEvent::Line(line));
        }
    }
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            let _ = sender.send(InstallEvent::Line(line));
        }
    }

    match child.wait() {
        Ok(status) => {
            if !status.success() {
                let _ = sender.send(InstallEvent::Failed(format!(
                    "Command exited with an error, code: {:?}",
                    status.code()
                )));
            }
        }
        Err(e) => {
            let _ = sender.send(InstallEvent::Failed(format!("Error waiting for process: {e}")));
        }
    }
}
