#[macro_export]
macro_rules! span {
    ($( $(#[$attr:meta])* $name:ident ),* $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy)]
            $(#[$attr])*
            pub struct $name<'a> { pub bytes: &'a [u8] }
        )*
    };
}
