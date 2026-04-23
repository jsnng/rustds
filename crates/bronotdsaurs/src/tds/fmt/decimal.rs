/// TDS DecimalN / NumericN display formatter.
///
/// Wire layout (after the 1-byte length prefix consumed by the row decoder):
///   byte 0     — sign: 0 = negative, 1 = positive
///   bytes 1..  — unsigned magnitude, little-endian (4, 8, 12, or 16 bytes)
///
/// `scale` (from column metadata) tells us how many digits follow the decimal point.
#[derive(Debug, Copy, Clone)]
pub struct Decimal<'a> {
    bytes: &'a [u8],
    scale: u8,
}

impl<'a> Decimal<'a> {
    #[inline]
    pub fn new(bytes: &'a [u8], scale: u8) -> Self {
        Self { bytes, scale }
    }

    #[inline]
    fn magnitude(&self) -> u128 {
        let mag = &self.bytes[1..];
        let mut buf = [0u8; 16];
        let len = mag.len().min(16);
        buf[..len].copy_from_slice(&mag[..len]);
        u128::from_le_bytes(buf)
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.bytes[0] == 0
    }
}

impl core::fmt::Display for Decimal<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.bytes.is_empty() {
            return f.write_str("NULL");
        }

        let mag = self.magnitude();
        let scale = self.scale as usize;

        if self.is_negative() {
            f.write_str("-")?;
        }

        if scale == 0 {
            return write!(f, "{}", mag);
        }

        // Write digits directly without allocating.
        // u128::MAX is 39 digits, so 40 is enough.
        let mut buf = [b'0'; 40];
        let mut v = mag;
        let mut pos = buf.len();
        if v == 0 {
            pos -= 1;
            // buf already filled with '0'
        } else {
            while v > 0 {
                pos -= 1;
                buf[pos] = b'0' + (v % 10) as u8;
                v /= 10;
            }
        }
        let digits = &buf[pos..];
        let digit_count = digits.len();

        if digit_count <= scale {
            // All digits are fractional: "0.000...digits"
            f.write_str("0.")?;
            for _ in 0..scale - digit_count {
                f.write_str("0")?;
            }
            // SAFETY: digits are ASCII 0-9
            f.write_str(unsafe { core::str::from_utf8_unchecked(digits) })
        } else {
            let split = digit_count - scale;
            let (integer, fraction) = digits.split_at(split);
            // SAFETY: digits are ASCII 0-9
            unsafe {
                f.write_str(core::str::from_utf8_unchecked(integer))?;
                f.write_str(".")?;
                f.write_str(core::str::from_utf8_unchecked(fraction))
            }
        }
    }
}
