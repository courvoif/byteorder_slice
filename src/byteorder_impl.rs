use crate::traits::ByteOrder;

use byteorder::{LittleEndian, BigEndian};

use std::convert::TryInto;

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

    fn read_uint(src: &[u8], nb_bytes: usize) -> Option<u128> {
        if nb_bytes == 0 || nb_bytes > 16 || nb_bytes > src.len() {
            return None;
        }

        let mut out = 0_u128;
        let ptr_out = &mut out as *mut u128 as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), ptr_out.offset((16 - nb_bytes) as isize), nb_bytes);
        }
        Some(out.to_be())
    }

    fn read_int(src: &[u8], nb_bytes: usize) -> Option<i128> {
        if nb_bytes == 0 || nb_bytes > 16 || nb_bytes > src.len() {
            return None;
        }

        // We copy the bytes to the left part of out then make shift to extend the bit sign
        let mut out = 0_i128;
        let ptr_out = &mut out as *mut i128 as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), ptr_out, nb_bytes);
        }

        out = out.to_be();
        out >>= 8*(16-nb_bytes);
        Some(out)
    }
}

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

    fn read_uint(src: &[u8], nb_bytes: usize) -> Option<u128> {
        if nb_bytes == 0 || nb_bytes > 16 || nb_bytes > src.len() {
            return None;
        }

        let mut out = 0_u128;
        let ptr_out = &mut out as *mut u128 as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), ptr_out, nb_bytes);
        }
        Some(out.to_le())
    }

    fn read_int(src: &[u8], nb_bytes: usize) -> Option<i128> {
        if nb_bytes == 0 || nb_bytes > 16 || nb_bytes > src.len() {
            return None;
        }

        // We copy the bytes to the left part of out then make shift to extend the bit sign
        let mut out = 0_i128;
        let ptr_out = &mut out as *mut i128 as *mut u8;
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), ptr_out.offset((16 - nb_bytes) as isize), nb_bytes);
        }

        out = out.to_le();
        out >>= 8*(16-nb_bytes);
        Some(out)
    }
}