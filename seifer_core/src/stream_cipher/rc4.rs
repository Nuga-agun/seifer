use super::StreamCipher;

const EMPTY_KEY_MSG: &'static str = "The key is missing!";

pub struct Rc4 {
    s: [u8;256],
    i: u8,
    j: u8
}

impl StreamCipher for Rc4 {
    fn init(key: &[u8]) -> Result<Box<Self>, &'static str> {
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
    fn process(&mut self, input: &u8, _decrypt: bool) -> u8 {
        self.i = self.i.wrapping_add(1);
        let si = self.s[self.i as usize];
        self.j = self.j.wrapping_add(si);
        let sj = self.s[self.j as usize];
        utils::swap_bytes(&mut self.s, si as usize, sj as usize);
        self.s[si.wrapping_add(sj) as usize]^input
    }
}

#[cfg(test)]
mod test_rc4 {
    use super::*;

    #[test]
    fn empty_key_initialization() {
        match Rc4::init("".as_bytes()) {
            Ok(_) => panic!(),
            Err(msg) => assert_eq!(msg,EMPTY_KEY_MSG)
        }
    }

    #[test]
    fn big_key_initialization() {
        let big_key: [u8;500] = [0;500];
        Rc4::init(&big_key).unwrap();
    }
}
