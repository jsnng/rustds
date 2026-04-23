#[derive(Debug, Clone, Copy)]
pub struct ScrollOpt(pub u32);
impl ScrollOpt {
    pub const KEYSET: Self = Self(0x0001);
    pub const DYNAMIC: Self = Self(0x0002);
    pub const FORWARD_ONLY: Self = Self(0x0004);
    pub const STATIC: Self = Self(0x0008);
    pub const FAST_FORWARD: Self = Self(0x10);
    pub const PARAMETERIZED_STMT: Self = Self(0x1000);
    pub const AUTO_FETCH: Self = Self(0x2000);
    pub const AUTO_CLOSE: Self = Self(0x4000);
    pub const CHECK_ACCEPTED_TYPES: Self = Self(0x8000);
    pub const KEYSET_ACCEPTABLE: Self = Self(0x10000);
    pub const DYNAMIC_ACCEPTABLE: Self = Self(0x20000);
    pub const FORWARD_ONLY_ACCEPTABLE: Self = Self(0x40000);
    pub const STATIC_ACCEPTABLE: Self = Self(0x80000);
    pub const FAST_FORWARD_ACCEPTABLE: Self = Self(0x100000);
}