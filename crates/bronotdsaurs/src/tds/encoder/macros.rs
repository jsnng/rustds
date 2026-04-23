macro_rules! wint {
    ($buf:expr, $cursor:expr, $ty:ty, $val:expr) => {
        $buf[$cursor..$cursor + size_of::<$ty>()].copy_from_slice(&($val as $ty).to_le_bytes());
        $cursor += size_of::<$ty>();
    };
}

macro_rules! wvec {
    ($buf:expr, $cursor:expr, $val:expr) => {
        $buf[$cursor..$cursor+$val.len()].copy_from_slice(&$val);
        $cursor += $val.len();
    };
}