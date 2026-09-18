use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolKind {
    Rustc,
    Cargo,
    Gcc,
    Gxx,
    Make,
    Gdb,
    Nasm,
    As,
    Objdump,
    Ld,
    Qemu,
    Cmake,
    Git,
}

impl ToolKind {
    pub fn binary_name(&self) -> &'static str {
        match self {
            ToolKind::Rustc => "rustc",
            ToolKind::Cargo => "cargo",
            ToolKind::Gcc => "gcc",
            ToolKind::Gxx => "g++",
            ToolKind::Make => "make",
            ToolKind::Gdb => "gdb",
            ToolKind::Nasm => "nasm",
            ToolKind::As => "as",
            ToolKind::Objdump => "objdump",
            ToolKind::Ld => "ld",
            ToolKind::Qemu => "qemu-system-x86_64",
            ToolKind::Cmake => "cmake",
            ToolKind::Git => "git",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ToolKind::Rustc => "Rust Compiler",
            ToolKind::Cargo => "Cargo",
            ToolKind::Gcc => "GCC",
            ToolKind::Gxx => "G++",
            ToolKind::Make => "Make",
            ToolKind::Gdb => "GDB",
            ToolKind::Nasm => "NASM",
            ToolKind::As => "GNU Assembler",
            ToolKind::Objdump => "objdump",
            ToolKind::Ld => "Linker (ld)",
            ToolKind::Qemu => "QEMU",
            ToolKind::Cmake => "CMake",
            ToolKind::Git => "Git",
        }
    }

    pub fn required() -> Vec<ToolKind> {
        vec![
            ToolKind::Rustc,
            ToolKind::Cargo,
            ToolKind::Gcc,
            ToolKind::Gxx,
            ToolKind::Make,
            ToolKind::Gdb,
            ToolKind::Nasm,
            ToolKind::As,
            ToolKind::Objdump,
            ToolKind::Ld,
            ToolKind::Qemu,
            ToolKind::Cmake,
            ToolKind::Git,
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolStatus {
    pub kind: ToolKind,
    pub found: bool,
    pub path: Option<PathBuf>,
    pub version: Option<String>,
}

pub fn detect_tool(kind: ToolKind) -> ToolStatus {
    let binary = kind.binary_name();
    match which::which(binary) {
        Ok(path) => {
            let version = query_version(&path);
            ToolStatus {
                kind,
                found: true,
                path: Some(path),
                version,
            }
        }
        Err(_) => ToolStatus {
            kind,
            found: false,
            path: None,
            version: None,
        },
    }
}

pub fn detect_all() -> Vec<ToolStatus> {
    ToolKind::required()
        .into_iter()
        .map(detect_tool)
        .collect()
}

fn query_version(path: &PathBuf) -> Option<String> {
    let output = Command::new(path).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines().next().map(|s| s.to_string())
}

pub fn detect_tool_at_path(kind: ToolKind, path: PathBuf) -> ToolStatus {
    if path.exists() {
        let version = query_version(&path);
        ToolStatus {
            kind,
            found: true,
            path: Some(path),
            version,
        }
    } else {
        ToolStatus {
            kind,
            found: false,
            path: None,
            version: None,
        }
    }
}
