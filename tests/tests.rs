#![allow(overflowing_literals)]

use byteorder_slice::ReadSlice;
use hex_literal::hex;
use byteorder_slice::{BigEndian, LittleEndian};

#[test]
fn u8() {
    let mut data = hex!("01 80").as_ref();
    assert_eq!(data.read_u8(), Some(1));
    assert_eq!(data.read_u8(), Some(128));
    assert_eq!(data.read_u8(), None);
}

#[test]
fn i8() {
    let mut data = hex!("01 80").as_ref();
    assert_eq!(data.read_i8(), Some(1));
    assert_eq!(data.read_i8(), Some(-128));
    assert_eq!(data.read_i8(), None);
}

#[test]
fn u16_be() {
    let mut data = hex!("0102 8000").as_ref();
    assert_eq!(data.read_u16::<BigEndian>(), Some(258));
    assert_eq!(data.read_u16::<BigEndian>(), Some(32768));
    assert_eq!(data.read_u16::<BigEndian>(), None);
}

#[test]
fn u16_le() {
    let mut data = hex!("0201 0080").as_ref();
    assert_eq!(data.read_u16::<LittleEndian>(), Some(258));
    assert_eq!(data.read_u16::<LittleEndian>(), Some(32768));
    assert_eq!(data.read_u16::<LittleEndian>(), None);
}

#[test]
fn i16_be() {
    let mut data = hex!("0102 8000").as_ref();
    assert_eq!(data.read_i16::<BigEndian>(), Some(258));
    assert_eq!(data.read_i16::<BigEndian>(), Some(-32768));
    assert_eq!(data.read_i16::<BigEndian>(), None);
}

#[test]
fn i16_le() {
    let mut data = hex!("0201 0080").as_ref();
    assert_eq!(data.read_i16::<LittleEndian>(), Some(258));
    assert_eq!(data.read_i16::<LittleEndian>(), Some(-32768));
    assert_eq!(data.read_i16::<LittleEndian>(), None);
}

#[test]
fn u32_be() {
    let mut data = hex!("01020304 80000000").as_ref();
    assert_eq!(data.read_u32::<BigEndian>(), Some(16909060));
    assert_eq!(data.read_u32::<BigEndian>(), Some(2147483648));
    assert_eq!(data.read_u32::<BigEndian>(), None);
}

#[test]
fn u32_le() {
    let mut data = hex!("04030201 00000080").as_ref();
    assert_eq!(data.read_u32::<LittleEndian>(), Some(16909060));
    assert_eq!(data.read_u32::<LittleEndian>(), Some(2147483648));
    assert_eq!(data.read_u32::<LittleEndian>(), None);
}

#[test]
fn i32_be() {
    let mut data = hex!("01020304 80000000").as_ref();
    assert_eq!(data.read_i32::<BigEndian>(), Some(16909060));
    assert_eq!(data.read_i32::<BigEndian>(), Some(-2147483648));
    assert_eq!(data.read_i32::<BigEndian>(), None);
}

#[test]
fn i32_le() {
    let mut data = hex!("04030201 00000080").as_ref();
    assert_eq!(data.read_i32::<LittleEndian>(), Some(16909060));
    assert_eq!(data.read_i32::<LittleEndian>(), Some(-2147483648));
    assert_eq!(data.read_i32::<LittleEndian>(), None);
}

#[test]
fn u64_be() {
    let mut data = hex!("0102030405060708 8000000000000000").as_ref();
    assert_eq!(data.read_u64::<BigEndian>(), Some(72623859790382856));
    assert_eq!(data.read_u64::<BigEndian>(), Some(9223372036854775808));
    assert_eq!(data.read_u64::<BigEndian>(), None);
}

#[test]
fn u64_le() {
    let mut data = hex!("0807060504030201 0000000000000080").as_ref();
    assert_eq!(data.read_u64::<LittleEndian>(), Some(72623859790382856));
    assert_eq!(data.read_u64::<LittleEndian>(), Some(9223372036854775808));
    assert_eq!(data.read_u64::<LittleEndian>(), None);
}

#[test]
fn i64_be() {
    let mut data = hex!("0102030405060708 8000000000000000").as_ref();
    assert_eq!(data.read_i64::<BigEndian>(), Some(72623859790382856));
    assert_eq!(data.read_i64::<BigEndian>(), Some(-9223372036854775808));
    assert_eq!(data.read_i64::<BigEndian>(), None);
}

#[test]
fn i64_le() {
    let mut data = hex!("0807060504030201 0000000000000080").as_ref();
    assert_eq!(data.read_i64::<LittleEndian>(), Some(72623859790382856));
    assert_eq!(data.read_i64::<LittleEndian>(), Some(-9223372036854775808));
    assert_eq!(data.read_i64::<LittleEndian>(), None);
}

#[test]
fn u128_be() {
    let mut data = hex!("0102030405060708090A0B0C0D0E0F10 80000000000000000000000000000000").as_ref();
    assert_eq!(data.read_u128::<BigEndian>(), Some(1339673755198158349044581307228491536));
    assert_eq!(data.read_u128::<BigEndian>(), Some(170141183460469231731687303715884105728));
    assert_eq!(data.read_u128::<BigEndian>(), None);
}

#[test]
fn u128_le() {
    let mut data = hex!("0102030405060708090A0B0C0D0E0F10 80000000000000000000000000000000").as_ref();
    assert_eq!(data.read_u128::<BigEndian>(), Some(1339673755198158349044581307228491536));
    assert_eq!(data.read_u128::<BigEndian>(), Some(170141183460469231731687303715884105728));
    assert_eq!(data.read_u128::<BigEndian>(), None);
}

#[test]
fn i128_be() {
    let mut data = hex!("0102030405060708090A0B0C0D0E0F10 80000000000000000000000000000000").as_ref();
    assert_eq!(data.read_i128::<BigEndian>(), Some(1339673755198158349044581307228491536));
    assert_eq!(data.read_i128::<BigEndian>(), Some(-170141183460469231731687303715884105728));
    assert_eq!(data.read_i128::<BigEndian>(), None);
}

#[test]
fn i128_le() {
    let mut data = hex!("100F0E0D0C0B0A090807060504030201 00000000000000000000000000000080").as_ref();
    assert_eq!(data.read_i128::<LittleEndian>(), Some(1339673755198158349044581307228491536));
    assert_eq!(data.read_i128::<LittleEndian>(), Some(-170141183460469231731687303715884105728));
    assert_eq!(data.read_i128::<LittleEndian>(), None);
}


#[test]
fn uint_be() {
    let mut data = hex!("01 0102 010203 01020304 0102030405 010203040506 01020304050607 0102030405060708
                         010203040506070809 0102030405060708090A 0102030405060708090A0B 0102030405060708090A0B0C
                         0102030405060708090A0B0C0D 0102030405060708090A0B0C0D0E 0102030405060708090A0B0C0D0E0F 0102030405060708090A0B0C0D0E0F10
                         0102030405060708090A0B0C0D0E0F1011").as_ref();
    assert_eq!(data.read_uint::<BigEndian>(0), None);
    assert_eq!(data.read_uint::<BigEndian>(1), Some(1));
    assert_eq!(data.read_uint::<BigEndian>(2), Some(258));
    assert_eq!(data.read_uint::<BigEndian>(3), Some(66051));
    assert_eq!(data.read_uint::<BigEndian>(4), Some(16909060));
    assert_eq!(data.read_uint::<BigEndian>(5), Some(4328719365));
    assert_eq!(data.read_uint::<BigEndian>(6), Some(1108152157446));
    assert_eq!(data.read_uint::<BigEndian>(7), Some(283686952306183));
    assert_eq!(data.read_uint::<BigEndian>(8), Some(72623859790382856));
    assert_eq!(data.read_uint::<BigEndian>(9), Some(18591708106338011145));
    assert_eq!(data.read_uint::<BigEndian>(10), Some(4759477275222530853130));
    assert_eq!(data.read_uint::<BigEndian>(11), Some(1218426182456967898401291));
    assert_eq!(data.read_uint::<BigEndian>(12), Some(311917102708983781990730508));
    assert_eq!(data.read_uint::<BigEndian>(13), Some(79850778293499848189627010061));
    assert_eq!(data.read_uint::<BigEndian>(14), Some(20441799243135961136544514575630));
    assert_eq!(data.read_uint::<BigEndian>(15), Some(5233100606242806050955395731361295));
    assert_eq!(data.read_uint::<BigEndian>(16), Some(1339673755198158349044581307228491536));
    assert_eq!(data.read_uint::<BigEndian>(17), None);
}

#[test]
fn uint_le() {
    let mut data = hex!("01 0201 030201 04030201 0504030201 060504030201 07060504030201 0807060504030201
                         090807060504030201 0A090807060504030201 0B0A090807060504030201 0C0B0A090807060504030201
                         0D0C0B0A090807060504030201 0E0D0C0B0A090807060504030201 0F0E0D0C0B0A090807060504030201 100F0E0D0C0B0A090807060504030201
                         11100F0E0D0C0B0A090807060504030201").as_ref();
    assert_eq!(data.read_uint::<LittleEndian>(0), None);
    assert_eq!(data.read_uint::<LittleEndian>(1), Some(1));
    assert_eq!(data.read_uint::<LittleEndian>(2), Some(258));
    assert_eq!(data.read_uint::<LittleEndian>(3), Some(66051));
    assert_eq!(data.read_uint::<LittleEndian>(4), Some(16909060));
    assert_eq!(data.read_uint::<LittleEndian>(5), Some(4328719365));
    assert_eq!(data.read_uint::<LittleEndian>(6), Some(1108152157446));
    assert_eq!(data.read_uint::<LittleEndian>(7), Some(283686952306183));
    assert_eq!(data.read_uint::<LittleEndian>(8), Some(72623859790382856));
    assert_eq!(data.read_uint::<LittleEndian>(9), Some(18591708106338011145));
    assert_eq!(data.read_uint::<LittleEndian>(10), Some(4759477275222530853130));
    assert_eq!(data.read_uint::<LittleEndian>(11), Some(1218426182456967898401291));
    assert_eq!(data.read_uint::<LittleEndian>(12), Some(311917102708983781990730508));
    assert_eq!(data.read_uint::<LittleEndian>(13), Some(79850778293499848189627010061));
    assert_eq!(data.read_uint::<LittleEndian>(14), Some(20441799243135961136544514575630));
    assert_eq!(data.read_uint::<LittleEndian>(15), Some(5233100606242806050955395731361295));
    assert_eq!(data.read_uint::<LittleEndian>(16), Some(1339673755198158349044581307228491536));
    assert_eq!(data.read_uint::<LittleEndian>(17), None);
}

#[test]
fn int_be() {
    let mut data = hex!("81 8182 818283 81828384 8182838485 818283848586 81828384858687 8182838485868788
                         818283848586878889 8182838485868788898A 8182838485868788898A8B 8182838485868788898A8B8C
                         8182838485868788898A8B8C8D 8182838485868788898A8B8C8D8E 8182838485868788898A8B8C8D8E8F 8182838485868788898A8B8C8D8E8F18
                         8182838485868788898A8B8C8D8E8F1811").as_ref();
    assert_eq!(data.read_int::<BigEndian>(0), None);
    assert_eq!(data.read_int::<BigEndian>(1), Some(-127));
    assert_eq!(data.read_int::<BigEndian>(2), Some(-32382));
    assert_eq!(data.read_int::<BigEndian>(3), Some(-8289661));
    assert_eq!(data.read_int::<BigEndian>(4), Some(-2122153084));
    assert_eq!(data.read_int::<BigEndian>(5), Some(-543271189371));
    assert_eq!(data.read_int::<BigEndian>(6), Some(-139077424478842));
    assert_eq!(data.read_int::<BigEndian>(7), Some(-35603820666583417));
    assert_eq!(data.read_int::<BigEndian>(8), Some(-9114578090645354616));
    assert_eq!(data.read_int::<BigEndian>(9), Some(-2333331991205210781559));
    assert_eq!(data.read_int::<BigEndian>(10), Some(-597332989748533960078966));
    assert_eq!(data.read_int::<BigEndian>(11), Some(-152917245375624693780215157));
    assert_eq!(data.read_int::<BigEndian>(12), Some(-39146814816159921607735080052));
    assert_eq!(data.read_int::<BigEndian>(13), Some(-10021584592936939931580180493171));
    assert_eq!(data.read_int::<BigEndian>(14), Some(-2565525655791856622484526206251634));
    assert_eq!(data.read_int::<BigEndian>(15), Some(-656774567882715295356038708800418161));
    assert_eq!(data.read_int::<BigEndian>(16), Some(-168134289377975115611145909452907049192));
    assert_eq!(data.read_int::<BigEndian>(17), None);

    assert_eq!(hex!("FF7F").as_ref().read_int::<BigEndian>(2), Some(-129));
}

#[test]
fn int_le() {
    let mut data = hex!("81 8281 838281 84838281 8584838281 868584838281 87868584838281 8887868584838281
                         898887868584838281 8A898887868584838281 8B8A898887868584838281 8C8B8A898887868584838281
                         8D8C8B8A898887868584838281 8E8D8C8B8A898887868584838281 8F8E8D8C8B8A898887868584838281 188F8E8D8C8B8A898887868584838281
                         11188F8E8D8C8B8A898887868584838281").as_ref();
    assert_eq!(data.read_int::<LittleEndian>(0), None);
    assert_eq!(data.read_int::<LittleEndian>(1), Some(-127));
    assert_eq!(data.read_int::<LittleEndian>(2), Some(-32382));
    assert_eq!(data.read_int::<LittleEndian>(3), Some(-8289661));
    assert_eq!(data.read_int::<LittleEndian>(4), Some(-2122153084));
    assert_eq!(data.read_int::<LittleEndian>(5), Some(-543271189371));
    assert_eq!(data.read_int::<LittleEndian>(6), Some(-139077424478842));
    assert_eq!(data.read_int::<LittleEndian>(7), Some(-35603820666583417));
    assert_eq!(data.read_int::<LittleEndian>(8), Some(-9114578090645354616));
    assert_eq!(data.read_int::<LittleEndian>(9), Some(-2333331991205210781559));
    assert_eq!(data.read_int::<LittleEndian>(10), Some(-597332989748533960078966));
    assert_eq!(data.read_int::<LittleEndian>(11), Some(-152917245375624693780215157));
    assert_eq!(data.read_int::<LittleEndian>(12), Some(-39146814816159921607735080052));
    assert_eq!(data.read_int::<LittleEndian>(13), Some(-10021584592936939931580180493171));
    assert_eq!(data.read_int::<LittleEndian>(14), Some(-2565525655791856622484526206251634));
    assert_eq!(data.read_int::<LittleEndian>(15), Some(-656774567882715295356038708800418161));
    assert_eq!(data.read_int::<LittleEndian>(16), Some(-168134289377975115611145909452907049192));
    assert_eq!(data.read_int::<LittleEndian>(17), None);
}

