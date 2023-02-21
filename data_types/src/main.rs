fn main() {
    integers_values();
}

fn integers_values() {
    println!("\n*** Integer Types Values ***");

    println!("\nUnsigned variants can store numbers from 0 to 2ⁿ - 1");
    println!("u8: 0 - {}", u8::MAX);
    println!("u16: 0 - {}", u16::MAX);
    println!("u32: 0 - {}", u32::MAX);
    println!("u64: 0 - {}", u64::MAX);
    println!("u128: 0 - {}", u128::MAX);
    println!("usize: (arch-dependent) 0 - {}", usize::MAX);

    println!("\nSigned variants can store numbers from -2ⁿ⁻¹ to 2ⁿ⁻¹ - 1");
    println!("i8: {} - {}", i8::MIN, i8::MAX);
    println!("i16: {} - {}", i16::MIN, i16::MAX);
    println!("i32: {} - {}", i32::MIN, i32::MAX);
    println!("i64: {} - {}", i64::MIN, i64::MAX);
    println!("i128: {} - {}", i128::MIN, i128::MAX);
    println!("isize: (arch-dependent) {} - {}", isize::MIN, isize::MAX);

    // *** EXAMPLE OUTPUT ***
    // *** Integer Types Values ***

    // Unsigned variants can store numbers from 0 to 2ⁿ - 1
    // u8: 0 - 255
    // u16: 0 - 65535
    // u32: 0 - 4294967295
    // u64: 0 - 18446744073709551615
    // u128: 0 - 340282366920938463463374607431768211455
    // usize: (arch-dependent) 0 - 18446744073709551615

    // Signed variants can store numbers from -2ⁿ⁻¹ to 2ⁿ⁻¹ - 1
    // i8: -128 - 127
    // i16: -32768 - 32767
    // i32: -2147483648 - 2147483647
    // i64: -9223372036854775808 - 9223372036854775807
    // i128: -170141183460469231731687303715884105728 - 170141183460469231731687303715884105727
    // isize: (arch-dependent) -9223372036854775808 - 9223372036854775807
}
