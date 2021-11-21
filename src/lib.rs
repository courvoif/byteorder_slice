#![allow(clippy::cast_lossless)]
mod traits;
mod byteorder_impl;
mod read_slice_impl;

pub use traits::{ByteOrder, ReadSlice};
pub use byteorder;
pub use byteorder::{BigEndian, LittleEndian, NativeEndian, NetworkEndian};