pub trait StreamCipher {
    fn init(key: &[u8]) -> Result<Box<Self>, &'static str>;
    fn process(&self, input: &u8, decrypt: bool) -> u8;
    fn process_stream(key: &[u8], input: &[u8], decrypt: bool) -> Result<Box<[u8]>, &'static str> {
        match Self::init(key) {
            Ok(c) => Ok(input.iter().map(|byte| c.process(byte, decrypt)).collect()),
            Err(msg) => Err(msg)
        }
    }

}

pub mod caesar;
pub mod rc4;
