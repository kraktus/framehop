use super::arch::ArchX86_64;
use super::framepointer::FramepointerUnwinderX86_64;
use super::unwind_rule::UnwindRuleX86_64;
use super::unwindregs::UnwindRegsX86_64;
use crate::macho::{CompactUnwindInfoUnwinderError, CompactUnwindInfoUnwinding, CuiUnwindResult};
use crate::unwind_result::UnwindResult;
use macho_unwind_info::opcodes::{OpcodeX86_64, RegisterNameX86_64};

impl CompactUnwindInfoUnwinding for ArchX86_64 {
    fn unwind_frame<F>(
        opcode: u32,
        regs: &mut UnwindRegsX86_64,
        read_mem: &mut F,
        is_first_frame: bool,
    ) -> Result<CuiUnwindResult<UnwindRuleX86_64>, CompactUnwindInfoUnwinderError>
    where
        F: FnMut(u64) -> Result<u64, ()>,
    {
        let opcode = OpcodeX86_64::parse(opcode);
        let r = match opcode {
            OpcodeX86_64::Null => {
                if is_first_frame {
                    CuiUnwindResult::ExecRule(UnwindRuleX86_64::JustReturn)
                } else {
                    return Err(CompactUnwindInfoUnwinderError::FunctionHasNoInfo);
                }
            }
            OpcodeX86_64::FramelessImmediate {
                stack_size_in_bytes,
                saved_regs,
            } => {
                if stack_size_in_bytes == 8 {
                    CuiUnwindResult::ExecRule(UnwindRuleX86_64::JustReturn)
                } else {
                    let bp_positon_from_outside = saved_regs
                        .iter()
                        .rev()
                        .flatten()
                        .position(|r| *r == RegisterNameX86_64::Rbp);
                    let bp_offset_from_sp = bp_positon_from_outside
                        .map(|pos| stack_size_in_bytes as i32 - 2 * 8 - pos as i32 * 8);
                    match bp_offset_from_sp.map(|offset| i8::try_from(offset / 8)) {
                        None => CuiUnwindResult::ExecRule(UnwindRuleX86_64::OffsetSp {
                            sp_offset_by_8: stack_size_in_bytes / 8,
                        }),
                        Some(Ok(bp_storage_offset_from_sp_by_8)) => {
                            CuiUnwindResult::ExecRule(UnwindRuleX86_64::OffsetSpAndRestoreBp {
                                sp_offset_by_8: stack_size_in_bytes / 8,
                                bp_storage_offset_from_sp_by_8,
                            })
                        }
                        Some(Err(_)) => {
                            eprintln!("Uncacheable rule in compact unwind info unwinder because Frameless stack size doesn't fit");
                            let sp = regs.sp();
                            let new_sp = sp + stack_size_in_bytes as u64;
                            let return_address = read_mem(new_sp - 8).map_err(|_| {
                                CompactUnwindInfoUnwinderError::CouldNotReadReturnAddress
                            })?;
                            if let Some(bp_offset_from_sp) = bp_offset_from_sp {
                                let new_bp =
                                    read_mem(sp.wrapping_add(bp_offset_from_sp as i64 as u64))
                                        .map_err(|_| {
                                            CompactUnwindInfoUnwinderError::CouldNotReadBp
                                        })?;
                                regs.set_bp(new_bp);
                            }
                            regs.set_sp(new_sp);
                            regs.set_ip(return_address);
                            CuiUnwindResult::Uncacheable(return_address)
                        }
                    }
                }
            }
            OpcodeX86_64::FramelessIndirect { .. } => {
                return Err(CompactUnwindInfoUnwinderError::CantHandleFramelessIndirect)
            }
            OpcodeX86_64::Dwarf { eh_frame_fde } => CuiUnwindResult::NeedDwarf(eh_frame_fde),
            OpcodeX86_64::FrameBased { .. } => {
                if is_first_frame {
                    // TODO: Detect if we're in an epilogue, by seeing if the current instruction restores
                    // registers from the stack (and then keep reading) or is a return instruction.
                    match FramepointerUnwinderX86_64.unwind_first() {
                        UnwindResult::ExecRule(rule) => CuiUnwindResult::ExecRule(rule),
                        UnwindResult::Uncacheable(return_address) => {
                            CuiUnwindResult::Uncacheable(return_address)
                        }
                    }
                } else {
                    CuiUnwindResult::ExecRule(UnwindRuleX86_64::UseFramePointer)
                }
            }
            OpcodeX86_64::UnrecognizedKind(kind) => {
                return Err(CompactUnwindInfoUnwinderError::BadOpcodeKind(kind))
            }
            OpcodeX86_64::InvalidFramelessImmediate => {
                return Err(CompactUnwindInfoUnwinderError::InvalidFramelessImmediate)
            }
        };
        Ok(r)
    }
}