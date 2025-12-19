/*
* Author: Aryan Singh <aryan.singh.iiitl@gmail.com>
* License: MIT
* Date: 2025-06-10
*/

use cp_lib::io::scanner::Scanner;

const _PROBLEM: &str = "F1. Tree Cutting (Easy Version)";


fn main() {
    let mut sc = Scanner::new();
    let t: i32 = sc.next();

    for _ in 0..t {
        let n: usize = sc.next();
        let s: String = sc.next();
        let arr: Vec<bool> = s.chars().map(|c| c == '1').collect();

        let first = arr.iter().position(|&x| x);
        let last = arr.iter().rposition(|&x| x);

        let count = match (first, last) {
            (Some(f), Some(l)) if f < l => {
                arr[f+1..l].iter().filter(|&&x| !x).count()
            }
            _ => 0,
        };

        println!("{}", count);
    }
}


