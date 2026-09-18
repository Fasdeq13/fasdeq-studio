use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Pacman,
    Zypper,
    Apk,
    Xbps,
    Emerge,
    Nix,
    Eopkg,
    Urpmi,
    Slackpkg,
    Pkg,
    Swupd,
    Guix,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct DistroInfo {
    pub id: String,
    pub name: String,
    pub id_like: Vec<String>,
    pub package_manager: PackageManager,
}

pub fn detect_distro() -> DistroInfo {
    let content = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let mut fields: HashMap<String, String> = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim().trim_matches('"').to_string();
            fields.insert(key.trim().to_string(), value);
        }
    }

    let id = fields.get("ID").cloned().unwrap_or_else(|| "linux".into());
    let name = fields
        .get("PRETTY_NAME")
        .cloned()
        .unwrap_or_else(|| "Unknown Linux".into());
    let id_like: Vec<String> = fields
        .get("ID_LIKE")
        .map(|s| s.split_whitespace().map(|x| x.to_string()).collect())
        .unwrap_or_default();

    let package_manager = resolve_package_manager(&id, &id_like);

    DistroInfo {
        id,
        name,
        id_like,
        package_manager,
    }
}

fn resolve_package_manager(id: &str, id_like: &[String]) -> PackageManager {
    let all_ids: Vec<&str> = std::iter::once(id)
        .chain(id_like.iter().map(|s| s.as_str()))
        .collect();

    for candidate in &all_ids {
        match *candidate {
            "ubuntu" | "debian" | "linuxmint" | "pop" | "elementary" | "zorin" | "kali"
            | "raspbian" | "deepin" | "mx" | "devuan" | "neon" => return PackageManager::Apt,
            "fedora" | "rhel" | "nobara" => return PackageManager::Dnf,
            "centos" | "rocky" | "almalinux" | "oracle" => return PackageManager::Dnf,
            "arch" | "manjaro" | "endeavouros" | "garuda" | "artix" | "arcolinux" => {
                return PackageManager::Pacman
            }
            "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" | "sles" => {
                return PackageManager::Zypper
            }
            "alpine" | "postmarketos" => return PackageManager::Apk,
            "void" => return PackageManager::Xbps,
            "gentoo" | "funtoo" => return PackageManager::Emerge,
            "nixos" => return PackageManager::Nix,
            "solus" => return PackageManager::Eopkg,
            "mageia" | "mandriva" => return PackageManager::Urpmi,
            "slackware" => return PackageManager::Slackpkg,
            "freebsd" => return PackageManager::Pkg,
            "clear-linux-os" => return PackageManager::Swupd,
            "guix" => return PackageManager::Guix,
            _ => continue,
        }
    }

    if which::which("apt-get").is_ok() || which::which("apt").is_ok() {
        PackageManager::Apt
    } else if which::which("dnf").is_ok() {
        PackageManager::Dnf
    } else if which::which("yum").is_ok() {
        PackageManager::Yum
    } else if which::which("pacman").is_ok() {
        PackageManager::Pacman
    } else if which::which("zypper").is_ok() {
        PackageManager::Zypper
    } else if which::which("apk").is_ok() {
        PackageManager::Apk
    } else if which::which("xbps-install").is_ok() {
        PackageManager::Xbps
    } else if which::which("emerge").is_ok() {
        PackageManager::Emerge
    } else if which::which("nix-env").is_ok() {
        PackageManager::Nix
    } else {
        PackageManager::Unknown
    }
}

impl PackageManager {
    pub fn display_name(&self) -> &'static str {
        match self {
            PackageManager::Apt => "APT",
            PackageManager::Dnf => "DNF",
            PackageManager::Yum => "YUM",
            PackageManager::Pacman => "Pacman",
            PackageManager::Zypper => "Zypper",
            PackageManager::Apk => "APK",
            PackageManager::Xbps => "XBPS",
            PackageManager::Emerge => "Portage",
            PackageManager::Nix => "Nix",
            PackageManager::Eopkg => "eopkg",
            PackageManager::Urpmi => "urpmi",
            PackageManager::Slackpkg => "slackpkg",
            PackageManager::Pkg => "pkg",
            PackageManager::Swupd => "swupd",
            PackageManager::Guix => "Guix",
            PackageManager::Unknown => "unknown",
        }
    }

    pub fn install_command(&self, packages: &[&str]) -> Vec<String> {
        let joined = packages.join(" ");
        match self {
            PackageManager::Apt => vec![
                "apt-get update".to_string(),
                format!("apt-get install -y {joined}"),
            ],
            PackageManager::Dnf => vec![format!("dnf install -y {joined}")],
            PackageManager::Yum => vec![format!("yum install -y {joined}")],
            PackageManager::Pacman => vec![format!("pacman -Sy --noconfirm {joined}")],
            PackageManager::Zypper => vec![format!("zypper install -y {joined}")],
            PackageManager::Apk => vec![format!("apk add --no-cache {joined}")],
            PackageManager::Xbps => vec![format!("xbps-install -Sy {joined}")],
            PackageManager::Emerge => vec![format!("emerge {joined}")],
            PackageManager::Nix => vec![format!("nix-env -iA {joined}")],
            PackageManager::Eopkg => vec![format!("eopkg install -y {joined}")],
            PackageManager::Urpmi => vec![format!("urpmi {joined}")],
            PackageManager::Slackpkg => vec![format!("slackpkg install {joined}")],
            PackageManager::Pkg => vec![format!("pkg install -y {joined}")],
            PackageManager::Swupd => vec![format!("swupd bundle-add {joined}")],
            PackageManager::Guix => vec![format!("guix install {joined}")],
            PackageManager::Unknown => vec![],
        }
    }

    pub fn package_name_for(&self, tool: &str) -> &'static str {
        match (self, tool) {
            (PackageManager::Apt, "gcc") => "gcc",
            (PackageManager::Apt, "g++") => "g++",
            (PackageManager::Apt, "make") => "make",
            (PackageManager::Apt, "gdb") => "gdb",
            (PackageManager::Apt, "nasm") => "nasm",
            (PackageManager::Apt, "as") => "binutils",
            (PackageManager::Apt, "objdump") => "binutils",
            (PackageManager::Apt, "qemu-system-x86_64") => "qemu-system-x86",
            (PackageManager::Apt, "cmake") => "cmake",
            (PackageManager::Apt, "git") => "git",

            (PackageManager::Dnf | PackageManager::Yum, "gcc") => "gcc",
            (PackageManager::Dnf | PackageManager::Yum, "g++") => "gcc-c++",
            (PackageManager::Dnf | PackageManager::Yum, "make") => "make",
            (PackageManager::Dnf | PackageManager::Yum, "gdb") => "gdb",
            (PackageManager::Dnf | PackageManager::Yum, "nasm") => "nasm",
            (PackageManager::Dnf | PackageManager::Yum, "as") => "binutils",
            (PackageManager::Dnf | PackageManager::Yum, "objdump") => "binutils",
            (PackageManager::Dnf | PackageManager::Yum, "qemu-system-x86_64") => "qemu-system-x86",
            (PackageManager::Dnf | PackageManager::Yum, "cmake") => "cmake",
            (PackageManager::Dnf | PackageManager::Yum, "git") => "git",

            (PackageManager::Pacman, "gcc") => "gcc",
            (PackageManager::Pacman, "g++") => "gcc",
            (PackageManager::Pacman, "make") => "make",
            (PackageManager::Pacman, "gdb") => "gdb",
            (PackageManager::Pacman, "nasm") => "nasm",
            (PackageManager::Pacman, "as") => "binutils",
            (PackageManager::Pacman, "objdump") => "binutils",
            (PackageManager::Pacman, "qemu-system-x86_64") => "qemu-system-x86",
            (PackageManager::Pacman, "cmake") => "cmake",
            (PackageManager::Pacman, "git") => "git",

            (PackageManager::Zypper, "gcc") => "gcc",
            (PackageManager::Zypper, "g++") => "gcc-c++",
            (PackageManager::Zypper, "make") => "make",
            (PackageManager::Zypper, "gdb") => "gdb",
            (PackageManager::Zypper, "nasm") => "nasm",
            (PackageManager::Zypper, "as") => "binutils",
            (PackageManager::Zypper, "objdump") => "binutils",
            (PackageManager::Zypper, "qemu-system-x86_64") => "qemu-system-x86",
            (PackageManager::Zypper, "cmake") => "cmake",
            (PackageManager::Zypper, "git") => "git",

            (PackageManager::Apk, "gcc") => "gcc",
            (PackageManager::Apk, "g++") => "g++",
            (PackageManager::Apk, "make") => "make",
            (PackageManager::Apk, "gdb") => "gdb",
            (PackageManager::Apk, "nasm") => "nasm",
            (PackageManager::Apk, "as") => "binutils",
            (PackageManager::Apk, "objdump") => "binutils",
            (PackageManager::Apk, "qemu-system-x86_64") => "qemu-system-x86_64",
            (PackageManager::Apk, "cmake") => "cmake",
            (PackageManager::Apk, "git") => "git",

            (PackageManager::Xbps, "gcc") => "gcc",
            (PackageManager::Xbps, "g++") => "gcc",
            (PackageManager::Xbps, "make") => "make",
            (PackageManager::Xbps, "gdb") => "gdb",
            (PackageManager::Xbps, "nasm") => "nasm",
            (PackageManager::Xbps, "as") => "binutils",
            (PackageManager::Xbps, "objdump") => "binutils",
            (PackageManager::Xbps, "qemu-system-x86_64") => "qemu",
            (PackageManager::Xbps, "cmake") => "cmake",
            (PackageManager::Xbps, "git") => "git",

            (_, "rustc") => "rustc",
            (_, "cargo") => "cargo",
            (_, other) => Box::leak(other.to_string().into_boxed_str()),
        }
    }
}
