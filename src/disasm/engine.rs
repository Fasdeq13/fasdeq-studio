use capstone::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    X86_16,
    X86_32,
    X86_64,
    Arm,
    Arm64,
}

impl Architecture {
    pub fn label(&self) -> &'static str {
        match self {
            Architecture::X86_16 => "x86 (16-bit)",
            Architecture::X86_32 => "x86 (32-bit)",
            Architecture::X86_64 => "x86-64",
            Architecture::Arm => "ARM",
            Architecture::Arm64 => "ARM64 (AArch64)",
        }
    }

    pub fn all() -> Vec<Architecture> {
        vec![
            Architecture::X86_16,
            Architecture::X86_32,
            Architecture::X86_64,
            Architecture::Arm,
            Architecture::Arm64,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct DisasmInstruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
}

pub fn build_capstone(arch: Architecture) -> anyhow::Result<Capstone> {
    let cs = match arch {
        Architecture::X86_16 => Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode16)
            .detail(true)
            .build()?,
        Architecture::X86_32 => Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode32)
            .detail(true)
            .build()?,
        Architecture::X86_64 => Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .detail(true)
            .build()?,
        Architecture::Arm => Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Arm)
            .detail(true)
            .build()?,
        Architecture::Arm64 => Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .detail(true)
            .build()?,
    };
    Ok(cs)
}

pub fn disassemble(
    bytes: &[u8],
    arch: Architecture,
    base_address: u64,
) -> anyhow::Result<Vec<DisasmInstruction>> {
    let cs = build_capstone(arch)?;
    let insns = cs
        .disasm_all(bytes, base_address)
        .map_err(|e| anyhow::anyhow!("Disassembly error: {e}"))?;

    let mut result = Vec::new();
    for insn in insns.as_ref() {
        result.push(DisasmInstruction {
            address: insn.address(),
            bytes: insn.bytes().to_vec(),
            mnemonic: insn.mnemonic().unwrap_or("?").to_string(),
            operands: insn.op_str().unwrap_or("").to_string(),
        });
    }
    Ok(result)
}
