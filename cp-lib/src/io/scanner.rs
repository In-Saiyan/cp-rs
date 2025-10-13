use std::io::{self, BufRead, Cursor};

pub struct Scanner<R: BufRead> {
    buffer: Vec<String>,
    reader: R,
}

impl Scanner<io::StdinLock<'static>> {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            reader: Box::leak(Box::new(io::stdin())).lock(), // I don't like this but it works :/
        }
    }
}

impl<R: BufRead> Scanner<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            buffer: Vec::new(),
            reader,
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().unwrap();
            }
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => panic!("Unexpected end of input"),
                Ok(_) => {
                    self.buffer = line.split_whitespace().rev().map(String::from).collect();
                }
                Err(e) => panic!("Error reading input: {}", e),
            }
        }
    }

    pub fn dump<T: std::str::FromStr + std::fmt::Debug>(&mut self, n: usize) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            // Get the next token as a string first for better error handling
            let token = loop {
                if let Some(token) = self.buffer.pop() {
                    break token;
                }
                let mut line = String::new();
                match self.reader.read_line(&mut line) {
                    Ok(0) => panic!(
                        "Unexpected end of input while reading entry {} of {} for dump operation", 
                        i + 1, n
                    ),
                    Ok(_) => {
                        self.buffer = line.split_whitespace().rev().map(String::from).collect();
                    }
                    Err(e) => panic!("Error reading input: {}", e),
                }
            };

            // Try to parse the token
            match token.parse::<T>() {
                Ok(value) => result.push(value),
                Err(e) => panic!(
                    "Failed to parse '{}' as type {} at position {} of {}: {:?}",
                    token,
                    std::any::type_name::<T>(),
                    i + 1,
                    n,
                    e
                ),
            }
        }
        result
    }
}

impl Scanner<Cursor<Vec<u8>>> {
    pub fn from_string(input: String) -> Self {
        Scanner::from_reader(Cursor::new(input.into_bytes()))
    }
}
