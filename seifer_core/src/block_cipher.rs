use crate::block_mode::BlockMode;

pub trait BlockCipher {
    type Key;
    type Block;

    fn process_stream<T: BlockMode>(key: &[u8], iv: &[u8], bytes: &[u8], decrypt: bool) -> Result<Vec<u8>, &'static str>;
    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str>;
    fn process(key: Self::Key, block: Self::Block, decrypt: bool) -> Self::Block;
}

pub mod tea;
