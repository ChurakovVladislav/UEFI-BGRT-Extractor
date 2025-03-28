#[macro_export]
macro_rules! signature_16 {
    ($a:expr, $b:expr) => {
        (($a as u32) | (($b as u32) << 8))
    };
}

#[macro_export]
macro_rules! signature_32 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ((($a as u32) | (($b as u32) << 8)) | ((($c as u32) | (($d as u32) << 8)) << 16))
    };
}
