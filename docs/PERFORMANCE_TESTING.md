# Performance Testing Guide

This guide covers how to test and benchmark performance for cp-rs features using hyperfine and other tools.

## Prerequisites

### Installing hyperfine
```bash
# On Ubuntu/Debian
sudo apt install hyperfine

# On macOS
brew install hyperfine

# On Arch Linux
sudo pacman -S hyperfine

# Using Cargo
cargo install hyperfine
```

## Performance Testing Workflow

### 1. Basic Performance Testing

For testing individual algorithms or functions:

```bash
# Create a simple benchmark binary
cat > benches/algorithm_test.rs << 'EOF'
use cp_lib::your_algorithm;

fn main() {
    let input_size = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    
    // Your performance test code here
    let start = std::time::Instant::now();
    your_algorithm(input_size);
    println!("Time: {:?}", start.elapsed());
}
EOF

# Compile with optimizations
cargo build --release --bin algorithm_test

# Benchmark with hyperfine
hyperfine --warmup 3 './target/release/algorithm_test 1000'
```

### 2. Comparing Implementations

When optimizing existing code:

```bash
# Test old vs new implementation
hyperfine --warmup 5 \
  './target/release/old_algorithm 10000' \
  './target/release/new_algorithm 10000'
```

### 3. Input Size Scaling

Test how performance scales with input size:

```bash
# Test multiple input sizes
hyperfine --warmup 3 \
  --parameter-scan size 1000 10000 1000 \
  './target/release/algorithm {size}'

# Or with exponential scaling
for size in 100 1000 10000 100000; do
  echo "Testing size: $size"
  hyperfine --warmup 2 "./target/release/algorithm $size"
done
```

### 4. Memory Usage Testing

For memory-intensive algorithms:

```bash
# Using time command for memory statistics
/usr/bin/time -v ./target/release/algorithm 10000

# Using valgrind (if available)
valgrind --tool=massif --stacks=yes ./target/release/algorithm 10000
```

## Competitive Programming Benchmarks

### Scanner Performance
```bash
# Create large test input
python3 -c "
import random
print(100000)  # number of test cases
for _ in range(100000):
    print(random.randint(1, 1000000))
" > large_input.txt

# Test scanner performance
hyperfine --warmup 2 \
  'cat large_input.txt | ./target/release/scanner_test'
```

### Algorithm Benchmarks

#### Graph Algorithms
```bash
# Generate graph input
python3 -c "
import random
n, m = 10000, 50000
print(n, m)
for _ in range(m):
    u, v = random.randint(1, n), random.randint(1, n)
    print(u, v)
" > graph_input.txt

hyperfine --warmup 2 \
  'cat graph_input.txt | ./target/release/graph_algorithm'
```

#### Sorting/Data Structure Tests
```bash
# Large array test
python3 -c "
import random
arr = [random.randint(1, 10**9) for _ in range(100000)]
print(len(arr))
print(*arr)
" > array_input.txt

hyperfine --warmup 3 \
  'cat array_input.txt | ./target/release/sorting_test'
```

## Performance Guidelines

### Time Limits
Competitive programming typically has these constraints:
- **1 second**: Standard time limit
- **2 seconds**: More computing intensive problems
- **0.5 seconds**: For easy problems with relatively small constraints

### Expected Performance
```bash
# Target performance for common operations (per second):
# - Simple operations: 10^7 - 10^8 (hmmm...)
# - Array operations:  > 10^7
# - Graph traversal: 10^6 - 10^7
# - Complex algorithms: 10^5 - 10^7
```

### Optimization Tips

1. **Compile with optimizations**:
   ```bash
   cargo build --release
   ```

2. **Profile before optimizing**:
   ```bash
   cargo install flamegraph
   cargo flamegraph --bin your_algorithm
   ```

3. **Use appropriate data structures**:
   - `Vec` for dynamic arrays
   - `VecDeque` for queues
   - `BinaryHeap` for priority queues
   - `HashMap`/`BTreeMap` for maps

## Bundled Code Performance

Test performance of bundled code vs development version:

```bash
# Bundle the code
cargo run --bin ast_bundler

# Compile bundled version
cd bundled
rustc -O solution_*.rs -o bundled_solution

# Compare performance
cd ..
hyperfine --warmup 3 \
  './target/release/main' \
  './bundled/bundled_solution'
```

## Regression Testing

Set up performance regression tests:

```bash
# Save baseline performance
hyperfine --export-json baseline.json \
  './target/release/algorithm 10000'

# After changes, compare
hyperfine --export-json current.json \
  './target/release/algorithm 10000'

# Analyze results (create script to compare JSONs)
```

## Continuous Integration Performance Testing

For automated performance monitoring:

```bash
# In CI script
hyperfine --export-json perf_results.json \
  --parameter-scan size 1000 10000 2000 \
  './target/release/algorithm {size}'

# Check if performance degraded significantly
# (implement threshold checking based on your needs)
```

## Common Performance Patterns

### Fast I/O
```rust
// Use Scanner for competitive programming I/O
let mut sc = Scanner::new();
let n: usize = sc.next();
let arr: Vec<i32> = sc.dump(n);  // Faster than individual reads
```

### Memory Pre-allocation
```rust
// Pre-allocate vectors when size is known
let mut result = Vec::with_capacity(n);
```

### Avoid Unnecessary Allocations
```rust
// Reuse buffers when possible
let mut buffer = Vec::new();
for _ in 0..test_cases {
    buffer.clear();
    // Use buffer...
}
```

Remember: Always test performance with realistic competitive programming inputs and constraints!