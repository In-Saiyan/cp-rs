use cp_lib::io::scanner::Scanner;

fn main() {
    // Example usage of Scanner with custom string input
    let input = "3\n10 20 30\n2\nhello world\n3\n3.14 2.718 1.414\n4\nAlice 25 Bob 30".to_string();
    let mut scanner = Scanner::from_string(input);
    
    // Example 1: Using dump for integers
    let n1: usize = scanner.next();
    let integers: Vec<i32> = scanner.dump(n1);
    println!("Integers (using dump): {:?}", integers);
    
    // Example 2: Using dump for strings
    let n2: usize = scanner.next();
    let words: Vec<String> = scanner.dump(n2);
    println!("Strings (using dump): {:?}", words);
    
    // Example 3: Using dump for floats
    let n3: usize = scanner.next();
    let floats: Vec<f64> = scanner.dump(n3);
    println!("Floats (using dump): {:?}", floats);
    
    // Example 4: Mixed usage - individual reads and dump
    let n4: usize = scanner.next();
    println!("Reading {} mixed values individually:", n4);
    for i in 0..n4 {
        if i % 2 == 0 {
            let name: String = scanner.next();
            println!("  Name {}: {}", i/2 + 1, name);
        } else {
            let age: i32 = scanner.next();
            println!("  Age {}: {}", i/2 + 1, age);
        }
    }
    
    println!("\n--- Competitive Programming Example ---");
    
    // Typical competitive programming pattern
    let cp_input = "3\n5\n1 2 3 4 5\n3\n10 20 30\n4\n100 200 300 400".to_string();
    let mut cp_scanner = Scanner::from_string(cp_input);
    
    let test_cases: usize = cp_scanner.next();
    println!("Number of test cases: {}", test_cases);
    
    for case in 1..=test_cases {
        let array_size: usize = cp_scanner.next();
        let array: Vec<i32> = cp_scanner.dump(array_size);
        let sum: i32 = array.iter().sum();
        println!("Test case {}: array = {:?}, sum = {}", case, array, sum);
    }
}