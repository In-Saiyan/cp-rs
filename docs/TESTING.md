# Testing Guidelines

Comprehensive guide for testing in the cp-rs project, covering unit tests, integration tests, and competitive programming specific testing patterns.

## Testing Philosophy

All code in cp-rs must be thoroughly tested because:
- **Reliability**: Competitive programming requires correct solutions
- **Bundler Compatibility**: Features must work in single-file bundled output  
- **Performance**: Code must meet strict time/memory constraints
- **Maintainability**: Tests prevent regressions when optimizing

## Test Structure

### Directory Organization
```
cp-lib/
├── src/           # Implementation
│   ├── lib.rs
│   └── io/
│       └── scanner.rs
└── tests/         # Test units (mirrors src structure)
    ├── mod.rs
    └── io/
        └── scanner.rs
```

### Test File Pattern
Each module in `src/` should have corresponding tests in `tests/`:
- `src/algorithms/graph.rs` → `tests/algorithms/graph.rs`
- `src/data_structures/segment_tree.rs` → `tests/data_structures/segment_tree.rs`

## Writing Tests

### Basic Test Structure
```rust
// cp-lib/tests/algorithms/example.rs
use cp_lib::algorithms::example::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = vec![1, 2, 3, 4, 5];
        let expected = 15;
        
        // Act
        let result = sum_array(&input);
        
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_cases() {
        // Empty input
        assert_eq!(sum_array(&[]), 0);
        
        // Single element
        assert_eq!(sum_array(&[42]), 42);
        
        // Negative numbers
        assert_eq!(sum_array(&[-1, -2, -3]), -6);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_error_conditions() {
        // Test that function properly handles overflow
        sum_array(&[i32::MAX, 1]);
    }
}
```

### Competitive Programming Test Patterns

#### 1. Sample Input/Output Testing
```rust
#[test]
fn test_sample_cases() {
    // Test with actual competitive programming sample
    let input = "3\n1 2 3";
    let mut scanner = Scanner::from_string(input.to_string());
    
    let n: usize = scanner.next();
    let arr: Vec<i32> = scanner.dump(n);
    
    let result = solve(&arr);
    assert_eq!(result, 6); // Expected output
}
```

#### 2. Large Input Testing
```rust
#[test]
fn test_large_input() {
    // Test with competitive programming constraints
    let n = 100_000;
    let large_input: Vec<i32> = (1..=n).collect();
    
    let start = std::time::Instant::now();
    let result = solve(&large_input);
    let duration = start.elapsed();
    
    // Ensure it completes within time limit
    assert!(duration.as_millis() < 1000, "Algorithm too slow: {:?}", duration);
    assert_eq!(result, n * (n + 1) / 2); // Expected mathematical result
}
```

#### 3. Scanner Comprehensive Testing
```rust
#[test]
fn test_scanner_mixed_input() {
    let input = "5 hello 3.14 true -42\n10 20\n";
    let mut scanner = Scanner::from_string(input.to_string());
    
    assert_eq!(scanner.next::<i32>(), 5);
    assert_eq!(scanner.next::<String>(), "hello");
    assert_eq!(scanner.next::<f64>(), 3.14);
    assert_eq!(scanner.next::<bool>(), true);
    assert_eq!(scanner.next::<i32>(), -42);
    
    let arr: Vec<i32> = scanner.dump(2);
    assert_eq!(arr, vec![10, 20]);
}
```

## Test Categories

### 1. Unit Tests
Test individual functions and methods:
```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(17, 13), 1);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
}
```

### 2. Integration Tests  
Test how components work together:
```rust
#[test]
fn test_graph_algorithm_integration() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, 10);
    graph.add_edge(1, 2, 5);
    
    let distances = dijkstra(&graph, 0);
    assert_eq!(distances[2], 15);
}
```

### 3. Property-Based Tests
Test algorithmic properties:
```rust
#[test]
fn test_sorting_properties() {
    let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let original_len = arr.len();
    
    custom_sort(&mut arr);
    
    // Length preserved
    assert_eq!(arr.len(), original_len);
    
    // Actually sorted
    for i in 1..arr.len() {
        assert!(arr[i-1] <= arr[i]);
    }
}
```

### 4. Performance Tests
```rust
#[test]
fn test_performance_constraints() {
    let n = 10_000;
    let data = generate_test_data(n);
    
    let start = std::time::Instant::now();
    let _result = expensive_algorithm(&data);
    let elapsed = start.elapsed();
    
    // Should complete within 100ms for this input size
    assert!(elapsed.as_millis() < 100, 
           "Too slow: {}ms for n={}", elapsed.as_millis(), n);
}
```

## Test Data Generation

### Random Test Data
```rust
use rand::Rng;

fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(1..=1000)).collect()
}

#[test] 
fn test_with_random_data() {
    for _ in 0..100 {  // Run 100 random tests
        let arr = generate_random_array(1000);
        let result = your_algorithm(&arr);
        // Test invariants that should always hold
        assert!(result >= 0);
    }
}
```

### Competitive Programming Test Cases
```rust
fn create_competitive_test_case() -> String {
    // Generate input in competitive programming format
    format!("5\n1 2 3 4 5\n3\n10 20 30\n")
}

#[test]
fn test_competitive_format() {
    let input = create_competitive_test_case();
    let mut scanner = Scanner::from_string(input);
    
    // Test parsing competitive programming input
    let n1: usize = scanner.next();
    let arr1: Vec<i32> = scanner.dump(n1);
    
    let n2: usize = scanner.next(); 
    let arr2: Vec<i32> = scanner.dump(n2);
    
    assert_eq!(n1, 5);
    assert_eq!(arr1, vec![1, 2, 3, 4, 5]);
    assert_eq!(n2, 3);
    assert_eq!(arr2, vec![10, 20, 30]);
}
```

## Running Tests

### Basic Test Execution
```bash
# Run all tests
cd cp-lib && cargo test

# Run specific test module
cargo test scanner

# Run specific test function
cargo test test_scanner_dump

# Run with output
cargo test -- --nocapture

# Run ignored tests (for slow performance tests)
cargo test -- --ignored
```

### Performance Testing
```bash
# Run tests in release mode for accurate performance measurement
cargo test --release

# Run with timing information
cargo test -- --show-output

# Profile test execution
cargo test --profile test-opt
```

### Integration with Bundler Testing
```bash
# Test that bundled code works
cd cp-lib && cargo test
cd .. && cargo run --bin ast_bundler
cd bundled && rustc solution_*.rs && echo "test input" | ./solution_*
```

## Test Organization Best Practices

### 1. Test Naming
- Use descriptive names: `test_scanner_handles_empty_input`
- Group related tests: `test_graph_bfs_*`, `test_graph_dfs_*`
- Include edge cases: `test_algorithm_with_zero_input`

### 2. Test Independence  
- Each test should be independent
- Use `setup()` and `teardown()` when needed
- Don't rely on test execution order

### 3. Comprehensive Coverage
For each function, test:
- **Happy path**: Normal expected usage
- **Edge cases**: Boundary conditions, empty inputs
- **Error conditions**: Invalid inputs, overflow
- **Performance**: Large inputs, time constraints

### 4. Documentation
```rust
/// Tests the segment tree range query functionality
/// 
/// Verifies that range queries return correct sums for:
/// - Single element ranges
/// - Full array ranges  
/// - Arbitrary subranges
/// - Edge cases (empty ranges, out of bounds)
#[test]
fn test_segment_tree_range_query() {
    // Test implementation
}
```

## Continuous Integration

### GitHub Actions Test Configuration
```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Run tests
      run: cd cp-lib && cargo test
      
    - name: Test bundler
      run: cargo run --bin ast_bundler
      
    - name: Verify bundled code compiles
      run: cd bundled && rustc solution_*.rs
```

## Common Testing Patterns

### Testing Scanner Input Parsing
```rust
#[test]
fn test_multiline_input() {
    let input = "3\n1 2 3\n2\n4 5\n";
    let mut scanner = Scanner::from_string(input.to_string());
    
    let n1: usize = scanner.next();
    let arr1: Vec<i32> = scanner.dump(n1);
    let n2: usize = scanner.next(); 
    let arr2: Vec<i32> = scanner.dump(n2);
    
    assert_eq!((n1, arr1, n2, arr2), (3, vec![1,2,3], 2, vec![4,5]));
}
```

### Testing Graph Algorithms
```rust
#[test]
fn test_shortest_path() {
    let mut graph = vec![vec![]; 4];
    graph[0].push((1, 1)); // edge 0->1 with weight 1
    graph[1].push((2, 2)); // edge 1->2 with weight 2
    graph[0].push((2, 5)); // direct edge 0->2 with weight 5
    
    let distances = dijkstra(&graph, 0);
    assert_eq!(distances[2], 3); // Should use path 0->1->2 (cost 3) not 0->2 (cost 5)
}
```

Remember: Good tests are as important as good algorithms in competitive programming!