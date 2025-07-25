fn main() {
    println!("🦀 Primitive types and their bounds 🦀\n");

    // Signed integer types
    println!("=== Signed integer types ===");
    let a: i8 = 1;
    println!("i8:   value = {}, min = {}, max = {}", a, i8::MIN, i8::MAX);
    
    let a: i16 = 1;
    println!("i16:  value = {}, min = {}, max = {}", a, i16::MIN, i16::MAX);
    
    let a: i32 = 1;
    println!("i32:  value = {}, min = {}, max = {}", a, i32::MIN, i32::MAX);
    
    let a: i64 = 1;
    println!("i64:  value = {}, min = {}, max = {}", a, i64::MIN, i64::MAX);
    
    let a: i128 = 1;
    println!("i128: value = {}, min = {}, max = {}", a, i128::MIN, i128::MAX);

    println!("\n=== Unsigned integer types ===");
    let b: u8 = 65;
    println!("u8:   value = {}, min = {}, max = {}", b, u8::MIN, u8::MAX);
    println!("u8:   value as char = {}", b as char);

    let b: u16 = 1;
    println!("u16:  value = {}, min = {}, max = {}", b, u16::MIN, u16::MAX);
    
    let b: u32 = 1;
    println!("u32:  value = {}, min = {}, max = {}", b, u32::MIN, u32::MAX);
    
    let b: u64 = 1;
    println!("u64:  value = {}, min = {}, max = {}", b, u64::MIN, u64::MAX);
    
    let b: u128 = 1;
    println!("u128: value = {}, min = {}, max = {}", b, u128::MIN, u128::MAX);

    println!("\n=== Floating point types ===");
    let c: f32 = 1.0;
    println!("f32:  value = {}, min = {}, max = {}", c, f32::MIN, f32::MAX);
    
    let c: f64 = 1.0;
    println!("f64:  value = {}, min = {}, max = {}", c, f64::MIN, f64::MAX);

    println!("\n=== Additional constants ===");
    println!("f32::INFINITY = {}", f32::INFINITY);
    println!("f32::NEG_INFINITY = {}", f32::NEG_INFINITY);
    println!("f32::NAN = {}", f32::NAN);
    
    // Architecture-dependent types
    println!("\n=== Architecture-dependent types ===");
    println!("usize: min = {}, max = {} (depends on architecture)", usize::MIN, usize::MAX);
    println!("isize: min = {}, max = {} (depends on architecture)", isize::MIN, isize::MAX);

    // Characters
    println!("\n=== Characters ===");
    let d: char = 'A';
    println!("char: value = {}", d);
    
    let e: char = '🦀';
    println!("char: value = {}", e);
    
    let f: char = 'Я';
    println!("char: value = {}", f);

    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());

}
