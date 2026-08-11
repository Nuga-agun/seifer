use super::StreamCipher;

const EMPTY_KEY_MSG: &'static str = "The key is missing!";

pub struct Caesar {
    key: u8,
}
impl StreamCipher for Caesar {
    type Input = u8;
    type Key = u8;
    type Iv = ();

    fn init(key: &Self::Key, _iv: &Self::Iv) -> Result<Box<Self>, &'static str> {
        Ok(Box::new(Caesar { key: *key }))
    }
    fn process(&mut self, input: &Self::Input, decrypt: bool) -> u8 {
        if decrypt {
            return input.wrapping_add(self.key);
        } else {
            return input.wrapping_sub(self.key);
        }
    }
    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str> {
        match bytes.len() {
            0 => Err(EMPTY_KEY_MSG),
            _ => Ok(bytes[0])
        }
    }
}

#[cfg(test)]
mod tests_caesar {
    use super::*;

    #[test]
    fn test_empty_key() {
        let mut caesar = Caesar::init(&0u8, &()).unwrap();
        assert_eq!(7, caesar.process(&7u8, true));
        assert_eq!(7, caesar.process(&7u8, false));
    }
}

