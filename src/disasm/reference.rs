pub struct InstructionInfo {
    pub mnemonic: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub flags_affected: &'static str,
}

pub fn lookup(mnemonic: &str) -> Option<InstructionInfo> {
    let normalized = mnemonic.to_lowercase();
    INSTRUCTION_TABLE
        .iter()
        .find(|i| i.mnemonic == normalized.as_str())
        .map(|i| InstructionInfo {
            mnemonic: i.mnemonic,
            summary: i.summary,
            description: i.description,
            flags_affected: i.flags_affected,
        })
}

pub fn all_mnemonics() -> Vec<&'static str> {
    INSTRUCTION_TABLE.iter().map(|i| i.mnemonic).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MnemonicCategory {
    ControlFlow,
    Stack,
    Arithmetic,
    Logic,
    DataMove,
    System,
    Other,
}

pub fn categorize(mnemonic: &str) -> MnemonicCategory {
    let m = mnemonic.to_lowercase();
    let base = m.trim_start_matches("lock ").trim_start_matches("rep ").trim_start_matches("repe ").trim_start_matches("repne ");
    match base {
        "jmp" | "je" | "jne" | "jg" | "jl" | "jge" | "jle" | "ja" | "jb" | "jae" | "jbe"
        | "jz" | "jnz" | "js" | "jns" | "jo" | "jno" | "jp" | "jnp" | "jcxz" | "jecxz"
        | "jrcxz" | "call" | "ret" | "retf" | "loop" | "loope" | "loopne" => {
            MnemonicCategory::ControlFlow
        }
        "push" | "pop" | "pushfq" | "popfq" | "pushf" | "popf" | "enter" | "leave" => {
            MnemonicCategory::Stack
        }
        "add" | "sub" | "imul" | "idiv" | "mul" | "div" | "inc" | "dec" | "neg" | "adc"
        | "sbb" | "cmp" => MnemonicCategory::Arithmetic,
        "and" | "or" | "xor" | "not" | "shl" | "shr" | "sar" | "rol" | "ror" | "test"
        | "bt" | "bts" | "btr" | "btc" => MnemonicCategory::Logic,
        "mov" | "movzx" | "movsx" | "lea" | "xchg" | "cmpxchg" | "movs" | "stos" | "lods"
        | "cdq" | "cwd" | "cbw" | "cdqe" => MnemonicCategory::DataMove,
        "int" | "syscall" | "sysenter" | "sysexit" | "hlt" | "cli" | "sti" | "in" | "out"
        | "lgdt" | "lidt" | "rdmsr" | "wrmsr" | "cpuid" | "nop" | "wait" | "lock" => {
            MnemonicCategory::System
        }
        _ => MnemonicCategory::Other,
    }
}

struct RawInstructionInfo {
    mnemonic: &'static str,
    summary: &'static str,
    description: &'static str,
    flags_affected: &'static str,
}

static INSTRUCTION_TABLE: &[RawInstructionInfo] = &[
    RawInstructionInfo {
        mnemonic: "mov",
        summary: "Move data",
        description: "Copies the value of the source operand into the destination operand. Does not affect flags. Works between registers, memory and immediates, but cannot copy memory to memory directly.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "lea",
        summary: "Load effective address",
        description: "Computes the address of a memory operand (base + index*scale + displacement) and writes the address itself into the destination register, without accessing memory. Frequently used for side-effect-free arithmetic.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "push",
        summary: "Push a value onto the stack",
        description: "Decrements the stack pointer (RSP/ESP/SP) by the operand size and writes the operand's value to the new top of the stack.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "pop",
        summary: "Pop a value off the stack",
        description: "Reads the value at the top of the stack into the destination operand and increments the stack pointer by the operand size.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "add",
        summary: "Addition",
        description: "Adds the source operand to the destination operand and stores the result in the destination. Sets flags based on the result.",
        flags_affected: "OF, SF, ZF, AF, CF, PF",
    },
    RawInstructionInfo {
        mnemonic: "sub",
        summary: "Subtraction",
        description: "Subtracts the source operand from the destination operand and stores the result in the destination. Sets flags based on the result.",
        flags_affected: "OF, SF, ZF, AF, CF, PF",
    },
    RawInstructionInfo {
        mnemonic: "imul",
        summary: "Signed multiplication",
        description: "Performs a signed multiplication of the operands. Can take one, two or three operands depending on the instruction form.",
        flags_affected: "OF, CF",
    },
    RawInstructionInfo {
        mnemonic: "idiv",
        summary: "Signed division",
        description: "Divides the value in the accumulator (AX/EAX/RAX with the high part in DX/EDX/RDX) by a signed operand; quotient and remainder are stored in the corresponding registers.",
        flags_affected: "undefined",
    },
    RawInstructionInfo {
        mnemonic: "cmp",
        summary: "Compare",
        description: "Computes destination minus source without storing the result, but sets flags as SUB would. Typically used before conditional jumps.",
        flags_affected: "OF, SF, ZF, AF, CF, PF",
    },
    RawInstructionInfo {
        mnemonic: "test",
        summary: "Bitwise AND for testing",
        description: "Performs a bitwise AND between the operands without storing the result, but sets the ZF and SF flags. Commonly used to test for zero or sign.",
        flags_affected: "SF, ZF, PF; OF=0, CF=0",
    },
    RawInstructionInfo {
        mnemonic: "jmp",
        summary: "Unconditional jump",
        description: "Transfers control to the specified address without checking any condition.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "je",
        summary: "Jump if equal",
        description: "The jump is taken if the ZF flag is set (typically after a CMP of equal values).",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "jne",
        summary: "Jump if not equal",
        description: "The jump is taken if the ZF flag is clear.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "jg",
        summary: "Jump if greater (signed)",
        description: "The jump is taken if ZF=0 and SF=OF, corresponding to a signed \"greater than\" comparison result.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "jl",
        summary: "Jump if less (signed)",
        description: "The jump is taken if SF differs from OF, corresponding to a signed \"less than\" comparison result.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "call",
        summary: "Call a subroutine",
        description: "Pushes the address of the next instruction onto the stack and transfers control to the specified address.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "ret",
        summary: "Return from a subroutine",
        description: "Pops the return address off the top of the stack and transfers control to it.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "nop",
        summary: "No operation",
        description: "Performs no action other than advancing the instruction pointer. Used for code alignment or padding.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "int",
        summary: "Software interrupt",
        description: "Triggers a software interrupt with the specified vector number, transferring control to the corresponding handler in the interrupt descriptor table.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "syscall",
        summary: "System call (x86-64)",
        description: "Switches the processor to kernel mode and transfers control to the system call handler whose address is stored in an MSR. The syscall number is passed in RAX.",
        flags_affected: "RF, VM, implementation-defined",
    },
    RawInstructionInfo {
        mnemonic: "hlt",
        summary: "Halt the processor",
        description: "Stops instruction execution until an interrupt occurs. Commonly used in OS kernels for power-efficient waiting.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "xor",
        summary: "Exclusive OR",
        description: "Performs a bitwise exclusive OR between the operands. Often used to zero a register (xor eax, eax) faster than MOV.",
        flags_affected: "SF, ZF, PF; OF=0, CF=0",
    },
    RawInstructionInfo {
        mnemonic: "and",
        summary: "Bitwise AND",
        description: "Performs a bitwise AND between the destination and source, storing the result in the destination.",
        flags_affected: "SF, ZF, PF; OF=0, CF=0",
    },
    RawInstructionInfo {
        mnemonic: "or",
        summary: "Bitwise OR",
        description: "Performs a bitwise OR between the destination and source, storing the result in the destination.",
        flags_affected: "SF, ZF, PF; OF=0, CF=0",
    },
    RawInstructionInfo {
        mnemonic: "not",
        summary: "Bitwise NOT",
        description: "Inverts every bit of the operand.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "shl",
        summary: "Logical shift left",
        description: "Shifts the operand's bits left by the given count, filling the low-order bits with zeros. Equivalent to multiplying by a power of two.",
        flags_affected: "CF, OF, SF, ZF, PF",
    },
    RawInstructionInfo {
        mnemonic: "shr",
        summary: "Logical shift right",
        description: "Shifts the operand's bits right by the given count, filling the high-order bits with zeros.",
        flags_affected: "CF, OF, SF, ZF, PF",
    },
    RawInstructionInfo {
        mnemonic: "sar",
        summary: "Arithmetic shift right",
        description: "Shifts the operand's bits right while preserving the sign bit, corresponding to division by a power of two for signed numbers.",
        flags_affected: "CF, OF, SF, ZF, PF",
    },
    RawInstructionInfo {
        mnemonic: "inc",
        summary: "Increment",
        description: "Increases the operand's value by one. Unlike ADD, it does not affect the CF flag.",
        flags_affected: "OF, SF, ZF, AF, PF",
    },
    RawInstructionInfo {
        mnemonic: "dec",
        summary: "Decrement",
        description: "Decreases the operand's value by one. Unlike SUB, it does not affect the CF flag.",
        flags_affected: "OF, SF, ZF, AF, PF",
    },
    RawInstructionInfo {
        mnemonic: "leave",
        summary: "Release the stack frame",
        description: "Equivalent to MOV RSP, RBP; POP RBP. Used in function epilogues to restore the caller's stack.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "cdq",
        summary: "Sign-extend EAX into EDX:EAX",
        description: "Extends the sign bit of EAX across the entire EDX register, preparing the EDX:EAX pair for the signed IDIV instruction.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "movzx",
        summary: "Move with zero-extension",
        description: "Copies a smaller operand into a larger register, filling the high-order bits with zeros. Used when working with unsigned values.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "movsx",
        summary: "Move with sign-extension",
        description: "Copies a smaller operand into a larger register, extending the sign bit into the high-order bits.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "cmpxchg",
        summary: "Atomic compare and exchange",
        description: "Compares the value in the accumulator with the destination; if equal, writes the source value into the destination, otherwise loads the destination's value into the accumulator. The basis of many lock-free algorithms.",
        flags_affected: "ZF and others as with CMP",
    },
    RawInstructionInfo {
        mnemonic: "lock",
        summary: "Bus lock prefix",
        description: "A prefix that guarantees exclusive access to memory while the following read-modify-write instruction executes, used for atomic operations in multithreaded code.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "cpuid",
        summary: "CPU identification",
        description: "Returns information about the processor and its supported features in EAX, EBX, ECX and EDX, depending on the input value in EAX.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "rdmsr",
        summary: "Read an MSR register",
        description: "Reads the value from the model-specific register indexed by ECX into EDX:EAX. Requires privileged mode.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "wrmsr",
        summary: "Write an MSR register",
        description: "Writes the value in EDX:EAX to the model-specific register indexed by ECX. Requires privileged mode.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "in",
        summary: "Read from an I/O port",
        description: "Reads a byte, word or double word from the specified I/O port into the accumulator.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "out",
        summary: "Write to an I/O port",
        description: "Writes the accumulator's value to the specified I/O port.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "lgdt",
        summary: "Load the GDTR register",
        description: "Loads the base address and size of the Global Descriptor Table (GDT) from the operand into the GDTR register. A key instruction when switching into protected mode.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "lidt",
        summary: "Load the IDTR register",
        description: "Loads the base address and size of the Interrupt Descriptor Table (IDT) into the IDTR register.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "sti",
        summary: "Enable interrupts",
        description: "Sets the IF flag in the flags register, enabling maskable hardware interrupts.",
        flags_affected: "IF",
    },
    RawInstructionInfo {
        mnemonic: "cli",
        summary: "Disable interrupts",
        description: "Clears the IF flag in the flags register, disabling maskable hardware interrupts.",
        flags_affected: "IF",
    },
    RawInstructionInfo {
        mnemonic: "pushfq",
        summary: "Push the flags register onto the stack",
        description: "Pushes the contents of the RFLAGS register onto the stack.",
        flags_affected: "none",
    },
    RawInstructionInfo {
        mnemonic: "popfq",
        summary: "Restore the flags register from the stack",
        description: "Pops a value off the stack and loads it into the RFLAGS register.",
        flags_affected: "all",
    },
];
