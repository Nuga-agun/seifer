use crate::block_mode::BlockMode;
use crate::block_cipher::BlockCipher;

const ERR_MSG_KEY_LEN: &'static str = "The key must be 128 bits long";
const DELTA: u32 = 0x9e3779b9;
const MAX_SUM: u32 = 0xc6ef3720;

pub struct Tea {}

impl BlockCipher for Tea {
    type Key = [u32;4];
    type Block = u64;

    fn process_stream<T: BlockMode>(key: &[u8], iv: &[u8], bytes: &[u8], decrypt: bool) -> Result<Vec<u8>, &'static str> {
        let key = Self::key_from_bytes(key).unwrap_or_else(|err| panic!("Error: {}", err));
        let blocks = T::to_64(bytes, iv);
        let mut result: Vec<u64> = Vec::new();
        for block in blocks {
            let crypted_block = Self::process(key, block, decrypt);
            result.push(crypted_block);
        }
        Ok(T::from_64(&result, iv))
    }
    fn key_from_bytes(bytes: &[u8]) -> Result<Self::Key, &'static str> {
        if bytes.len() < 16 {
            return Err(ERR_MSG_KEY_LEN);
        }
        let mut result = [0u32;4];
        for i in 0..4 {
            let mut part = 0u32;
            for y in 0..4 {part += (bytes[4*i+y] as u32)<<8*y}
            result[i] = part;
        }
        Ok(result)
    }
    fn process(key: Self::Key, block: Self::Block, decrypt: bool) -> Self::Block {
            let mut state = State{
                v0: block as u32,
                v1: (block>>32) as u32,
                sum: if decrypt {MAX_SUM} else {0u32},
            };
            for _ in 0..32 {
                if decrypt {state.decryption_step(key);} else {state.encryption_step(key);}
            }
            state.v0 as u64 + (state.v1 as u64)<<32
    }
}

struct State {
    v0: u32,
    v1: u32,
    sum: u32,
}

impl State {
    fn encryption_step(&mut self, key: [u32;4]) {
        self.sum = self.sum.wrapping_add(DELTA);
        self.v0 = self.v0.wrapping_add(((self.v1<<4).wrapping_add(key[0])) ^ (self.v1.wrapping_add(self.sum)) ^ ((self.v1>>5).wrapping_add(key[1])));
        self.v1 = self.v1.wrapping_add(((self.v0<<4).wrapping_add(key[2])) ^ (self.v0.wrapping_add(self.sum)) ^ ((self.v0>>5).wrapping_add(key[3])));
    }

    fn decryption_step(&mut self, key: [u32;4]) {
        self.sum = self.sum.wrapping_sub(DELTA);
        self.v1 = self.v1.wrapping_sub(((self.v0<<4).wrapping_add(key[2])) ^ (self.v0.wrapping_add(self.sum)) ^ ((self.v0>>5).wrapping_add(key[3])));
        self.v0 = self.v0.wrapping_sub(((self.v1<<4).wrapping_add(key[0])) ^ (self.v1.wrapping_add(self.sum)) ^ ((self.v1>>5).wrapping_add(key[1])));
    }
}
