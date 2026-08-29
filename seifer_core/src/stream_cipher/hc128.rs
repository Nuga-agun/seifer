use super::StreamCipher;
use getrandom::rand_core::{Rng,UnwrapErr};
use getrandom::SysRng;

const INVALID_KEY_SIZE_MSG: &'static str = "The key must be 128 bits long";
const INVALID_IV_SIZE_MSG: &'static str = "The initialization vector must be 128 bits long";

pub struct Hc128 {
    p: [u32;512],
    q: [u32;512],
    counter: u16,
    keystream_word: u32,
    is_init: bool,
}

impl StreamCipher for Hc128 {
    type Input = u32;
    type Key = Vec<u32>;
    type Iv = Vec<u32>;

    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str> {
        if bytes.len() < 16 {
            return Err(INVALID_KEY_SIZE_MSG);
        }
        let mut key: Vec<u32> = Vec::new();
        for i in 0..4 {
            let mut key_part = 0u32;
            for j in 0..4 {key_part += (bytes[4*i+j] as u32)<<(8*j)}
            key.push(key_part);
        }
        Ok(key)
    }
    fn stream_from_bytes(bytes: &[u8]) -> Vec<Self::Input> {
        let mut result: Vec<u32> = Vec::new();
        for i in 0..=bytes.len()/4 {
            let mut block = 0u32;
            for j in 0..4 {if 4*i+j < bytes.len() {block += (bytes[4*i+j] as u32)<<(8*j)}}
            result.push(block);
        }
        result
    }
    fn bytes_from_input(input: Vec<Self::Input>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for input_part in input {
            for j in 0..4 {result.push((input_part>>(8*j)) as u8)}
        }
        result
    }

    fn init(key: &Self::Key, iv: &Self::Iv) -> Result<Box<Self>, &'static str> {
        let mut w: [u32;1280] = [0;1280];
        for i in 0..4 {
            w[i] = key[i];
            w[i+4] = key[i];
            w[i+8] = iv[i];
            w[i+12] = iv[i];
        }
        for i in 16..1280 {
            w[i] = f2(w[i-2]).wrapping_add(w[i-7]).wrapping_add(f1(w[i-15])).wrapping_add(w[i-16]);
        }

        let counter = 0;
        let mut p: [u32;512] = [0;512];
        let mut q: [u32;512] = [0;512];
        for i in 0..512 {
            p[i] = w[i+512];
            q[i] = w[i+768];
        }
        let mut hc128 = Hc128{counter, p, q, keystream_word: 0, is_init: true};
        for _ in 0..1024 {
            _ = hc128.process(&0u32, false);
        }
        hc128.is_init = false;
        Ok(Box::new(hc128))
    }
    fn process(&mut self, input: &Self::Input, _decrypt: bool) -> Self::Input {
        let i = (self.counter % 512) as usize;
        let i3 = (i + 509)%512;
        let i10 = (i + 502)%512;
        let i12 = (i + 500)%512;
        let i511 = (i + 1)%512;
        if self.counter < 512 {
            self.p[i] = self.p[i].wrapping_add(g1(self.p[i3], self.p[i10], self.p[i511]));
            if self.is_init {
                self.p[i] = self.h1(self.p[i12])^self.p[i];
            } else {
                self.keystream_word = self.h1(self.p[i12])^self.p[i];
            }
        }
        else {
            self.q[i] = self.q[i].wrapping_add(g2(self.q[i3], self.q[i10], self.q[i511]));
            if self.is_init {
                self.q[i] = self.h2(self.q[i12])^self.q[i];
            } else {
                self.keystream_word = self.h2(self.q[i12])^self.q[i];
            }
        }
        self.counter = (self.counter+1)%1024;
        input^self.keystream_word
    }

    fn process_stream(key_bytes: &[u8], iv: &Self::Iv, message: &[u8], decrypt: bool) -> Result<Vec<u8>, &'static str> {
        let key = Self::key_from_bytes(key_bytes).unwrap_or_else(|msg| panic!("Error while creating the key : {msg}"));
        let mut c = Self::init(&key, iv).unwrap_or_else(|msg| panic!("Error while initialization : {msg}"));
        let input = Self::stream_from_bytes(message);
        let result = Self::bytes_from_input(input.iter().map(|input| c.process(input, decrypt)).collect());
        Ok(result[0..message.len()].to_vec())
    }
}

impl Hc128 {
    pub fn random_iv() -> Vec<u32> {
        let mut iv: Vec<u32> = Vec::new();
        let mut rng = UnwrapErr(SysRng);
        iv.push(rng.next_u32());
        iv.push(rng.next_u32());
        iv.push(rng.next_u32());
        iv.push(rng.next_u32());
        iv
    }
    pub fn iv_from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
        if bytes.len() < 16 {
            return Err(INVALID_IV_SIZE_MSG);
        }
        let mut iv: Vec<u32> = Vec::new();
        for i in 0..4 {
            let mut iv_part = 0u32;
            for j in 0..4 {iv_part += (bytes[4*i+j] as u32)<<(8*j)}
            iv.push(iv_part);
        }
        Ok(iv)
    }
    pub fn bytes_from_iv(iv: &[u32]) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for iv_part in iv {
            for j in 0..4 {bytes.push((iv_part>>(8*j)) as u8)}
        }
        bytes
    }

    fn h1(&self, u: u32) -> u32 {
        self.q[(u&0xff) as usize].wrapping_add(self.q[(((u>>16)&0xff) + 256) as usize])
    }
    fn h2(&self, u: u32) -> u32 {
        self.p[(u&0xff) as usize].wrapping_add(self.q[(((u>>16)&0xff)) as usize])
    }
}

fn circular_left_shift(x: u32, n: u8) -> u32 {
    x<<n | x>>(32-n)
}
fn circular_right_shift(x: u32, n: u8) -> u32 {
    x>>n | x<<(32-n)
}
fn f1(x: u32) -> u32 {
    (circular_right_shift(x,7))^(circular_right_shift(x,18))^(x>>3)
}
fn f2(x: u32) -> u32 {
    (circular_right_shift(x,17))^(circular_right_shift(x,19))^(x>>10)
}
fn g1(x: u32, y: u32, z: u32) -> u32 {
    (circular_right_shift(x,10)^circular_right_shift(z,23)).wrapping_add(circular_right_shift(y,8))
}
fn g2(x: u32, y: u32, z: u32) -> u32 {
    (circular_left_shift(x,10)^circular_left_shift(z,23)).wrapping_add(circular_left_shift(y,8))
}
