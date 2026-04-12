#[derive(Debug, Clone, Copy)]
pub struct CcOpt(pub u32);
impl CcOpt {
    pub const READ_ONLY: Self = Self(0x0001);
    pub const SCROLL_LOCKS: Self = Self(0x0002);
    pub const OPTIMISTIC: Self = Self(0x0004); // OPTCC
    pub const OPTIMISTIC_VAL: Self = Self(0x0008); // OPTCCVAL
    pub const ALLOW_DIRECT: Self = Self(0x2000);
    pub const UPDT_IN_PLACE: Self = Self(0x4000);
    pub const CHECK_ACCEPTED_OPTS: Self = Self(0x8000);
    pub const READ_ONLY_ACCEPTABLE: Self = Self(0x10000);
    pub const SCROLL_LOCKS_ACCEPTABLE: Self = Self(0x20000);
    pub const OPTIMISTIC_ACCEPTABLE: Self = Self(0x40000);
    pub const OPTIMISTIC_ACCEPTABLE2: Self = Self(0x80000); // spec typo: OPTIMISITC
}
