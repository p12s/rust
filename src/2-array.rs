fn main() {
    // Practical examples of usize/isize usage
    println!("\n=== Practical usize/isize examples ===");

    // usize for array indexing and sizes
    let numbers = vec![10, 20, 30, 40, 50];
    let length: usize = numbers.len();
    let index: usize = 2;
    println!("Vector length: {}", length);
    println!("Element at index {}: {}", index, numbers[index]);
    
    // usize for memory sizes
    use std::mem;
    let size_of_i32: usize = mem::size_of::<i32>();
    let size_of_vec: usize = mem::size_of_val(&numbers);
    println!("Size of i32: {} bytes", size_of_i32);
    println!("Size of vector: {} bytes", size_of_vec);
    
    // isize for pointer arithmetic and differences
    let start_pos: usize = 10;
    let end_pos: usize = 3;
    let difference: isize = end_pos as isize - start_pos as isize;
    println!("Position difference: {}", difference);
}