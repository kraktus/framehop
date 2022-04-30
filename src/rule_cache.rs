use crate::unwind_rule::UnwindRule;

pub struct RuleCache<R: UnwindRule> {
    entries: Box<[Option<CacheEntry<R>>; 509]>,
}

impl<R: UnwindRule> RuleCache<R> {
    pub fn new() -> Self {
        Self {
            entries: Box::new([None; 509]),
        }
    }

    pub fn lookup(&mut self, address: u64, modules_generation: u16) -> CacheResult<R> {
        let slot = (address % 509) as u16;
        if let Some(entry) = &self.entries[slot as usize] {
            if entry.modules_generation == modules_generation && entry.address == address {
                return CacheResult::Hit(entry.unwind_rule);
            }
        }
        CacheResult::Miss(CacheHandle {
            slot,
            address,
            modules_generation,
        })
    }

    pub fn insert(&mut self, handle: CacheHandle, unwind_rule: R) {
        let CacheHandle {
            slot,
            address,
            modules_generation,
        } = handle;
        self.entries[slot as usize] = Some(CacheEntry {
            address,
            modules_generation,
            unwind_rule,
        });
    }
}

pub enum CacheResult<R: UnwindRule> {
    Miss(CacheHandle),
    Hit(R),
}

pub struct CacheHandle {
    slot: u16,
    address: u64,
    modules_generation: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct CacheEntry<R: UnwindRule> {
    address: u64,
    modules_generation: u16,
    unwind_rule: R,
}
