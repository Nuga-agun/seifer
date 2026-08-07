use super::StreamCipher;

const EMPTY_KEY_MSG: &'static str = "The key is missing!";

pub struct Caesar {
    key: u8,
}
impl StreamCipher for Caesar {
    fn init(key: &[u8]) -> Result<Box<Self>, &'static str> {
        if key.len() < 1 {
            return Err(EMPTY_KEY_MSG);
        }
        Ok(Box::new(Caesar { key: key[0] }))
    }
    fn process(&mut self, input: &u8, decrypt: bool) -> u8 {
        if decrypt {
            return input.wrapping_add(self.key);
        } else {
            return input.wrapping_sub(self.key);
        }
    }
}

#[cfg(test)]
mod tests_caesar {
    use super::*;

    #[test]
    fn test_empty_key_initialization() {
        match Caesar::init("".as_bytes()) {
            Ok(_) => panic!(),
            Err(msg) => assert_eq!(msg,EMPTY_KEY_MSG)
        }
    }
}

