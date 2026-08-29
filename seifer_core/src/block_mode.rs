pub trait BlockMode {
    fn to_8(bytes: &[u8], iv: &[u8]) -> Vec<u8>;
    fn to_16(bytes: &[u8], iv: &[u8]) -> Vec<u16>;
    fn to_32(bytes: &[u8], iv: &[u8]) -> Vec<u32>;
    fn to_64(bytes: &[u8], iv: &[u8]) -> Vec<u64>;

    fn from_8(bytes: &[u8], iv: &[u8]) -> Vec<u8>;
    fn from_16(bytes: &[u16], iv: &[u8]) -> Vec<u8>;
    fn from_32(bytes: &[u32], iv: &[u8]) -> Vec<u8>;
    fn from_64(bytes: &[u64], iv: &[u8]) -> Vec<u8>;
}

pub mod ecb;
