pub trait StreamCipher {
    fn init(key: &[u8]) -> Result<Box<Self>, &'static str>;
    fn process(&self, input: &u8, decrypt: bool) -> u8;
    fn process_stream(key: &[u8], input: &[u8], decrypt: bool)->Result<Box<[u8]>, &'static str>;
}

pub mod caesar;
