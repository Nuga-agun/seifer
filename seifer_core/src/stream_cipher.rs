pub trait StreamCipher {
    type Input;
    type Key;
    type Iv;

    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str>;
    fn stream_from_bytes(bytes: &[u8]) -> Vec<Self::Input>;
    fn bytes_from_input(input: Vec<Self::Input>) -> Vec<u8>;

    fn init(key: &Self::Key, iv: &Self::Iv) -> Result<Box<Self>, &'static str>;
    fn process(&mut self, input: &Self::Input, decrypt: bool) -> Self::Input;
    
    fn process_stream(key_bytes: &[u8], iv: &Self::Iv, message: &[u8], decrypt: bool) -> Result<Vec<u8>, &'static str> {
        let key = Self::key_from_bytes(key_bytes).unwrap_or_else(|msg| panic!("Error while creating the key : {msg}"));
        let mut c = Self::init(&key, iv).unwrap_or_else(|msg| panic!("Error while initialization : {msg}"));
        let input = Self::stream_from_bytes(message);
        let result = input.iter().map(|input| c.process(input, decrypt)).collect();
        Ok(Self::bytes_from_input(result))
    }

}

pub mod caesar;
pub mod rc4;
pub mod hc128;
