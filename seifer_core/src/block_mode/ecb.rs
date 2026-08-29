use super::BlockMode;

pub struct Ecb {}

impl BlockMode for Ecb {
    fn to_8(bytes: &[u8], _iv: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }
    fn to_16(bytes: &[u8], _iv: &[u8]) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::new();
        for i in 0..=bytes.len()/2 {
            let mut block = 0u16;
            for y in 0..2 {if 2*i+y < bytes.len() {block += (bytes[2*i+y] as u16)<<8*y}}
            result.push(block);
        }
        result
    }
    fn to_32(bytes: &[u8], _iv: &[u8]) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        for i in 0..=bytes.len()/4 {
            let mut block = 0u32;
            for y in 0..4 {if 4*i+y < bytes.len() {block += (bytes[4*i+y] as u32)<<8*y}}
            result.push(block);
        }
        result
    }
    fn to_64(bytes: &[u8], _iv: &[u8]) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        for i in 0..=bytes.len()/8 {
            let mut block = 0u64;
            for y in 0..8 {if 8*i+y < bytes.len() {block += (bytes[8*i+y] as u64)<<8*y}}
            result.push(block);
        }
        result
    }

    fn from_8(bytes: &[u8], _iv: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }
    fn from_16(bytes: &[u16], _iv: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        for byte in bytes {
            for y in 0..2 {result.push((byte>>8*y) as u8)}
        }
        result
    }
    fn from_32(bytes: &[u32], _iv: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        for byte in bytes {
            for y in 0..4 {result.push((byte>>8*y) as u8)}
        }
        result
    }
    fn from_64(bytes: &[u64], _iv: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        for byte in bytes {
            for y in 0..8 {result.push((byte>>8*y) as u8)}
        }
        result
    }
}
