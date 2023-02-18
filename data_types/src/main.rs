fn main() {
    integers_values();
}

fn integers_values() {
    println!("\n*** Integer Types Values ***");

    println!("\nUnsigned variants can store numbers from 0 to 2ⁿ - 1");
    println!("u8: 0 - {}", u8::max_value());
    println!("u16: 0 - {}", u16::max_value());
    println!("u32: 0 - {}", u32::max_value());
    println!("u64: 0 - {}", u64::max_value());
    println!("u128: 0 - {}", u128::max_value());
    println!("usize: (arch-dependent) 0 - {}", usize::max_value());

    println!("\nSigned variants can store numbers from -2ⁿ⁻¹ to 2ⁿ⁻¹ - 1");
    println!("i8: {} - {}", i8::min_value(), i8::max_value());
    println!("i16: {} - {}", i16::min_value(), i16::max_value());
    println!("i32: {} - {}", i32::min_value(), i32::max_value());
    println!("i64: {} - {}", i64::min_value(), i64::max_value());
    println!("i128: {} - {}", i128::min_value(), i128::max_value());
    println!(
        "isize: (arch-dependent) {} - {}",
        isize::min_value(),
        isize::max_value()
    );

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
