use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub message: String,
}

pub fn parse_gcc_output(output: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for line in output.lines() {
        if let Some(diag) = parse_gcc_line(line) {
            diagnostics.push(diag);
        }
    }
    diagnostics
}

fn parse_gcc_line(line: &str) -> Option<Diagnostic> {
    let parts: Vec<&str> = line.splitn(4, ':').collect();

    if parts.len() >= 4 {
        if let Some(diag) = try_parse_four_part(&parts) {
            return Some(diag);
        }
    }

    // NASM and some other assemblers use `file:line: severity: message`
    // (three colon-separated parts, no column number).
    let parts3: Vec<&str> = line.splitn(3, ':').collect();
    if parts3.len() == 3 {
        if let Some(diag) = try_parse_three_part(&parts3) {
            return Some(diag);
        }
    }

    None
}

fn try_parse_four_part(parts: &[&str]) -> Option<Diagnostic> {
    let file = PathBuf::from(parts[0]);
    let line_no: usize = parts[1].trim().parse().ok()?;
    let rest = parts[2].trim();
    let (column, severity_and_message) = if let Ok(col) = rest.parse::<usize>() {
        (col, parts[3])
    } else {
        (0, parts[3])
    };

    let (severity, message) = extract_severity(severity_and_message)?;

    Some(Diagnostic {
        file,
        line: line_no,
        column,
        severity,
        message,
    })
}

fn try_parse_three_part(parts: &[&str]) -> Option<Diagnostic> {
    let file = PathBuf::from(parts[0]);
    let line_no: usize = parts[1].trim().parse().ok()?;
    let (severity, message) = extract_severity(parts[2])?;

    Some(Diagnostic {
        file,
        line: line_no,
        column: 0,
        severity,
        message,
    })
}

fn extract_severity(text: &str) -> Option<(Severity, String)> {
    let trimmed = text.trim();
    if let Some(m) = trimmed.strip_prefix("error:") {
        Some((Severity::Error, m.trim().to_string()))
    } else if let Some(m) = trimmed.strip_prefix("fatal error:") {
        Some((Severity::Error, m.trim().to_string()))
    } else if let Some(m) = trimmed.strip_prefix("warning:") {
        Some((Severity::Warning, m.trim().to_string()))
    } else if let Some(m) = trimmed.strip_prefix("note:") {
        Some((Severity::Note, m.trim().to_string()))
    } else {
        None
    }
}

pub fn parse_rustc_output(output: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut current_severity: Option<Severity> = None;
    let mut current_message = String::new();

    for line in output.lines() {
        let trimmed = line.trim_start();
        if let Some(msg) = trimmed.strip_prefix("error[") {
            current_severity = Some(Severity::Error);
            current_message = msg.splitn(2, ']').nth(1).unwrap_or(msg).trim_start_matches(':').trim().to_string();
        } else if let Some(msg) = trimmed.strip_prefix("error:") {
            current_severity = Some(Severity::Error);
            current_message = msg.trim().to_string();
        } else if let Some(msg) = trimmed.strip_prefix("warning:") {
            current_severity = Some(Severity::Warning);
            current_message = msg.trim().to_string();
        } else if trimmed.starts_with("-->") {
            if let Some(severity) = current_severity.clone() {
                let location = trimmed.trim_start_matches("-->").trim();
                let segments: Vec<&str> = location.rsplitn(3, ':').collect();
                if segments.len() == 3 {
                    let file = PathBuf::from(segments[2]);
                    let line_no: usize = segments[1].parse().unwrap_or(0);
                    let col_no: usize = segments[0].parse().unwrap_or(0);
                    diagnostics.push(Diagnostic {
                        file,
                        line: line_no,
                        column: col_no,
                        severity,
                        message: current_message.clone(),
                    });
                }
            }
            current_severity = None;
        }
    }

    diagnostics
}

pub struct BuildResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn run_build(project_root: &PathBuf, tool: &str) -> anyhow::Result<BuildResult> {
    let output = match tool {
        "cargo" => Command::new("cargo")
            .arg("build")
            .current_dir(project_root)
            .output()?,
        _ => Command::new("make").current_dir(project_root).output()?,
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let diagnostics = if tool == "cargo" {
        parse_rustc_output(&stderr)
    } else {
        parse_gcc_output(&stderr)
    };

    Ok(BuildResult {
        success: output.status.success(),
        stdout,
        stderr,
        diagnostics,
    })
}
