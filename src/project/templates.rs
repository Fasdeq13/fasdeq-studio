use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::icons;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OsBitness {
    Bits16,
    Bits32,
}

impl OsBitness {
    pub fn label(&self) -> &'static str {
        match self {
            OsBitness::Bits16 => "16-bit (real mode)",
            OsBitness::Bits32 => "32-bit (protected mode)",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectTemplate {
    OsCAsm(OsBitness),
    OsCpp(OsBitness),
    OsRust,
    EmptyC,
    EmptyCpp,
    EmptyRust,
    EmptyAsm,
}

impl ProjectTemplate {
    pub fn all() -> Vec<ProjectTemplate> {
        vec![
            ProjectTemplate::OsCAsm(OsBitness::Bits32),
            ProjectTemplate::OsCAsm(OsBitness::Bits16),
            ProjectTemplate::OsCpp(OsBitness::Bits32),
            ProjectTemplate::OsCpp(OsBitness::Bits16),
            ProjectTemplate::OsRust,
            ProjectTemplate::EmptyC,
            ProjectTemplate::EmptyCpp,
            ProjectTemplate::EmptyRust,
            ProjectTemplate::EmptyAsm,
        ]
    }

    pub fn title(&self) -> String {
        match self {
            ProjectTemplate::OsCAsm(bits) => {
                format!("Bare-metal OS starter in C & Assembler ({})", bits.label())
            }
            ProjectTemplate::OsCpp(bits) => {
                format!("Bare-metal OS starter in C++ ({})", bits.label())
            }
            ProjectTemplate::OsRust => "Bare-metal OS starter in Rust".to_string(),
            ProjectTemplate::EmptyC => "Empty C project".to_string(),
            ProjectTemplate::EmptyCpp => "Empty C++ project".to_string(),
            ProjectTemplate::EmptyRust => "Empty Rust project".to_string(),
            ProjectTemplate::EmptyAsm => "Empty Assembler project".to_string(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ProjectTemplate::OsCAsm(OsBitness::Bits32) => {
                "An assembly bootloader, a jump into 32-bit protected mode, a C kernel, and basic VGA output"
            }
            ProjectTemplate::OsCAsm(OsBitness::Bits16) => {
                "An assembly bootloader and a C kernel that stay entirely in 16-bit real mode, using BIOS interrupts for output"
            }
            ProjectTemplate::OsCpp(OsBitness::Bits32) => {
                "A C++ kernel skeleton in 32-bit protected mode with basic device classes and a linker script"
            }
            ProjectTemplate::OsCpp(OsBitness::Bits16) => {
                "A C++-flavored kernel skeleton that runs in 16-bit real mode via BIOS calls"
            }
            ProjectTemplate::OsRust => {
                "A no_std x86_64 kernel in Rust with a basic VGA buffer driver"
            }
            ProjectTemplate::EmptyC => "A minimal plain C project with a Makefile",
            ProjectTemplate::EmptyCpp => "A minimal plain C++ project with a Makefile",
            ProjectTemplate::EmptyRust => "A standard binary Rust project managed by cargo",
            ProjectTemplate::EmptyAsm => "A minimal NASM project linked with ld",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ProjectTemplate::OsCAsm(_) => icons::CUBE,
            ProjectTemplate::OsCpp(_) => icons::GEAR_SIX,
            ProjectTemplate::OsRust => icons::FILE_RS,
            ProjectTemplate::EmptyC => icons::FILE_C,
            ProjectTemplate::EmptyCpp => icons::FILE_CPP,
            ProjectTemplate::EmptyRust => icons::CUBE,
            ProjectTemplate::EmptyAsm => icons::FILE_CODE,
        }
    }

    pub fn files(&self, project_name: &str) -> Vec<(PathBuf, String)> {
        match self {
            ProjectTemplate::OsCAsm(OsBitness::Bits32) => os_c_asm_32_files(project_name),
            ProjectTemplate::OsCAsm(OsBitness::Bits16) => os_c_asm_16_files(project_name),
            ProjectTemplate::OsCpp(OsBitness::Bits32) => os_cpp_32_files(project_name),
            ProjectTemplate::OsCpp(OsBitness::Bits16) => os_cpp_16_files(project_name),
            ProjectTemplate::OsRust => os_rust_files(project_name),
            ProjectTemplate::EmptyC => empty_c_files(project_name),
            ProjectTemplate::EmptyCpp => empty_cpp_files(project_name),
            ProjectTemplate::EmptyRust => empty_rust_files(project_name),
            ProjectTemplate::EmptyAsm => empty_asm_files(project_name),
        }
    }

    pub fn create_at(&self, root: &Path, project_name: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(root)?;
        for (relative, content) in self.files(project_name) {
            let full = root.join(relative);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(full, content)?;
        }
        Ok(())
    }
}

fn os_c_asm_32_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("boot/boot.asm"),
            r#"[org 0x7c00]
[bits 16]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00
    sti

    mov si, msg_boot
    call print16

    call enable_a20
    call load_kernel
    call enter_protected_mode

hang:
    jmp hang

print16:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp print16
.done:
    ret

enable_a20:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

load_kernel:
    mov ah, 0x02
    mov al, 64
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov bx, 0x1000
    mov es, bx
    xor bx, bx
    int 0x13
    ret

gdt_start:
    dq 0
gdt_code:
    dw 0xffff
    dw 0
    db 0
    db 10011010b
    db 11001111b
    db 0
gdt_data:
    dw 0xffff
    dw 0
    db 0
    db 10010010b
    db 11001111b
    db 0
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

enter_protected_mode:
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp CODE_SEG:init_pm

[bits 32]
init_pm:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov ebp, 0x90000
    mov esp, ebp
    jmp 0x10000

msg_boot: db "Fasdeq OS bootloader starting...", 0

times 510-($-$$) db 0
dw 0xaa55
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/entry.asm"),
            r#"[bits 32]
[extern kernel_main]
global _start

_start:
    call kernel_main
.hang:
    hlt
    jmp .hang
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/kernel.c"),
            r#"#include <stdint.h>
#include <stddef.h>

static uint16_t *const VGA_MEMORY = (uint16_t *)0xb8000;
static const int VGA_WIDTH = 80;
static const int VGA_HEIGHT = 25;

static int cursor_row = 0;
static int cursor_col = 0;

static uint16_t make_entry(char c, uint8_t color) {
    return (uint16_t)c | ((uint16_t)color << 8);
}

void vga_clear(void) {
    for (int y = 0; y < VGA_HEIGHT; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[y * VGA_WIDTH + x] = make_entry(' ', 0x0f);
        }
    }
    cursor_row = 0;
    cursor_col = 0;
}

void vga_put_char(char c) {
    if (c == '\n') {
        cursor_col = 0;
        cursor_row++;
        return;
    }
    VGA_MEMORY[cursor_row * VGA_WIDTH + cursor_col] = make_entry(c, 0x0f);
    cursor_col++;
    if (cursor_col >= VGA_WIDTH) {
        cursor_col = 0;
        cursor_row++;
    }
}

void vga_print(const char *str) {
    for (size_t i = 0; str[i] != '\0'; i++) {
        vga_put_char(str[i]);
    }
}

void kernel_main(void) {
    vga_clear();
    vga_print("Fasdeq OS kernel loaded successfully.\n");
    vga_print("C kernel is running in 32-bit protected mode.\n");
    for (;;) {
        __asm__ volatile("hlt");
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/linker.ld"),
            r#"ENTRY(_start)

SECTIONS
{
    . = 0x10000;

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(COMMON)
        *(.bss)
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"PROJECT = {name}
BUILD = build

all: $(BUILD)/os-image.bin

$(BUILD)/boot.bin: boot/boot.asm
	mkdir -p $(BUILD)
	nasm -f bin boot/boot.asm -o $(BUILD)/boot.bin

$(BUILD)/entry.o: kernel/entry.asm
	nasm -f elf32 kernel/entry.asm -o $(BUILD)/entry.o

$(BUILD)/kernel.o: kernel/kernel.c
	gcc -m32 -ffreestanding -fno-pie -c kernel/kernel.c -o $(BUILD)/kernel.o

$(BUILD)/kernel.bin: $(BUILD)/entry.o $(BUILD)/kernel.o kernel/linker.ld
	ld -m elf_i386 -T kernel/linker.ld -o $(BUILD)/kernel.bin $(BUILD)/entry.o $(BUILD)/kernel.o --oformat binary

$(BUILD)/os-image.bin: $(BUILD)/boot.bin $(BUILD)/kernel.bin
	cat $(BUILD)/boot.bin $(BUILD)/kernel.bin > $(BUILD)/os-image.bin

run: all
	qemu-system-x86_64 -drive format=raw,file=$(BUILD)/os-image.bin

clean:
	rm -rf $(BUILD)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!(
                r#"[project]
name = "{name}"
kind = "os-c-asm"
target = "i386"

[build]
tool = "make"
"#
            ),
        ),
        (
            PathBuf::from("README.md"),
            format!("# {name}\n\nA bare-metal operating system starter written in C and Assembler, created with Fasdeq Studio.\n\n## Build\n\n```\nmake run\n```\n"),
        ),
    ]
}

fn os_cpp_32_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("boot/boot.asm"),
            r#"[org 0x7c00]
[bits 16]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00
    sti
    mov si, msg
    call print16
    jmp $

print16:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp print16
.done:
    ret

msg: db "Fasdeq OS (C++) bootloader...", 0

times 510-($-$$) db 0
dw 0xaa55
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/entry.asm"),
            r#"[bits 32]
[extern kernel_main]
global _start

_start:
    call kernel_main
.hang:
    hlt
    jmp .hang
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/Vga.hpp"),
            r#"#pragma once
#include <cstdint>
#include <cstddef>

class Vga {
public:
    static void clear();
    static void print(const char *text);

private:
    static uint16_t *memory;
    static int row;
    static int col;
    static void putChar(char c);
};
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/Vga.cpp"),
            r#"#include "Vga.hpp"

uint16_t *Vga::memory = reinterpret_cast<uint16_t *>(0xb8000);
int Vga::row = 0;
int Vga::col = 0;

void Vga::putChar(char c) {
    if (c == '\n') {
        col = 0;
        row++;
        return;
    }
    memory[row * 80 + col] = static_cast<uint16_t>(c) | (0x0f << 8);
    col++;
    if (col >= 80) {
        col = 0;
        row++;
    }
}

void Vga::clear() {
    for (int y = 0; y < 25; y++) {
        for (int x = 0; x < 80; x++) {
            memory[y * 80 + x] = static_cast<uint16_t>(' ') | (0x0f << 8);
        }
    }
    row = 0;
    col = 0;
}

void Vga::print(const char *text) {
    for (size_t i = 0; text[i] != '\0'; i++) {
        putChar(text[i]);
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/kernel.cpp"),
            r#"#include "Vga.hpp"

extern "C" void kernel_main() {
    Vga::clear();
    Vga::print("Fasdeq OS C++ kernel is running.\n");
    for (;;) {
        asm volatile("hlt");
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/linker.ld"),
            r#"ENTRY(_start)

SECTIONS
{
    . = 0x10000;

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(COMMON)
        *(.bss)
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"PROJECT = {name}
BUILD = build
CXXFLAGS = -m32 -ffreestanding -fno-exceptions -fno-rtti -fno-pie

all: $(BUILD)/os-image.bin

$(BUILD)/boot.bin: boot/boot.asm
	mkdir -p $(BUILD)
	nasm -f bin boot/boot.asm -o $(BUILD)/boot.bin

$(BUILD)/entry.o: kernel/entry.asm
	nasm -f elf32 kernel/entry.asm -o $(BUILD)/entry.o

$(BUILD)/vga.o: kernel/Vga.cpp
	g++ $(CXXFLAGS) -c kernel/Vga.cpp -o $(BUILD)/vga.o

$(BUILD)/kernel.o: kernel/kernel.cpp
	g++ $(CXXFLAGS) -c kernel/kernel.cpp -o $(BUILD)/kernel.o

$(BUILD)/kernel.bin: $(BUILD)/entry.o $(BUILD)/vga.o $(BUILD)/kernel.o kernel/linker.ld
	ld -m elf_i386 -T kernel/linker.ld -o $(BUILD)/kernel.bin $(BUILD)/entry.o $(BUILD)/vga.o $(BUILD)/kernel.o --oformat binary

$(BUILD)/os-image.bin: $(BUILD)/boot.bin $(BUILD)/kernel.bin
	cat $(BUILD)/boot.bin $(BUILD)/kernel.bin > $(BUILD)/os-image.bin

run: all
	qemu-system-x86_64 -drive format=raw,file=$(BUILD)/os-image.bin

clean:
	rm -rf $(BUILD)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!(
                r#"[project]
name = "{name}"
kind = "os-cpp"
target = "i386"

[build]
tool = "make"
"#
            ),
        ),
        (
            PathBuf::from("README.md"),
            format!("# {name}\n\nA bare-metal operating system starter written in C++, created with Fasdeq Studio.\n\n## Build\n\n```\nmake run\n```\n"),
        ),
    ]
}

fn os_rust_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("src/main.rs"),
            r#"#![no_std]
#![no_main]

use core::panic::PanicInfo;

static VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

fn print(text: &str) {
    for (i, byte) in text.bytes().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0x0f;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Fasdeq OS Rust kernel is running.");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Cargo.toml"),
            format!(
                r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
"#
            ),
        ),
        (
            PathBuf::from("x86_64-fasdeq_os.json"),
            r#"{
    "llvm-target": "x86_64-unknown-none",
    "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "os": "none",
    "executable": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float"
}
"#
            .to_string(),
        ),
        (
            PathBuf::from(".cargo/config.toml"),
            r#"[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]

[build]
target = "x86_64-fasdeq_os.json"
"#
            .to_string(),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!(
                r#"[project]
name = "{name}"
kind = "os-rust"
target = "x86_64-fasdeq_os"

[build]
tool = "cargo"
"#
            ),
        ),
        (
            PathBuf::from("README.md"),
            format!("# {name}\n\nA bare-metal (no_std) operating system starter written in Rust, created with Fasdeq Studio.\n\n## Build\n\nRequires a nightly toolchain with the rust-src and llvm-tools-preview components.\n"),
        ),
    ]
}

fn os_c_asm_16_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("boot/boot.asm"),
            r#"[org 0x7c00]
[bits 16]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00
    sti

    mov si, msg_boot
    call print16

    call load_kernel

    jmp 0x0000:0x1000

hang:
    jmp hang

print16:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp print16
.done:
    ret

load_kernel:
    mov ah, 0x02
    mov al, 32
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov bx, 0x1000
    mov es, bx
    xor bx, bx
    int 0x13
    jc disk_error
    ret

disk_error:
    mov si, msg_disk_error
    call print16
    jmp hang

msg_boot: db "Fasdeq OS 16-bit bootloader starting...", 0
msg_disk_error: db "Disk read error!", 0

times 510-($-$$) db 0
dw 0xaa55
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/entry.asm"),
            r#"[bits 16]
[org 0x1000]
[extern kernel_main]
global _start

_start:
    mov ax, 0x1000
    mov ds, ax
    mov es, ax
    mov ax, 0
    mov ss, ax
    mov sp, 0x7c00
    call kernel_main
.hang:
    hlt
    jmp .hang
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/kernel.c"),
            r#"#include <stdint.h>
#include <stddef.h>

static void bios_putchar(char c) {
    __asm__ volatile(
        "mov $0x0e, %%ah\n\t"
        "int $0x10\n\t"
        :
        : "a"(c)
    );
}

static void bios_print(const char *str) {
    for (size_t i = 0; str[i] != '\0'; i++) {
        if (str[i] == '\n') {
            bios_putchar('\r');
        }
        bios_putchar(str[i]);
    }
}

void kernel_main(void) {
    bios_print("Fasdeq OS 16-bit kernel loaded via BIOS interrupts.\n");
    bios_print("Running entirely in real mode.\n");
    for (;;) {
        __asm__ volatile("hlt");
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"PROJECT = {name}
BUILD = build

all: $(BUILD)/os-image.bin

$(BUILD)/boot.bin: boot/boot.asm
	mkdir -p $(BUILD)
	nasm -f bin boot/boot.asm -o $(BUILD)/boot.bin

$(BUILD)/entry.o: kernel/entry.asm
	nasm -f elf kernel/entry.asm -o $(BUILD)/entry.o

$(BUILD)/kernel.o: kernel/kernel.c
	gcc -m16 -march=i386 -ffreestanding -fno-pie -c kernel/kernel.c -o $(BUILD)/kernel.o

$(BUILD)/kernel.bin: $(BUILD)/entry.o $(BUILD)/kernel.o
	ld -m elf_i386 -Ttext 0x1000 --oformat binary -o $(BUILD)/kernel.bin $(BUILD)/entry.o $(BUILD)/kernel.o

$(BUILD)/os-image.bin: $(BUILD)/boot.bin $(BUILD)/kernel.bin
	cat $(BUILD)/boot.bin $(BUILD)/kernel.bin > $(BUILD)/os-image.bin

run: all
	qemu-system-x86_64 -drive format=raw,file=$(BUILD)/os-image.bin

clean:
	rm -rf $(BUILD)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!(
                r#"[project]
name = "{name}"
kind = "os-c-asm"
target = "i8086"
bitness = "16"

[build]
tool = "make"
"#
            ),
        ),
        (
            PathBuf::from("README.md"),
            format!("# {name}\n\nA 16-bit real mode operating system starter written in C and Assembler, created with Fasdeq Studio. All output goes through BIOS interrupts.\n\n## Build\n\n```\nmake run\n```\n"),
        ),
    ]
}

fn os_cpp_16_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("boot/boot.asm"),
            r#"[org 0x7c00]
[bits 16]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00
    sti
    mov si, msg
    call print16
    call load_kernel
    jmp 0x0000:0x1000

print16:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp print16
.done:
    ret

load_kernel:
    mov ah, 0x02
    mov al, 32
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov bx, 0x1000
    mov es, bx
    xor bx, bx
    int 0x13
    ret

msg: db "Fasdeq OS (C++) 16-bit bootloader...", 0

times 510-($-$$) db 0
dw 0xaa55
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/entry.asm"),
            r#"[bits 16]
[org 0x1000]
[extern kernel_main]
global _start

_start:
    mov ax, 0x1000
    mov ds, ax
    mov es, ax
    mov ax, 0
    mov ss, ax
    mov sp, 0x7c00
    call kernel_main
.hang:
    hlt
    jmp .hang
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/Bios.hpp"),
            r#"#pragma once
#include <cstddef>

class Bios {
public:
    static void print(const char *text);

private:
    static void putChar(char c);
};
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/Bios.cpp"),
            r#"#include "Bios.hpp"

void Bios::putChar(char c) {
    asm volatile(
        "mov $0x0e, %%ah\n\t"
        "int $0x10\n\t"
        :
        : "a"(c)
    );
}

void Bios::print(const char *text) {
    for (size_t i = 0; text[i] != '\0'; i++) {
        if (text[i] == '\n') {
            putChar('\r');
        }
        putChar(text[i]);
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("kernel/kernel.cpp"),
            r#"#include "Bios.hpp"

extern "C" void kernel_main() {
    Bios::print("Fasdeq OS C++ 16-bit kernel is running.\n");
    for (;;) {
        asm volatile("hlt");
    }
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"PROJECT = {name}
BUILD = build
CXXFLAGS = -m16 -march=i386 -ffreestanding -fno-exceptions -fno-rtti -fno-pie

all: $(BUILD)/os-image.bin

$(BUILD)/boot.bin: boot/boot.asm
	mkdir -p $(BUILD)
	nasm -f bin boot/boot.asm -o $(BUILD)/boot.bin

$(BUILD)/entry.o: kernel/entry.asm
	nasm -f elf kernel/entry.asm -o $(BUILD)/entry.o

$(BUILD)/bios.o: kernel/Bios.cpp
	g++ $(CXXFLAGS) -c kernel/Bios.cpp -o $(BUILD)/bios.o

$(BUILD)/kernel.o: kernel/kernel.cpp
	g++ $(CXXFLAGS) -c kernel/kernel.cpp -o $(BUILD)/kernel.o

$(BUILD)/kernel.bin: $(BUILD)/entry.o $(BUILD)/bios.o $(BUILD)/kernel.o
	ld -m elf_i386 -Ttext 0x1000 --oformat binary -o $(BUILD)/kernel.bin $(BUILD)/entry.o $(BUILD)/bios.o $(BUILD)/kernel.o

$(BUILD)/os-image.bin: $(BUILD)/boot.bin $(BUILD)/kernel.bin
	cat $(BUILD)/boot.bin $(BUILD)/kernel.bin > $(BUILD)/os-image.bin

run: all
	qemu-system-x86_64 -drive format=raw,file=$(BUILD)/os-image.bin

clean:
	rm -rf $(BUILD)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!(
                r#"[project]
name = "{name}"
kind = "os-cpp"
target = "i8086"
bitness = "16"

[build]
tool = "make"
"#
            ),
        ),
        (
            PathBuf::from("README.md"),
            format!("# {name}\n\nA 16-bit real mode operating system starter written in C++, created with Fasdeq Studio. All output goes through BIOS interrupts.\n\n## Build\n\n```\nmake run\n```\n"),
        ),
    ]
}

fn empty_c_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("src/main.c"),
            r#"#include <stdio.h>

int main(void) {
    printf("Hello from Fasdeq Studio!\n");
    return 0;
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"CC = gcc
CFLAGS = -Wall -Wextra -std=c17
TARGET = {name}

all: $(TARGET)

$(TARGET): src/main.c
	$(CC) $(CFLAGS) src/main.c -o $(TARGET)

run: all
	./$(TARGET)

clean:
	rm -f $(TARGET)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!("[project]\nname = \"{name}\"\nkind = \"c\"\n\n[build]\ntool = \"make\"\n"),
        ),
    ]
}

fn empty_cpp_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("src/main.cpp"),
            r#"#include <iostream>

int main() {
    std::cout << "Hello from Fasdeq Studio!" << std::endl;
    return 0;
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"CXX = g++
CXXFLAGS = -Wall -Wextra -std=c++20
TARGET = {name}

all: $(TARGET)

$(TARGET): src/main.cpp
	$(CXX) $(CXXFLAGS) src/main.cpp -o $(TARGET)

run: all
	./$(TARGET)

clean:
	rm -f $(TARGET)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!("[project]\nname = \"{name}\"\nkind = \"cpp\"\n\n[build]\ntool = \"make\"\n"),
        ),
    ]
}

fn empty_rust_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("src/main.rs"),
            r#"fn main() {
    println!("Hello from Fasdeq Studio!");
}
"#
            .to_string(),
        ),
        (
            PathBuf::from("Cargo.toml"),
            format!(
                r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!("[project]\nname = \"{name}\"\nkind = \"rust\"\n\n[build]\ntool = \"cargo\"\n"),
        ),
    ]
}

fn empty_asm_files(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (
            PathBuf::from("src/main.asm"),
            r#"section .data
    msg db "Hello from Fasdeq Studio!", 10
    msg_len equ $ - msg

section .text
    global _start

_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, msg_len
    syscall

    mov rax, 60
    xor rdi, rdi
    syscall
"#
            .to_string(),
        ),
        (
            PathBuf::from("Makefile"),
            format!(
                r#"TARGET = {name}

all: $(TARGET)

$(TARGET): src/main.asm
	nasm -f elf64 src/main.asm -o build.o
	ld build.o -o $(TARGET)
	rm -f build.o

run: all
	./$(TARGET)

clean:
	rm -f $(TARGET)

.PHONY: all run clean
"#
            ),
        ),
        (
            PathBuf::from("fasdeq.toml"),
            format!("[project]\nname = \"{name}\"\nkind = \"asm\"\n\n[build]\ntool = \"make\"\n"),
        ),
    ]
}
