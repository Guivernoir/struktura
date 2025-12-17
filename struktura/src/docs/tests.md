# Struktura Test Suite

Comprehensive testing infrastructure for the beginner calculation system, covering both backend logic and API integration.

## 📋 Test Organization

### Unit Tests: `test_beginner_calculators.rs`

Tests individual calculator implementations for accuracy, edge cases, and warning generation.

**Modules Tested:**

- Garden Calculators (`PlanterBoxCalculator`, `MulchBedCalculator`)
- Interior Calculators (`WallFramingCalculator`, `DrywallCountCalculator`)
- Outdoor Calculators (`DeckCalculator`, `ConcreteSlabCalculator`)
- Utility Calculators (`PaintCoverageCalculator`, `TileCountCalculator`)
- Metadata Validation

### Integration Tests: `test_calculus_api.rs`

Tests API endpoints, request/response contracts, and error handling.

**Endpoints Tested:**

- `GET /catalogue` - Calculator discovery
- `POST /calculate` - Calculation execution
- HTTP method validation
- Content-Type validation
- Error response formats

## 🚀 Running Tests

### All Tests

```bash
cargo test
```

### Specific Test Suite

```bash
# Calculator logic tests
cargo test --test test_beginner_calculators

# API integration tests
cargo test --test test_calculus_api
```

### Specific Test

```bash
# By exact name
cargo test test_deck_basic_calculation

# By pattern
cargo test deck

# By module
cargo test garden_tests::
```

### With Output

```bash
# Show println! output
cargo test -- --nocapture

# Show test names as they run
cargo test -- --nocapture --test-threads=1
```

### Coverage-Focused

```bash
# Sequential execution for debugging
cargo test -- --test-threads=1

# Stop on first failure
cargo test -- --no-fail-fast
```

## 📊 Test Coverage

### Calculator Unit Tests

#### Garden Module

- ✅ Planter box volume calculations
- ✅ Soil mix ratios (60/30/10)
- ✅ Shallow depth warnings (<20cm)
- ✅ Deep depth warnings (>80cm)
- ✅ Large volume drainage warnings (>3m³)
- ✅ Mulch bed area calculations
- ✅ Landscape fabric overlap (10%)
- ✅ Thin mulch warnings (<5cm)
- ✅ Thick mulch warnings (>15cm)

#### Interiors Module

- ✅ Wall framing stud calculations
- ✅ Plate length (3x wall length)
- ✅ Tall wall warnings (>3.5m)
- ✅ Long wall warnings (>8m)
- ✅ Drywall sheet calculations
- ✅ Waste factor application (15%)
- ✅ Large project warnings (>100m²)

#### Outdoors Module

- ✅ Deck area calculations
- ✅ Joist spacing logic (40cm)
- ✅ Support post requirements (elevated)
- ✅ Large deck warnings (>40m²)
- ✅ Concrete volume with waste (8%)
- ✅ Rebar density calculations
- ✅ Thin slab warnings (<8cm)
- ✅ Thick slab warnings (>30cm)
- ✅ Large slab warnings (>50m²)

#### Utilities Module

- ✅ Paint coverage (walls + ceiling)
- ✅ Opening deductions (10%)
- ✅ Two-coat calculations
- ✅ Primer requirements
- ✅ Large project warnings (>150m²)
- ✅ Tile count with waste (12%)
- ✅ Tile size standardization (30cm)
- ✅ Large installation warnings (>80m²)
- ✅ Small area waste warnings

### API Integration Tests

#### Catalogue Endpoint

- ✅ Returns 200 OK
- ✅ Valid JSON structure
- ✅ Contains version field
- ✅ Contains 4 categories
- ✅ Contains 8 calculators
- ✅ Category IDs correct
- ✅ Metadata completeness
- ✅ GET-only validation

#### Calculate Endpoint

- ✅ All 8 calculator types
- ✅ Valid response structure
- ✅ Result item format (label/value/unit)
- ✅ Warning generation
- ✅ Invalid calculator type handling
- ✅ Missing field validation
- ✅ Malformed JSON handling
- ✅ Zero dimension handling
- ✅ Negative dimension handling
- ✅ Very large dimension handling
- ✅ Content-Type validation
- ✅ POST-only validation

#### Error Handling

- ✅ 400 Bad Request for invalid calculator
- ✅ 400 Bad Request for invalid JSON
- ✅ 415 Unsupported Media Type for wrong content-type
- ✅ 422 Unprocessable Entity for missing fields
- ✅ 405 Method Not Allowed for wrong HTTP method

### Metadata Validation

- ✅ All calculators have complete metadata
- ✅ ID uniqueness
- ✅ Non-empty required fields
- ✅ Valid typical ranges (ascending)

## 🎯 Test Patterns

### Calculation Accuracy

```rust
fn assert_approx_eq(actual: f64, expected: f64, tolerance: f64, label: &str) {
    let diff = (actual - expected).abs();
    assert!(diff <= tolerance, "{}: expected {:.2}, got {:.2}", label, expected, actual);
}
```

### Warning Detection

```rust
assert!(
    result.warnings.iter().any(|w| w.contains("expected text")),
    "Should warn about specific condition"
);
```

### API Response Validation

```rust
let json = parse_json_response(response).await;
assert!(json.get("field").is_some(), "Field should exist");
assert!(json["value"].is_number(), "Value should be numeric");
```

## 🔍 Debugging Failed Tests

### View Test Output

```bash
cargo test test_name -- --nocapture --show-output
```

### Run Single Test

```bash
cargo test test_deck_basic_calculation -- --exact
```

### Enable Rust Backtrace

```bash
RUST_BACKTRACE=1 cargo test
```

### Check Test Binary

```bash
# List all tests without running
cargo test -- --list

# Show ignored tests
cargo test -- --ignored --list
```

## 📝 Adding New Tests

### For New Calculator

1. Add unit tests in `test_beginner_calculators.rs`:

   ```rust
   #[test]
   fn new_calculator_basic_calculation() {
       let calc = NewCalculator;
       let dims = Dimensions { width: 1.0, length: 2.0, height: 0.5 };
       let result = calc.calculate(dims);
       // Assertions...
   }
   ```

2. Add to API integration test in `test_calculus_api.rs`:
   ```rust
   let calculator_types = vec![
       "existing_type",
       "new_calculator_type",  // Add here
   ];
   ```

### For New Endpoint

1. Create test in `test_calculus_api.rs`:
   ```rust
   #[tokio::test]
   async fn test_new_endpoint() {
       let app = create_test_router();
       let response = app.oneshot(Request::builder()
           .uri("/new-endpoint")
           .body(Body::empty())
           .unwrap())
           .await
           .unwrap();

       assert_eq!(response.status(), StatusCode::OK);
   }
   ```

## 🎖️ Test Quality Standards

### All Tests Must

- ✅ Have descriptive names (`test_deck_elevated_posts` not `test1`)
- ✅ Test one logical concept
- ✅ Be independent (no shared state)
- ✅ Be deterministic (same input = same output)
- ✅ Include failure messages
- ✅ Clean up resources (if applicable)

### Calculation Tests Must

- ✅ Verify mathematical accuracy
- ✅ Test edge cases (zero, negative, very large)
- ✅ Validate warning generation
- ✅ Check all result items

### API Tests Must

- ✅ Verify status codes
- ✅ Validate response structure
- ✅ Test error paths
- ✅ Validate HTTP method constraints
- ✅ Check content-type handling

## 🚨 Common Issues

### Test Fails Intermittently

- Likely a timing issue or floating-point precision
- Use `assert_approx_eq` for float comparisons
- Increase tolerance if needed

### Test Hangs

- Check for infinite loops
- Ensure async operations complete
- Add timeouts to integration tests

### Mock State Issues

- Ensure `create_test_state()` properly initializes
- Check database pool configuration
- Verify rate limiter setup

### Compilation Errors

- Check module imports in `tests/mod.rs`
- Verify test features in `Cargo.toml`
- Ensure `#[cfg(test)]` guards where needed

## 📚 Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Axum Testing Examples](https://github.com/tokio-rs/axum/tree/main/examples)

## 🎭 Test Philosophy

> "Tests are the specification. Code is the implementation."

These tests serve as:

1. **Documentation** - Demonstrate how calculators should behave
2. **Regression Protection** - Catch breaking changes
3. **Design Validation** - Verify API contracts
4. **Confidence** - Enable fearless refactoring

---

**Maintained by**: Struktura Engineering Team  
**Last Updated**: December 2025
**Test Coverage Target**: >85% for calculation logic, 100% for API endpoints
