#![allow(clippy::cast_lossless)]
mod read_slice_impl;
mod traits;

pub use byteorder;
pub use byteorder::{ByteOrder, BigEndian, LittleEndian, NativeEndian, NetworkEndian};

pub use crate::traits::ReadSlice;