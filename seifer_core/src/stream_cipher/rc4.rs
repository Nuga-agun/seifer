use super::StreamCipher;

const EMPTY_KEY_MSG: &'static str = "The key is missing!";

pub struct Rc4 {
    s: [u8;256],
    i: u8,
    j: u8
}

impl StreamCipher for Rc4 {
    type Input = u8;
    type Key = Vec<u8>;
    type Iv = ();
    fn init(key: &Self::Key, _iv: &Self::Iv) -> Result<Box<Self>, &'static str> {
        if key.len() == 0 {
            return Err(EMPTY_KEY_MSG);
        }
        let mut s: [u8;256] = core::array::from_fn(|i| i as u8);
        let mut j: u8 = 0;
        for i in 0..256 {
            let offset = key[i%key.len()];
            j = j.wrapping_add(s[i as usize]).wrapping_add(offset);
            utils::swap_bytes(&mut s, i, j as usize);
        }
        Ok(Box::new(Rc4{s, i:0, j:0}))
    }
    fn process(&mut self, input: &Self::Input, _decrypt: bool) -> u8 {
        self.i = self.i.wrapping_add(1);
        let si = self.s[self.i as usize];
        self.j = self.j.wrapping_add(si);
        let sj = self.s[self.j as usize];
        utils::swap_bytes(&mut self.s, si as usize, sj as usize);
        self.s[si.wrapping_add(sj) as usize]^input
    }
    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str> {
        match bytes.len() {
            0 => Err(EMPTY_KEY_MSG),
            _ => Ok(bytes.to_vec())
        }
    }

}

#[cfg(test)]
mod test_rc4 {
    use super::*;

    #[test]
    fn empty_key_initialization() {
        match Rc4::init(&Vec::new(), &()) {
            Ok(_) => panic!(),
            Err(msg) => assert_eq!(msg,EMPTY_KEY_MSG)
        }
    }

    #[test]
    fn big_key_initialization() {
        Rc4::init(&[0u8;500].to_vec(), &()).unwrap();
    }

    #[test]
    fn encryption_decryption() {
        let key = "clef".as_bytes().to_vec();
        let mut encryptor = Rc4::init(&key, &()).unwrap();
        let mut decryptor = Rc4::init(&key, &()).unwrap();
        assert_eq!(decryptor.process(&encryptor.process(&1u8, false), true), 1u8);
    }
}
