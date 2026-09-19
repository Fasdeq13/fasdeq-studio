use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl NumberBase {
    pub fn radix(&self) -> u32 {
        match self {
            NumberBase::Binary => 2,
            NumberBase::Octal => 8,
            NumberBase::Decimal => 10,
            NumberBase::Hexadecimal => 16,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            NumberBase::Binary => "Binary",
            NumberBase::Octal => "Octal",
            NumberBase::Decimal => "Decimal",
            NumberBase::Hexadecimal => "Hexadecimal",
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            NumberBase::Binary => "0b",
            NumberBase::Octal => "0o",
            NumberBase::Decimal => "",
            NumberBase::Hexadecimal => "0x",
        }
    }
}

pub fn parse_value(input: &str, base: NumberBase) -> Option<u64> {
    let cleaned = input.trim();
    let cleaned = cleaned
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .trim_start_matches("0b")
        .trim_start_matches("0B")
        .trim_start_matches("0o")
        .trim_start_matches("0O");
    if cleaned.is_empty() {
        return None;
    }
    u64::from_str_radix(cleaned, base.radix()).ok()
}

pub fn format_value(value: u64, base: NumberBase, grouped: bool) -> String {
    let raw = match base {
        NumberBase::Binary => format!("{value:b}"),
        NumberBase::Octal => format!("{value:o}"),
        NumberBase::Decimal => format!("{value}"),
        NumberBase::Hexadecimal => format!("{value:x}"),
    };
    if !grouped {
        return raw;
    }
    let group_size = match base {
        NumberBase::Binary => 4,
        NumberBase::Hexadecimal => 4,
        NumberBase::Octal => 3,
        NumberBase::Decimal => 3,
    };
    group_digits(&raw, group_size)
}

fn group_digits(digits: &str, group_size: usize) -> String {
    let chars: Vec<char> = digits.chars().rev().collect();
    let mut result = Vec::new();
    for (i, ch) in chars.iter().enumerate() {
        if i != 0 && i % group_size == 0 {
            result.push(' ');
        }
        result.push(*ch);
    }
    result.iter().rev().collect()
}

pub fn to_binary_bytes(value: u64, byte_width: u8) -> Vec<u8> {
    let bytes = value.to_le_bytes();
    bytes[..(byte_width as usize).min(8)].to_vec()
}

pub fn signed_interpretation(value: u64, byte_width: u8) -> i64 {
    match byte_width {
        1 => (value as u8) as i8 as i64,
        2 => (value as u16) as i16 as i64,
        4 => (value as u32) as i32 as i64,
        _ => value as i64,
    }
}
