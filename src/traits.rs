pub trait ByteOrder {

    fn read_u16(src: &[u8]) -> Option<u16>;
    fn read_u32(src: &[u8]) -> Option<u32>;
    fn read_u64(src: &[u8]) -> Option<u64>;
    fn read_u128(src: &[u8]) -> Option<u128>;
    fn read_uint(src: &[u8], nb_bytes: usize) -> Option<u64>;
}


pub trait ReadSlice<'a> {

    fn read_u8(&mut self) -> Option<u8>;
    fn read_u16<B: ByteOrder>(&mut self) -> Option<u16>;
    fn read_u32<B: ByteOrder>(&mut self) -> Option<u32>;
    fn read_u64<B: ByteOrder>(&mut self) -> Option<u64>;
    fn read_u128<B: ByteOrder>(&mut self) -> Option<u128>;
    fn read_uint<B: ByteOrder>(&mut self, nb_bytes: usize) -> Option<u64>;

    fn read_i8(&mut self) -> Option<i8>;
    fn read_i16<B: ByteOrder>(&mut self) -> Option<i16>;
    fn read_i32<B: ByteOrder>(&mut self) -> Option<i32>;
    fn read_i64<B: ByteOrder>(&mut self) -> Option<i64>;
    fn read_i128<B: ByteOrder>(&mut self) -> Option<i128>;
    fn read_int<B: ByteOrder>(&mut self, nb_bytes: usize) -> Option<i64>;

    fn read_f32<B: ByteOrder>(&mut self) -> Option<f32>;
    fn read_f64<B: ByteOrder>(&mut self) -> Option<f64>;

    fn read_slice(&mut self, nb_bytes: usize) -> Option<&'a [u8]>;
    fn read_slice_to_end(&mut self) -> &'a [u8];

    fn move_forward(&mut self, nb_bytes: usize) -> Option<()>;
}

impl<'a> ReadSlice<'a> for &'a [u8] {

    fn read_u8(&mut self) -> Option<u8> {

        let res = *self.get(0)?;
        *self = &self[1..];
        Some(res)
    }

    fn read_u16<B: ByteOrder>(&mut self) -> Option<u16> {
        let res = B::read_u16(self);
        if res.is_some() {
            *self = &self[2..];
        }
        res
    }

    fn read_u32<B: ByteOrder>(&mut self) -> Option<u32> {
        let res = B::read_u32(self);
        if res.is_some() {
            *self = &self[4..];
        }
        res
    }

    fn read_u64<B: ByteOrder>(&mut self) -> Option<u64> {
        let res = B::read_u64(self);
        if res.is_some() {
            *self = &self[8..];
        }
        res
    }

    fn read_u128<B: ByteOrder>(&mut self) -> Option<u128> {
        let res = B::read_u128(self);
        *self = &self[16..];
        res
    }

    fn read_uint<B: ByteOrder>(&mut self, nb_bytes: usize) -> Option<u64> {
        let res = B::read_uint(self, nb_bytes);
        if res.is_some() {
            *self = &self[nb_bytes..];
        }
        res
    }


    fn read_i8(&mut self) -> Option<i8> {
        self.read_u8().map(|n| n as i8)
    }

    fn read_i16<B: ByteOrder>(&mut self) -> Option<i16> {
        self.read_u16::<B>().map(|n| n as i16)
    }

    fn read_i32<B: ByteOrder>(&mut self) -> Option<i32> {
        self.read_u32::<B>().map(|n| n as i32)
    }

    fn read_i64<B: ByteOrder>(&mut self) -> Option<i64> {
        self.read_u64::<B>().map(|n| n as i64)
    }

    fn read_i128<B: ByteOrder>(&mut self) -> Option<i128> {
        self.read_u128::<B>().map(|n| n as i128)
    }

    fn read_int<B: ByteOrder>(&mut self, nb_bytes: usize) -> Option<i64> {
        self.read_uint::<B>(nb_bytes).map(|n| n as i64)
    }

    fn read_f32<B: ByteOrder>(&mut self) -> Option<f32> {
        self.read_u32::<B>().map(f32::from_bits)
    }

    fn read_f64<B: ByteOrder>(&mut self) -> Option<f64> {
        self.read_u64::<B>().map(f64::from_bits)
    }

    fn read_slice(&mut self, nb_bytes: usize) -> Option<&'a [u8]> {
        if self.len() < nb_bytes {
            None
        }
        else {
            let res = &self[..nb_bytes];
            *self = &self[nb_bytes..];
            Some(res)
        }
    }

    fn read_slice_to_end(&mut self) -> &'a [u8] {
        self.read_slice(self.len()).unwrap()
    }

    fn move_forward(&mut self, nb_bytes: usize) -> Option<()> {
        if self.len() < nb_bytes {
            None
        }
        else {
            *self = &self[nb_bytes..];
            Some(())
        }
    }
}

