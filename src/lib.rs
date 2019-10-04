#![allow(clippy::cast_lossless)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod traits;
mod impls;

pub use traits::{ByteOrder, ReadSlice};
pub use impls::{BigEndian, LittleEndian};