use crate::arch::Arch;

pub trait InstructionAnalysis: Arch {
    /// Caller guarantees pc_offset <= text_bytes.len()
    fn rule_from_prologue_analysis(text_bytes: &[u8], pc_offset: usize)
        -> Option<Self::UnwindRule>;

    /// Caller guarantees pc_offset <= text_bytes.len()
    fn rule_from_epilogue_analysis(text_bytes: &[u8], pc_offset: usize)
        -> Option<Self::UnwindRule>;
}
