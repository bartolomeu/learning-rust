fn main() {

    const ANY_CONSTANT: u32 = 12;
    println!("const -> {ANY_CONSTANT}");

    let int8: u8 = 255; //MAX
    let int16: u16 = 65535; //MAX
    let int32: u32 = 4294967295; //MAX
    let int64: u64 = 18446744073709551615 ; //MAX
    let int128: u128 = 340282366920938463463374607431768211455; //MAX

    println!("int - unsigned - 8 b -> {int8}");
    println!("int - unsigned - 16 b -> {int16}");
    println!("int - unsigned - 32 b -> {int32}");
    println!("int - unsigned - 64 b -> {int64}");
    println!("int - unsigned - 128 b -> {int128}");

    let min8: i8 = -128; //MIN
    let max8: i8 = 127; //MAX
    let min16: i16 = -32768; //MIN
    let max16: i16 = 32767; //MAX
    let min32: i32 = -2147483648; //MIN
    let max32: i32 = 2147483647; //MAX
    let min64: i64 = -9223372036854775808; //MIN
    let max64: i64 = 9223372036854775807; //MAX
    let min128: i128 = -170141183460469231731687303715884105728; //MIN
    let max128: i128 = 170141183460469231731687303715884105727; //MAX
    
    println!("int - signed - 8 b = {min8} <-> {max8}");
    println!("int - signed - 16 b = {min16} <-> {max16}");
    println!("int - signed - 32 b = {min32} <-> {max32}");
    println!("int - signed - 64 b = {min64} <-> {max64}");
    println!("int - signed - 128 b = {min128} <-> {max128}");
}
