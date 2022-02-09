use crate::error::Error;

pub trait UnwindRule: Copy {
    type UnwindRegs;

    fn exec<F>(self, regs: &mut Self::UnwindRegs, read_mem: &mut F) -> Result<u64, Error>
    where
        F: FnMut(u64) -> Result<u64, ()>;
}
