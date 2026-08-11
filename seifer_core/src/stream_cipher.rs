pub trait StreamCipher {
    type Input;
    type Key;
    type Iv;

    fn init(key: &Self::Key, iv: &Self::Iv) -> Result<Box<Self>, &'static str>;
    fn process(&mut self, input: &Self::Input, decrypt: bool) -> Self::Input;
    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str>;
    fn process_stream(key_bytes: &[u8], iv: &Self::Iv, message: &[Self::Input], decrypt: bool) -> Result<Box<[Self::Input]>, &'static str> {
        match Self::key_from_bytes(key_bytes) {
            Err(msg) => Err(msg),
            Ok(key) => match Self::init(&key, iv) {
                Err(msg) => Err(msg),
                Ok(mut state) => Ok(message.iter().map(|input| state.process(input, decrypt)).collect())
            }
        }
    }

}

pub mod caesar;
pub mod rc4;
