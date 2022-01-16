use std::fmt::Display;

use super::{
    nice::OpcodeBitfield, OPCODE_KIND_ARM64_DWARF, OPCODE_KIND_ARM64_FRAMEBASED,
    OPCODE_KIND_ARM64_FRAMELESS, OPCODE_KIND_NULL, OPCODE_KIND_X86_DWARF,
    OPCODE_KIND_X86_FRAMEBASED, OPCODE_KIND_X86_FRAMELESS_IMMEDIATE,
    OPCODE_KIND_X86_FRAMELESS_INDIRECT,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpcodeRegX86 {
    Ebx,
    Ecx,
    Edx,
    Edi,
    Esi,
    Ebp,
}

impl OpcodeRegX86 {
    pub fn parse(n: u32) -> Option<Self> {
        match n {
            1 => Some(OpcodeRegX86::Ebx),
            2 => Some(OpcodeRegX86::Ecx),
            3 => Some(OpcodeRegX86::Edx),
            4 => Some(OpcodeRegX86::Edi),
            5 => Some(OpcodeRegX86::Esi),
            6 => Some(OpcodeRegX86::Ebp),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpcodeRegX86_64 {
    Rbx,
    R12,
    R13,
    R14,
    R15,
    Rbp,
}

impl OpcodeRegX86_64 {
    pub fn parse(n: u32) -> Option<Self> {
        match n {
            1 => Some(OpcodeRegX86_64::Rbx),
            2 => Some(OpcodeRegX86_64::R12),
            3 => Some(OpcodeRegX86_64::R13),
            4 => Some(OpcodeRegX86_64::R14),
            5 => Some(OpcodeRegX86_64::R15),
            6 => Some(OpcodeRegX86_64::Rbp),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpcodeX86 {
    Null,
    FrameBased {
        stack_offset_in_bytes: u16,
        saved_regs: [Option<OpcodeRegX86>; 5],
    },
    FramelessImmediate {
        stack_size_in_bytes: u16,
        saved_regs: [Option<OpcodeRegX86>; 6],
    },
    FramelessIndirect,
    Dwarf {
        eh_frame_fde: u32,
    },
}

impl OpcodeX86 {
    pub fn parse(opcode: &OpcodeBitfield) -> Option<Self> {
        let opcode = match opcode.kind() {
            OPCODE_KIND_NULL => OpcodeX86::Null,
            OPCODE_KIND_X86_FRAMEBASED => OpcodeX86::FrameBased {
                stack_offset_in_bytes: (((opcode.0 >> 16) & 0xff) as u16) * 4,
                saved_regs: [
                    OpcodeRegX86::parse((opcode.0 >> 12) & 0b111),
                    OpcodeRegX86::parse((opcode.0 >> 9) & 0b111),
                    OpcodeRegX86::parse((opcode.0 >> 6) & 0b111),
                    OpcodeRegX86::parse((opcode.0 >> 3) & 0b111),
                    OpcodeRegX86::parse(opcode.0 & 0b111),
                ],
            },
            OPCODE_KIND_X86_FRAMELESS_IMMEDIATE => OpcodeX86::FramelessImmediate {
                stack_size_in_bytes: (((opcode.0 >> 16) & 0xff) as u16) * 4,
                saved_regs: [None, None, None, None, None, None],
            },
            OPCODE_KIND_X86_FRAMELESS_INDIRECT => OpcodeX86::FramelessIndirect,
            OPCODE_KIND_X86_DWARF => OpcodeX86::Dwarf {
                eh_frame_fde: (opcode.0 & 0xffffff),
            },
            _ => return None,
        };
        Some(opcode)
    }
}

impl Display for OpcodeX86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpcodeX86::Null => {
                write!(f, "(uncovered)")?;
            }
            OpcodeX86::FrameBased { .. } => {
                write!(f, "frame-based")?;
            }
            OpcodeX86::FramelessImmediate { .. } => {
                write!(f, "frameless immediate")?;
            }
            OpcodeX86::FramelessIndirect { .. } => {
                write!(f, "frameless indirect")?;
            }
            OpcodeX86::Dwarf { eh_frame_fde } => {
                write!(f, "(check eh_frame FDE 0x{:x})", eh_frame_fde)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpcodeX86_64 {
    Null,
    FrameBased {
        stack_offset_in_bytes: u16,
        saved_regs: [Option<OpcodeRegX86_64>; 5],
    },
    FramelessImmediate {
        stack_size_in_bytes: u16,
        saved_regs: [Option<OpcodeRegX86_64>; 6],
    },
    FramelessIndirect,
    Dwarf {
        eh_frame_fde: u32,
    },
}

impl OpcodeX86_64 {
    pub fn parse(opcode: &OpcodeBitfield) -> Option<Self> {
        let opcode = match opcode.kind() {
            OPCODE_KIND_NULL => OpcodeX86_64::Null,
            OPCODE_KIND_X86_FRAMEBASED => OpcodeX86_64::FrameBased {
                stack_offset_in_bytes: (((opcode.0 >> 16) & 0xff) as u16) * 8,
                saved_regs: [
                    OpcodeRegX86_64::parse((opcode.0 >> 12) & 0b111),
                    OpcodeRegX86_64::parse((opcode.0 >> 9) & 0b111),
                    OpcodeRegX86_64::parse((opcode.0 >> 6) & 0b111),
                    OpcodeRegX86_64::parse((opcode.0 >> 3) & 0b111),
                    OpcodeRegX86_64::parse(opcode.0 & 0b111),
                ],
            },
            OPCODE_KIND_X86_FRAMELESS_IMMEDIATE => OpcodeX86_64::FramelessImmediate {
                stack_size_in_bytes: (((opcode.0 >> 16) & 0xff) as u16) * 8,
                saved_regs: [None, None, None, None, None, None],
            },
            OPCODE_KIND_X86_FRAMELESS_INDIRECT => OpcodeX86_64::FramelessIndirect,
            OPCODE_KIND_X86_DWARF => OpcodeX86_64::Dwarf {
                eh_frame_fde: (opcode.0 & 0xffffff),
            },
            _ => return None,
        };
        Some(opcode)
    }
}

impl Display for OpcodeX86_64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpcodeX86_64::Null => {
                write!(f, "(uncovered)")?;
            }
            OpcodeX86_64::FrameBased { .. } => {
                write!(f, "frame-based")?;
            }
            OpcodeX86_64::FramelessImmediate { .. } => {
                write!(f, "frameless immediate")?;
            }
            OpcodeX86_64::FramelessIndirect { .. } => {
                write!(f, "frameless indirect")?;
            }
            OpcodeX86_64::Dwarf { eh_frame_fde } => {
                write!(f, "(check eh_frame FDE 0x{:x})", eh_frame_fde)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpcodeArm64 {
    Null,
    Frameless {
        stack_size_in_bytes: u16,
    },
    Dwarf {
        eh_frame_fde: u32,
    },
    FrameBased {
        // Whether each register pair was pushed
        d14_and_d15_saved: bool,
        d12_and_d13_saved: bool,
        d10_and_d11_saved: bool,
        d8_and_d9_saved: bool,

        x27_and_x28_saved: bool,
        x25_and_x26_saved: bool,
        x23_and_x24_saved: bool,
        x21_and_x22_saved: bool,
        x19_and_x20_saved: bool,
    },
}

impl OpcodeArm64 {
    pub fn parse(opcode: &OpcodeBitfield) -> Option<Self> {
        let opcode = match opcode.kind() {
            OPCODE_KIND_NULL => OpcodeArm64::Null,
            OPCODE_KIND_ARM64_FRAMELESS => OpcodeArm64::Frameless {
                stack_size_in_bytes: (((opcode.0 >> 12) & 0b1111_1111_1111) as u16) * 16,
            },
            OPCODE_KIND_ARM64_DWARF => OpcodeArm64::Dwarf {
                eh_frame_fde: (opcode.0 & 0xffffff),
            },
            OPCODE_KIND_ARM64_FRAMEBASED => OpcodeArm64::FrameBased {
                d14_and_d15_saved: ((opcode.0 >> 8) & 1) == 1,
                d12_and_d13_saved: ((opcode.0 >> 7) & 1) == 1,
                d10_and_d11_saved: ((opcode.0 >> 6) & 1) == 1,
                d8_and_d9_saved: ((opcode.0 >> 5) & 1) == 1,
                x27_and_x28_saved: ((opcode.0 >> 4) & 1) == 1,
                x25_and_x26_saved: ((opcode.0 >> 3) & 1) == 1,
                x23_and_x24_saved: ((opcode.0 >> 2) & 1) == 1,
                x21_and_x22_saved: ((opcode.0 >> 1) & 1) == 1,
                x19_and_x20_saved: (opcode.0 & 1) == 1,
            },
            _ => return None,
        };
        Some(opcode)
    }
}

impl Display for OpcodeArm64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpcodeArm64::Null => {
                write!(f, "(uncovered)")?;
            }
            OpcodeArm64::Frameless {
                stack_size_in_bytes,
            } => {
                if *stack_size_in_bytes == 0 {
                    write!(f, "CFA=reg31")?;
                } else {
                    write!(f, "CFA=reg31+{}", stack_size_in_bytes)?;
                }
            }
            OpcodeArm64::Dwarf { eh_frame_fde } => {
                write!(f, "(check eh_frame FDE 0x{:x})", eh_frame_fde)?;
            }
            OpcodeArm64::FrameBased {
                d14_and_d15_saved,
                d12_and_d13_saved,
                d10_and_d11_saved,
                d8_and_d9_saved,
                x27_and_x28_saved,
                x25_and_x26_saved,
                x23_and_x24_saved,
                x21_and_x22_saved,
                x19_and_x20_saved,
            } => {
                write!(f, "CFA=reg29: reg31=reg29+16")?;
                let mut offset = 16;
                let mut next_pair = |pair_saved, a, b| {
                    if pair_saved {
                        let r = write!(f, " {}=[CFA+{}] {}=[CFA+{}]", a, offset, b, offset + 8);
                        offset += 16;
                        r
                    } else {
                        Ok(())
                    }
                };
                next_pair(*x19_and_x20_saved, "reg19", "reg20")?;
                next_pair(*x21_and_x22_saved, "reg21", "reg22")?;
                next_pair(*x23_and_x24_saved, "reg23", "reg24")?;
                next_pair(*x25_and_x26_saved, "reg25", "reg26")?;
                next_pair(*x27_and_x28_saved, "reg27", "reg28")?;
                next_pair(*d8_and_d9_saved, "reg8", "reg9")?;
                next_pair(*d10_and_d11_saved, "reg10", "reg11")?;
                next_pair(*d12_and_d13_saved, "reg12", "reg13")?;
                next_pair(*d14_and_d15_saved, "reg14", "reg15")?;
                write!(f, " reg30=[CFA+8] reg29=[CFA]")?;
            }
        }
        Ok(())
    }
}