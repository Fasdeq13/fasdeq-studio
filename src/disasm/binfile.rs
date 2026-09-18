use goblin::Object;

#[derive(Clone, Debug)]
pub struct SectionInfo {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub file_offset: u64,
}

impl SectionInfo {
    pub fn purpose(&self) -> &'static str {
        section_purpose(&self.name)
    }
}

pub fn section_purpose(name: &str) -> &'static str {
    match name {
        ".text" => "Executable machine code instructions.",
        ".rodata" | ".rdata" => "Read-only data: string literals, constants, jump tables.",
        ".data" => "Initialized global and static variables.",
        ".bss" => "Uninitialized global and static variables, zero-filled at load time and not stored in the file.",
        ".symtab" => "Symbol table used for linking and debugging.",
        ".strtab" => "String table backing symbol and section names.",
        ".shstrtab" => "String table for section header names.",
        ".dynsym" => "Dynamic symbol table for runtime linking.",
        ".dynstr" => "String table for dynamic symbols.",
        ".dynamic" => "Dynamic linking information used by the loader.",
        ".rel.text" | ".rela.text" => "Relocation entries applied to the .text section.",
        ".rel.data" | ".rela.data" => "Relocation entries applied to the .data section.",
        ".plt" => "Procedure Linkage Table, used to call functions from shared libraries.",
        ".got" | ".got.plt" => "Global Offset Table, holds resolved addresses for dynamic symbols.",
        ".init" => "Code run before main(), part of program startup.",
        ".fini" => "Code run after main() returns, part of program teardown.",
        ".init_array" => "Array of function pointers run before main().",
        ".fini_array" => "Array of function pointers run after main() returns.",
        ".comment" => "Compiler and toolchain version information, not loaded at runtime.",
        ".note.gnu.build-id" => "A unique build identifier embedded by the linker.",
        ".eh_frame" => "Exception handling and stack unwinding information.",
        ".debug_info" | ".debug_line" | ".debug_str" | ".debug_abbrev" => {
            "DWARF debug information used by debuggers such as GDB."
        }
        ".idata" => "Import table on PE binaries, lists functions imported from DLLs.",
        ".edata" => "Export table on PE binaries, lists functions exported by this module.",
        ".reloc" => "Base relocation table on PE binaries, used when the image is loaded at a non-preferred address.",
        ".pdata" => "Exception and unwind information table on PE (x64).",
        ".tls" => "Thread-local storage template.",
        _ => "General-purpose or toolchain-specific section.",
    }
}

#[derive(Clone, Debug)]
pub struct BinaryInfo {
    pub format: String,
    pub architecture: String,
    pub entry_point: u64,
    pub sections: Vec<SectionInfo>,
    pub is_64: bool,
}

pub fn analyze(data: &[u8]) -> anyhow::Result<BinaryInfo> {
    let object = Object::parse(data)?;
    match object {
        Object::Elf(elf) => {
            let sections = elf
                .section_headers
                .iter()
                .filter_map(|sh| {
                    let name = elf.shdr_strtab.get_at(sh.sh_name)?.to_string();
                    Some(SectionInfo {
                        name,
                        address: sh.sh_addr,
                        size: sh.sh_size,
                        file_offset: sh.sh_offset,
                    })
                })
                .collect();
            Ok(BinaryInfo {
                format: "ELF".to_string(),
                architecture: format!("{:?}", elf.header.e_machine),
                entry_point: elf.entry,
                sections,
                is_64: elf.is_64,
            })
        }
        Object::PE(pe) => {
            let sections = pe
                .sections
                .iter()
                .map(|s| SectionInfo {
                    name: String::from_utf8_lossy(&s.name)
                        .trim_end_matches('\0')
                        .to_string(),
                    address: s.virtual_address as u64,
                    size: s.virtual_size as u64,
                    file_offset: s.pointer_to_raw_data as u64,
                })
                .collect();
            Ok(BinaryInfo {
                format: "PE".to_string(),
                architecture: format!("{:?}", pe.header.coff_header.machine),
                entry_point: pe.entry as u64,
                sections,
                is_64: pe.is_64,
            })
        }
        Object::Mach(_) => Ok(BinaryInfo {
            format: "Mach-O".to_string(),
            architecture: "unknown".to_string(),
            entry_point: 0,
            sections: vec![],
            is_64: true,
        }),
        _ => anyhow::bail!("Unsupported or unrecognized file format"),
    }
}
