#[derive(Debug, Copy, Clone)]
pub struct Guid([u8;16]);

impl Guid {
    pub fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl core::fmt::Display for Guid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let g = u32::from_le_bytes([self.0[0],
            self.0[1], self.0[2], self.0[3]]);
        let u = u16::from_le_bytes([self.0[4],
            self.0[5]]);
        let i = u16::from_le_bytes([self.0[6],
            self.0[7]]);
        let d = &self.0[8..];
        write!(f, "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        g, u, i, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7])
    }
}