/*
* Author: Aryan Singh <aryan.singh.iiitl@gmail.com>, Cluaude Sonnet 4.0
* License: MIT
* Date: 2025-06-10
 */

#[cfg(test)]
mod tests {
    use cp_lib::io::scanner::Scanner;

    #[test]
    fn test_scanner_single_integer() {
        let input = "42".to_string();
        let mut scanner = Scanner::from_string(input);
        let result: i32 = scanner.next();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_scanner_multiple_integers() {
        let input = "10 20 30 40".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let a: i32 = scanner.next();
        let b: i32 = scanner.next();
        let c: i32 = scanner.next();
        let d: i32 = scanner.next();
        
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(c, 30);
        assert_eq!(d, 40);
    }

    #[test]
    fn test_scanner_string_parsing() {
        let input = "hello world".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let first: String = scanner.next();
        let second: String = scanner.next();
        
        assert_eq!(first, "hello");
        assert_eq!(second, "world");
    }

    #[test]
    fn test_scanner_floating_point() {
        let input = "3.14159 2.718".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let pi: f64 = scanner.next();
        let e: f64 = scanner.next();
        
        assert_eq!(pi, 3.14159);
        assert_eq!(e, 2.718);
    }

    #[test]
    fn test_scanner_negative_numbers() {
        let input = "-42 -100 -1".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let a: i32 = scanner.next();
        let b: i32 = scanner.next();
        let c: i32 = scanner.next();
        
        assert_eq!(a, -42);
        assert_eq!(b, -100);
        assert_eq!(c, -1);
    }

    #[test]
    fn test_scanner_large_numbers() {
        let input = "1000000000 9223372036854775807".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let large_int: i64 = scanner.next();
        let max_i64: i64 = scanner.next();
        
        assert_eq!(large_int, 1000000000i64);
        assert_eq!(max_i64, 9223372036854775807i64);
    }

    #[test]
    fn test_scanner_whitespace_handling() {
        let input = "  10   20\t30\n40  ".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let a: i32 = scanner.next();
        let b: i32 = scanner.next();
        let c: i32 = scanner.next();
        let d: i32 = scanner.next();
        
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(c, 30);
        assert_eq!(d, 40);
    }

    #[test]
    fn test_scanner_mixed_types() {
        let input = "Alice 25 3.14".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let name: String = scanner.next();
        let age: i32 = scanner.next();
        let score: f64 = scanner.next();
        
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
        assert_eq!(score, 3.14);
    }

    #[test]
    fn test_scanner_multiline_input() {
        let input = "\n\n42\n100 200\n\nhello\n".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let first: i32 = scanner.next();
        let second: i32 = scanner.next();
        let third: i32 = scanner.next();
        let word: String = scanner.next();
        
        assert_eq!(first, 42);
        assert_eq!(second, 100);
        assert_eq!(third, 200);
        assert_eq!(word, "hello");
    }

    #[test]
    fn test_scanner_sequential_reading() {
        let input = "1 2 3 4".to_string();
        let mut scanner = Scanner::from_string(input);
        
        // Test that Scanner reads tokens in correct order
        let a: i32 = scanner.next();
        let b: i32 = scanner.next();
        let c: i32 = scanner.next();
        let d: i32 = scanner.next();
        
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
        assert_eq!(d, 4);
    }

    #[test]
    fn test_scanner_competitive_programming_format() {
        let input = "5\n1 2 3 4 5\nhello world".to_string();
        let mut scanner = Scanner::from_string(input);
        
        // First line: count
        let n: i32 = scanner.next();
        assert_eq!(n, 5);
        
        // Second line: array of integers
        let mut numbers = Vec::new();
        for _ in 0..n {
            numbers.push(scanner.next::<i32>());
        }
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
        
        // Third line: strings
        let word1: String = scanner.next();
        let word2: String = scanner.next();
        assert_eq!(word1, "hello");
        assert_eq!(word2, "world");
    }

    #[test]
    #[should_panic]
    fn test_scanner_invalid_integer_parsing() {
        let input = "abc".to_string();
        let mut scanner = Scanner::from_string(input);
        let _: i32 = scanner.next(); // This should panic
    }

    #[test]
    #[should_panic]
    fn test_scanner_empty_input() {
        let input = "".to_string();
        let mut scanner = Scanner::from_string(input);
        let _: i32 = scanner.next(); // This should panic due to no input
    }

    #[test]
    fn test_scanner_various_integer_types() {
        let input = "127 32767 2147483647 9223372036854775807 1000".to_string();
        let mut scanner = Scanner::from_string(input);
        
        // Test i8
        let val_i8: i8 = scanner.next();
        assert_eq!(val_i8, 127i8);
        
        // Test i16
        let val_i16: i16 = scanner.next();
        assert_eq!(val_i16, 32767i16);
        
        // Test i32
        let val_i32: i32 = scanner.next();
        assert_eq!(val_i32, 2147483647i32);
        
        // Test i64
        let val_i64: i64 = scanner.next();
        assert_eq!(val_i64, 9223372036854775807i64);
        
        // Test usize (common in competitive programming)
        let val_usize: usize = scanner.next();
        assert_eq!(val_usize, 1000usize);
    }

    #[test]
    fn test_scanner_competitive_programming_patterns() {
        // Test common competitive programming input patterns
        let input = "3\n5 7\n1 3 5 7 9\n10 20".to_string();
        let mut scanner = Scanner::from_string(input);
        
        // Pattern 1: T test cases
        let t: i32 = scanner.next();
        assert_eq!(t, 3);
        
        // Pattern 2: N and M (dimensions)
        let n: i32 = scanner.next();
        let m: i32 = scanner.next();
        assert_eq!(n, 5);
        assert_eq!(m, 7);
        
        // Pattern 3: Array input
        let mut arr = Vec::new();
        for _ in 0..5 {
            arr.push(scanner.next::<i32>());
        }
        assert_eq!(arr, vec![1, 3, 5, 7, 9]);
        
        // Pattern 4: Coordinates
        let x: i32 = scanner.next();
        let y: i32 = scanner.next();
        assert_eq!(x, 10);
        assert_eq!(y, 20);
    }

    #[test]
    fn test_scanner_matrix_input() {
        let input = "3 3\n1 2 3\n4 5 6\n7 8 9".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let rows: usize = scanner.next();
        let cols: usize = scanner.next();
        
        let mut matrix = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(scanner.next::<i32>());
            }
            matrix.push(row);
        }
        
        assert_eq!(rows, 3);
        assert_eq!(cols, 3);
        assert_eq!(matrix, vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);
    }

    #[test]
    fn test_scanner_test_case_format() {
        let input = "2\n3\n1 2 3\n2\n10 20".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let t: i32 = scanner.next(); // Number of test cases
        assert_eq!(t, 2);
        
        // Test case 1
        let n1: i32 = scanner.next();
        assert_eq!(n1, 3);
        let mut case1 = Vec::new();
        for _ in 0..n1 {
            case1.push(scanner.next::<i32>());
        }
        assert_eq!(case1, vec![1, 2, 3]);
        
        // Test case 2
        let n2: i32 = scanner.next();
        assert_eq!(n2, 2);
        let mut case2 = Vec::new();
        for _ in 0..n2 {
            case2.push(scanner.next::<i32>());
        }
        assert_eq!(case2, vec![10, 20]);
    }

    #[test]
    fn test_scanner_dump_integers() {
        let input = "5\n1 2 3 4 5".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let numbers: Vec<i32> = scanner.dump(n);
        
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(numbers.len(), 5);
    }

    #[test]
    fn test_scanner_dump_floats() {
        let input = "3\n3.14 2.718 1.414".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let floats: Vec<f64> = scanner.dump(n);
        
        assert_eq!(floats, vec![3.14, 2.718, 1.414]);
    }

    #[test]
    fn test_scanner_dump_strings() {
        let input = "4\nhello world rust programming".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let words: Vec<String> = scanner.dump(n);
        
        assert_eq!(words, vec!["hello", "world", "rust", "programming"]);
    }

    #[test]
    fn test_scanner_dump_multiline() {
        let input = "6\n10 20\n30 40\n50 60".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let numbers: Vec<i32> = scanner.dump(n);
        
        assert_eq!(numbers, vec![10, 20, 30, 40, 50, 60]);
    }

    #[test]
    fn test_scanner_dump_zero_elements() {
        let input = "0\n".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let numbers: Vec<i32> = scanner.dump(n);
        
        assert_eq!(numbers, Vec::<i32>::new());
        assert_eq!(numbers.len(), 0);
    }

    #[test]
    fn test_scanner_dump_competitive_programming_pattern() {
        let input = "2\n3\n100 200 300\n4\n10 20 30 40".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let t: usize = scanner.next(); // Number of test cases
        
        for _ in 0..t {
            let n: usize = scanner.next();
            let arr: Vec<i32> = scanner.dump(n);
            
            if n == 3 {
                assert_eq!(arr, vec![100, 200, 300]);
            } else if n == 4 {
                assert_eq!(arr, vec![10, 20, 30, 40]);
            }
        }
    }

    #[test]
    #[should_panic(expected = "Failed to parse 'abc' as type i32")]
    fn test_scanner_dump_invalid_type_parsing() {
        let input = "3\n1 abc 3".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let _: Vec<i32> = scanner.dump(n); // Should panic on 'abc'
    }

    #[test]
    #[should_panic(expected = "Unexpected end of input while reading entry")]
    fn test_scanner_dump_insufficient_input() {
        let input = "5\n1 2 3".to_string(); // Only 3 numbers but expecting 5
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let _: Vec<i32> = scanner.dump(n); // Should panic due to insufficient input
    }

    #[test]
    #[should_panic(expected = "Failed to parse '3.14' as type i32")]
    fn test_scanner_dump_wrong_type() {
        let input = "2\n3.14 2.718".to_string();
        let mut scanner = Scanner::from_string(input);
        
        let n: usize = scanner.next();
        let _: Vec<i32> = scanner.dump(n); // Should panic trying to parse floats as integers
    }

    #[test]
    fn test_scanner_dump_mixed_usage() {
        let input = "Alice 25\n3\n10 20 30\nhello".to_string();
        let mut scanner = Scanner::from_string(input);
        
        // Read individual values first
        let name: String = scanner.next();
        let age: i32 = scanner.next();
        
        // Then dump an array
        let n: usize = scanner.next();
        let numbers: Vec<i32> = scanner.dump(n);
        
        // Then read another value
        let greeting: String = scanner.next();
        
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
        assert_eq!(numbers, vec![10, 20, 30]);
        assert_eq!(greeting, "hello");
    }
}