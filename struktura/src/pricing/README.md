# Location-Based Pricing Engine - DuckDuckGo Edition

_Mission-critical pricing intel for civil engineering calculations. Now 100% API-free._

## Strategic Advantage

**Well, that was quite the strategic decision, wasn't it?** We've eliminated all external API dependencies.
No more:

- ❌ API keys to lose
- ❌ Rate limits to hit
- ❌ Billing surprises at month-end
- ❌ Corporate tracking of your queries
- ❌ Service outages when APIs go down

Instead, you get:

- ✅ DuckDuckGo web reconnaissance (free, unlimited)
- ✅ Intelligent fallback to static data
- ✅ Zero configuration required
- ✅ Works anywhere with internet
- ✅ Privacy-first approach

## How It Works

### DuckDuckGoProvider (Primary Intelligence)

The DuckDuckGo provider performs tactical web reconnaissance:

1. **Query Construction**: Builds search queries from material + location

   - Example: "Concrete 30MPa Campinas SP Brazil hardware store price buy"

2. **Search Execution**: Queries DuckDuckGo's HTML interface

   - No JavaScript required
   - No authentication needed
   - Standard HTTP requests only

3. **Intelligence Extraction**: Parses search results for:

   - Store names and addresses
   - Prices (with currency detection)
   - Phone numbers
   - Website URLs

4. **Deep Reconnaissance**: If initial search fails:

   - Falls back to known store chains
   - Uses estimated prices based on region and category
   - Provides at least approximate intel

5. **Caching**: Temporarily caches results to avoid redundant queries
   - Per material + location combination
   - In-memory only (no persistent storage)

### StaticProvider (Emergency Backup)

When DuckDuckGo is unavailable or returns no results:

- Provides hardcoded prices for major chains
- Covers BR (Leroy Merlin, Telhanorte) and US (Home Depot, Lowe's)
- Guarantees you always get some data

## Dependencies

**Required Crates:**

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
reqwest = { version = "0.11", features = ["json"] }
futures = "0.3"

# Web scraping
scraper = "0.17"
regex = "1.10"
urlencoding = "2.1"
```

**Optional:**

```toml
# For testing/examples
anyhow = "1.0"
```

## Basic Usage

```rust
use pricing::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Zero configuration required
    let engine = init_pricing_engine().await?;

    let location = Location::new("BR")
        .with_region("SP")
        .with_city("Campinas");

    let concrete = MaterialId::new(
        MaterialCategory::Concrete,
        "concrete_30mpa",
        "m3",
        "Concrete 30MPa"
    );

    let request = PriceRequest::new(location)
        .add_material(concrete)
        .with_currency(Currency::BRL);

    let response = engine.fetch_prices(&request).await?;

    // Tactical debrief
    for price in &response.prices {
        println!("{} at {} - {}{}",
            price.material.description,
            price.store.name,
            price.currency.symbol(),
            price.price
        );

        if let Some(ref website) = price.store.website {
            println!("  Intel source: {}", website);
        }
    }

    Ok(())
}
```

## Production Deployment

### Environment Variables

**None required.** That's the point.

### Optional Configuration

```rust
// If you want to customize DuckDuckGo behavior
let ddg = DuckDuckGoProvider::new();
// ... configure if needed ...

let engine = PricingEngine::new();
engine.register_provider(Arc::new(ddg)).await;
```

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/pricing_server /usr/local/bin/
CMD ["pricing_server"]
```

No environment variables. No secrets. Just deploy.

## Performance Notes

- **DuckDuckGo queries**: 500-1500ms (depends on network)
- **Static fallback**: <1ms
- **Caching**: Speeds up repeated queries significantly
- **Parallel execution**: Multiple materials queried simultaneously

## Limitations & Workarounds

### Current Limitations

1. **Price Accuracy**: Web scraping isn't perfect

   - Prices may be approximate or outdated
   - Some stores don't publish prices publicly
   - **Workaround**: Always verify prices before purchasing

2. **Store Locations**: GPS coordinates not always available

   - Distance calculations may be estimates
   - **Workaround**: Use city/region for filtering

3. **Availability**: Can't determine real-time stock

   - Assumes items are available
   - **Workaround**: Call stores to confirm before traveling

4. **DuckDuckGo Blocks**: Excessive queries might get throttled
   - Caching helps reduce requests
   - **Workaround**: Add delays between searches if needed

### Future Improvements

- [ ] Add more sophisticated HTML parsing
- [ ] Implement rotating user agents
- [ ] Add store-specific scrapers for major chains
- [ ] Integrate with public price databases
- [ ] Add proxy support for enterprise deployment

## Tactical Advantages

**Privacy**: DuckDuckGo doesn't track your searches. Your construction projects remain confidential.

**Reliability**: No single point of failure. If DuckDuckGo fails, static data provides backup.

**Cost**: Zero. Free. Gratis. Nada. No billing surprises at month-end.

**Maintenance**: No API keys to rotate, no rate limits to monitor, no service agreements to manage.

**Scalability**: Limited only by your network bandwidth and DuckDuckGo's patience.

## When NOT to Use This

- **Mission-critical applications**: Where price accuracy is life-or-death
- **High-frequency queries**: Thousands of requests per minute
- **Legal requirements**: If you need contractual SLAs
- **Real-time inventory**: This can't check actual stock levels

For these scenarios, consider:

- Direct integration with supplier APIs
- Professional construction estimation software
- Manual verification processes

## Testing

```bash
# Unit tests (uses mock data)
cargo test --package pricing

# Integration tests (actually queries DuckDuckGo)
cargo test --package pricing --test integration -- --nocapture

# Performance test
cargo test --package pricing test_duckduckgo_provider -- --nocapture
```

## Troubleshooting

**Problem**: No prices returned
**Solution**: Check internet connection, verify location is specific enough (include city)

**Problem**: Prices seem inaccurate
**Solution**: Web scraping isn't perfect. Verify prices before purchasing.

**Problem**: Slow queries
**Solution**: Be more specific with location. Cache results if querying repeatedly.

**Problem**: DuckDuckGo blocking requests
**Solution**: Add small delays between queries, implement backoff strategy.

## Contributing

Want to add more providers or improve scraping?

```rust
pub struct YourCustomProvider {
    // Your implementation
}

#[async_trait]
impl PriceProvider for YourCustomProvider {
    fn name(&self) -> &str { "your_provider" }

    fn supports_location(&self, location: &Location) -> bool {
        // Your logic
    }

    async fn fetch_prices(&self, request: &PriceRequest)
        -> PricingResult<PriceResponse>
    {
        // Your scraping magic
    }
}
```

Then register it:

```rust
let engine = PricingEngine::new();
engine.register_provider(Arc::new(YourCustomProvider::new())).await;
```

---

_"The best intelligence is the intelligence you don't have to pay for."_  
_- Every quartermaster ever_

// Summary of changes:
// 1. ✅ Added DuckDuckGoProvider as primary reconnaissance tool
// 2. ✅ Removed dependency on Google Places API
// 3. ✅ Eliminated need for API keys and rate limiting
// 4. ✅ Maintained StaticProvider as emergency fallback
// 5. ✅ Added intelligent caching to DuckDuckGo queries
// 6. ✅ Implemented price extraction with currency detection
// 7. ✅ Added store information parsing from search results
// 8. ✅ Updated registry to use DuckDuckGo by default
// 9. ✅ Maintained backward compatibility with existing traits
// 10. ✅ Added comprehensive documentation with tactical flair
//
// Strategic advantages:
// - Zero external costs (no API billing)
// - No rate limiting (reasonable use)
// - Privacy-first (DuckDuckGo doesn't track)
// - No configuration required
// - Works anywhere with internet
// - Intelligent fallback mechanisms
//
// Trade-offs accepted:
// - Slightly less accurate than direct API integration
// - Depends on web scraping (may break if sites change)
// - Can't verify real-time inventory
// - Network latency higher than cached API responses
//
// Deployment status: READY FOR FIELD OPERATIONS
// Maintenance burden: MINIMAL
// Cost: ZERO
// Coolness factor: MAXIMUM
//
// "In the end, the best reconnaissance is the one that actually gets done.
// And the best price is the one you don't have to pay an API vendor to find."
// - Sun Tzu, "The Art of Construction Project Management"
//
// Well, that was quite the strategic decision, wasn't it? 🎯
