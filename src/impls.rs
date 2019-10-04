use std::convert::TryInto;
use crate::traits::ByteOrder;

pub struct BigEndian;
impl ByteOrder for BigEndian {

    fn read_u16(src: &[u8]) -> Option<u16> {

        src.get(..2)
            .map(|b| b.try_into().unwrap())
            .map(u16::from_be_bytes)
    }

    fn read_u32(src: &[u8]) -> Option<u32> {

        src.get(..4)
            .map(|b| b.try_into().unwrap())
            .map(u32::from_be_bytes)
    }

    fn read_u64(src: &[u8]) -> Option<u64> {
        src.get(..8)
            .map(|b| b.try_into().unwrap())
            .map(u64::from_be_bytes)
    }

    fn read_u128(src: &[u8]) -> Option<u128> {
        src.get(..16)
            .map(|b| b.try_into().unwrap())
            .map(u128::from_be_bytes)
    }

    fn read_uint(src: &[u8], nb_bytes: usize) -> Option<u64> {

        if nb_bytes == 0 || nb_bytes > 8 {
            return None;
        }

        if src.len() < nb_bytes {
            return None;
        }

        let mut n = 0;
        for (i, b) in src.iter().take(nb_bytes).enumerate() {
            n += (*b as u64) << ((nb_bytes-1-i)*8) as u64;
        }

        Some(n)
    }
}


pub struct LittleEndian;
impl ByteOrder for LittleEndian {

    fn read_u16(src: &[u8]) -> Option<u16> {

        src.get(..2)
            .map(|b| b.try_into().unwrap())
            .map(u16::from_le_bytes)
    }

    fn read_u32(src: &[u8]) -> Option<u32> {

        src.get(..4)
            .map(|b| b.try_into().unwrap())
            .map(u32::from_le_bytes)
    }

    fn read_u64(src: &[u8]) -> Option<u64> {
        src.get(..8)
            .map(|b| b.try_into().unwrap())
            .map(u64::from_le_bytes)
    }

    fn read_u128(src: &[u8]) -> Option<u128> {
        src.get(..16)
            .map(|b| b.try_into().unwrap())
            .map(u128::from_le_bytes)
    }

    fn read_uint(src: &[u8], nb_bytes: usize) -> Option<u64> {

        if nb_bytes == 0 || nb_bytes > 8 {
            return None;
        }

        if src.len() < nb_bytes {
            return None;
        }

        let mut n = 0;
        for (i, b) in src.iter().take(nb_bytes).enumerate() {
            n += (*b  as u64) << (8*i) as u64;
        }

        Some(n)
    }
}
