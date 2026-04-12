#[derive(Debug, Copy, Clone)]
pub struct SmallMoney(i32);

impl SmallMoney {
    pub fn new(bytes: [u8; 4]) -> Self {
        Self(i32::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
        ]))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Money(i64);

impl Money {
    pub fn new(bytes: [u8; 8]) -> Self {
        
        let dollars = i32::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
        ]);
        let cents = u32::from_le_bytes([
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7]
        ]);

        Self((dollars as i64) << 32 | (cents as i64))
    }
}

impl core::fmt::Display for SmallMoney {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let total = self.0.abs();
        let dollars = total / 10_000;
        let cents =  total % 10_000;
        if self.0 < 0 {
            return write!(f, "-${}.{:04}", dollars, cents)
        }
        write!(f, "${}.{:04}", dollars, cents)
    }
}

impl core::fmt::Display for Money {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let total = self.0.abs();
        let dollars = total / 10_000;
        let cents =  total % 10_000;
        if self.0 < 0 {
            return write!(f, "-${}.{:04}", dollars, cents)
        }
        write!(f, "${}.{:04}", dollars, cents)
    }
}