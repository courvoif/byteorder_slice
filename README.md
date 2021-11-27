# byteorder_slice

Provides convenience methods for reading numbers and slices from a slice

## Usage

```rust
use byteorder_slice::{BigEndian, LittleEndian, ReadSlice}
let data = vec![0_u8; 100];
let src = &mut &data[..];

let a = src.read_u8().unwrap();
let b = src.read_u32::<BigEndian>().unwrap();
let c = src.read_uint::<LittleEndian>(3).unwrap();
let d = src.read_slice(10).unwrap();
```
