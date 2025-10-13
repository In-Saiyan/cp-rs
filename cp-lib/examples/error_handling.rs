use cp_lib::io::scanner::Scanner;

fn main() {
    println!("=== Error Handling Examples ===\n");

    // Example 1: Type mismatch error
    println!("1. Type mismatch error (trying to parse floats as integers):");
    let input1 = "3\n3.14 2.718 1.414".to_string();
    let mut scanner1 = Scanner::from_string(input1);
    let n1: usize = scanner1.next();
    
    // This will panic with a descriptive error message
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Vec<i32> = scanner1.dump(n1);
    })) {
        Ok(_) => println!("   No error occurred (unexpected)"),
        Err(_) => println!("   ✓ Caught expected panic for type mismatch"),
    }

    // Example 2: Insufficient input error
    println!("\n2. Insufficient input error (expecting 5 numbers but only 3 provided):");
    let input2 = "5\n1 2 3".to_string();
    let mut scanner2 = Scanner::from_string(input2);
    let n2: usize = scanner2.next();
    
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Vec<i32> = scanner2.dump(n2);
    })) {
        Ok(_) => println!("   No error occurred (unexpected)"),
        Err(_) => println!("   ✓ Caught expected panic for insufficient input"),
    }

    // Example 3: Invalid token error
    println!("\n3. Invalid token error (trying to parse 'abc' as integer):");
    let input3 = "3\n1 abc 3".to_string();
    let mut scanner3 = Scanner::from_string(input3);
    let n3: usize = scanner3.next();
    
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Vec<i32> = scanner3.dump(n3);
    })) {
        Ok(_) => println!("   No error occurred (unexpected)"),
        Err(_) => println!("   ✓ Caught expected panic for invalid token"),
    }

    println!("\n=== Successful Usage Examples ===\n");

    // Example 4: Successful usage
    println!("4. Successful dump operations:");
    let input4 = "3\n100 200 300\n2\nhello world".to_string();
    let mut scanner4 = Scanner::from_string(input4);
    
    let n4a: usize = scanner4.next();
    let numbers: Vec<i32> = scanner4.dump(n4a);
    println!("   Numbers: {:?}", numbers);
    
    let n4b: usize = scanner4.next();
    let words: Vec<String> = scanner4.dump(n4b);
    println!("   Words: {:?}", words);

    println!("\n✓ All examples completed successfully!");
}