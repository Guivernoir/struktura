use std::sync::Arc;
use pricing::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize with zero configuration
    let engine = init_pricing_engine().await?;
    
    println!("🎯 Pricing Engine initialized");
    println!("📡 Providers: {:?}", engine.list_providers().await);
    println!();
    
    // Define the operational theater
    let location = Location::new("BR")
        .with_region("SP")
        .with_city("Campinas")
        .with_coordinates(-22.9056, -47.0608);
    
    // Specify required materials
    let materials = vec![
        MaterialId::new(
            MaterialCategory::Concrete,
            "concrete_30mpa",
            "m3",
            "Concrete 30MPa"
        ),
        MaterialId::new(
            MaterialCategory::Lumber,
            "lumber_2x4_3m",
            "unit",
            "2x4 Lumber 3m"
        ),
        MaterialId::new(
            MaterialCategory::Rebar,
            "rebar_10mm",
            "m",
            "10mm Rebar"
        ),
    ];
    
    // Build the request
    let mut request = PriceRequest::new(location);
    for material in materials {
        request = request.add_material(material);
    }
    request = request
        .with_max_distance(25.0)  // 25km radius
        .with_currency(Currency::BRL);
    
    println!("🔍 Deploying reconnaissance units...\n");
    
    // Execute the mission
    let response = engine.fetch_prices(&request).await?;
    
    // Mission debrief
    println!("📊 RECONNAISSANCE REPORT");
    println!("========================\n");
    
    if response.prices.is_empty() {
        println!("⚠️  No prices found. Falling back to manual verification.");
    } else {
        println!("✅ Found {} prices from {} stores\n", 
            response.prices.len(),
            response.prices.iter()
                .map(|p| &p.store.name)
                .collect::<std::collections::HashSet<_>>()
                .len()
        );
        
        // Group by material
        for material in &request.materials {
            let prices = response.all_prices_for(material);
            if !prices.is_empty() {
                println!("📦 {}", material.description);
                println!("   Unit: {}", material.unit);
                
                for (i, price) in prices.iter().enumerate() {
                    println!("   {}. {} - {}{:.2}", 
                        i + 1,
                        price.store.name,
                        price.currency.symbol(),
                        price.price
                    );
                    
                    if let Some(ref addr) = price.store.address {
                        println!("      📍 {}", addr);
                    }
                    if let Some(dist) = price.store.distance_km {
                        println!("      🚗 {:.1}km away", dist);
                    }
                    if let Some(ref website) = price.store.website {
                        println!("      🌐 {}", website);
                    }
                }
                println!();
            }
        }
        
        // Tactical recommendations
        println!("🎯 TACTICAL RECOMMENDATIONS");
        println!("===========================\n");
        
        let one_stop = response.stores_with_all_materials();
        if !one_stop.is_empty() {
            println!("✨ One-stop shops (have all materials):");
            for store in one_stop {
                println!("   • {}", store);
            }
            println!();
        }
        
        // Best prices
        println!("💰 Best prices per material:");
        for material in &request.materials {
            if let Some(best) = response.best_price(material) {
                println!("   • {}: {}{:.2} at {}",
                    material.description,
                    best.currency.symbol(),
                    best.price,
                    best.store.name
                );
            }
        }
        println!();
    }
    
    // Warnings and intelligence gaps
    if !response.warnings.is_empty() {
        println!("⚠️  INTELLIGENCE GAPS");
        println!("===================\n");
        for warning in &response.warnings {
            println!("   • {}", warning);
        }
        println!();
    }
    
    // Unavailable materials
    if !response.unavailable.is_empty() {
        println!("❌ UNAVAILABLE MATERIALS");
        println!("======================\n");
        for material in &response.unavailable {
            println!("   • {}", material.description);
        }
        println!("\n💡 Recommendation: Try expanding search radius or checking specialized suppliers");
        println!();
    }
    
    println!("📅 Report generated: {}", response.searched_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("\n✅ Mission complete. Well, that was quite the strategic decision, wasn't it?");
    
    Ok(())
}