#![allow(clippy::cast_lossless)]
mod traits;
mod impls;

pub use traits::{ByteOrder, ReadSlice};
pub use impls::{BigEndian, LittleEndian};