#[inline(always)]
pub fn r_u16_be(buf: &[u8], ib: usize) -> u16 {
    let lo = buf[ib+1]; // ib+1 checked first
    u16::from_be_bytes([buf[ib], lo])
}
#[inline(always)]
pub fn r_u16_le(buf: &[u8], ib: usize) -> u16 {
    let hi = buf[ib+1];
    u16::from_le_bytes([buf[ib], hi])
}
#[inline(always)]
pub fn r_i16_le(buf: &[u8], ib: usize) -> i16 {
    let hi = buf[ib+1];
    i16::from_le_bytes([buf[ib], hi])
}
#[inline(always)]
pub fn r_u32_le(buf: &[u8], ib: usize) -> u32 {
    let b3 = buf[ib+3];
    u32::from_le_bytes([buf[ib], buf[ib+1], buf[ib+2], b3])
}
#[inline(always)]
pub fn r_i32_le(buf: &[u8], ib: usize) -> i32 {
    let b3 = buf[ib+3];
    i32::from_le_bytes([buf[ib], buf[ib+1], buf[ib+2], b3])
}
#[inline(always)]
pub fn r_f32_le(buf: &[u8], ib: usize) -> f32 {
    let b3 = buf[ib+3];
    f32::from_le_bytes([buf[ib], buf[ib+1], buf[ib+2], b3])
}