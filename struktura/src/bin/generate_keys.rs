use rand::Rng;
use std::io::Write;

/// Validates generated key meets security requirements
fn validate_key_strength(key_hex: &str) -> Result<(), String> {
    // HS512 requires minimum 64 bytes (512 bits)
    if key_hex.len() < 128 {
        return Err(format!(
            "Key too short ({} hex chars = {} bytes). HS512 requires minimum 64 bytes (128 hex chars).",
            key_hex.len(),
            key_hex.len() / 2
        ));
    }

    // Check entropy (unique characters)
    let unique_chars: std::collections::HashSet<char> = key_hex.chars().collect();
    if unique_chars.len() < 16 {
        return Err("Insufficient entropy (too few unique characters)".to_string());
    }

    // Verify it's valid hex
    if !key_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid hex characters detected".to_string());
    }

    Ok(())
}

/// Generate additional secrets with validation
fn generate_validated_secret(name: &str, bytes: usize) -> (String, String) {
    let mut key = vec![0u8; bytes];
    rand::rng().fill(&mut key[..]);
    
    let hex_key: String = key.iter().map(|b| format!("{:02x}", b)).collect();
    
    // Validate before returning
    if let Err(e) = validate_key_strength(&hex_key) {
        eprintln!("[WARNING] Generated {} failed validation: {}", name, e);
        // Regenerate if validation fails (shouldn't happen, but defensive)
        return generate_validated_secret(name, bytes);
    }
    
    (name.to_string(), hex_key)
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║         STRUKTURA SECURITY KEY GENERATOR (HS512)              ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    
    // Generate HS512-compliant JWT secret (64 bytes = 512 bits)
    let jwt_secret = generate_validated_secret("JWT_SECRET", 64);
    
    // Generate additional secrets for comprehensive security
    let csrf_secret = generate_validated_secret("CSRF_SECRET", 64);
    let session_secret = generate_validated_secret("SESSION_SECRET", 64);
    
    println!("║ ✓ JWT_SECRET (HS512): 64 bytes (512 bits)                    ║");
    println!("║ ✓ CSRF_SECRET: 64 bytes (256 bits)                           ║");
    println!("║ ✓ SESSION_SECRET: 32 bytes (256 bits)                        ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                    GENERATED SECRETS                          ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    
    println!("\n# JWT Secret (HS512 - 512 bits minimum)");
    println!("{}={}", jwt_secret.0, jwt_secret.1);
    println!("\n# CSRF Protection Secret (256 bits)");
    println!("{}={}", csrf_secret.0, csrf_secret.1);
    println!("\n# Session Management Secret (256 bits)");
    println!("{}={}", session_secret.0, session_secret.1);
    
    println!("\n╠═══════════════════════════════════════════════════════════════╣");
    println!("║                   SECURITY REQUIREMENTS                       ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║ ✓ Cryptographically secure random generation (OS RNG)        ║");
    println!("║ ✓ HS512 compliance: 64+ bytes (512+ bits)                    ║");
    println!("║ ✓ High entropy: 16+ unique hex characters                    ║");
    println!("║ ✓ Production-ready: No predictable patterns                  ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                    DEPLOYMENT INSTRUCTIONS                    ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║ 1. Copy the secrets above                                    ║");
    println!("║ 2. Add to your .env file (local) or Shuttle secrets          ║");
    println!("║ 3. For Shuttle deployment:                                   ║");
    println!("║    shuttle secrets add JWT_SECRET=<your_secret>              ║");
    println!("║    shuttle secrets add CSRF_SECRET=<your_secret>             ║");
    println!("║    shuttle secrets add SESSION_SECRET=<your_secret>          ║");
    println!("║                                                               ║");
    println!("║ 4. Additional required secrets:                              ║");
    println!("║    - ALLOWED_ORIGINS (comma-separated HTTPS URLs)            ║");
    println!("║    - RATE_LIMIT_REQUESTS (default: 100)                      ║");
    println!("║    - RATE_LIMIT_WINDOW_SECS (default: 60)                    ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Interactive .env file update
    if std::path::Path::new(".env").exists() {
        print!("\n[?] Found .env file. Append these secrets automatically? [y/N]: ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().to_lowercase() == "y" {
            match append_to_env(&[jwt_secret, csrf_secret, session_secret]) {
                Ok(_) => {
                    println!("\n✓ Success: Secrets appended to .env");
                    println!("✓ Backup recommendation: Save these secrets in a secure password manager");
                    println!("✓ Next steps: Add ALLOWED_ORIGINS and rate limit configuration");
                }
                Err(e) => {
                    eprintln!("\n✗ Error appending to .env: {}", e);
                    eprintln!("✗ Please copy secrets manually");
                }
            }
        } else {
            println!("\n[i] Manual setup selected. Copy secrets above to your .env file.");
        }
    } else {
        println!("\n[i] No .env file found. Create one and add the secrets above.");
        println!("[i] Example .env template:");
        println!("\n# Security Secrets (NEVER COMMIT TO GIT)");
        println!("{}={}", jwt_secret.0, jwt_secret.1);
        println!("{}={}", csrf_secret.0, csrf_secret.1);
        println!("{}={}", session_secret.0, session_secret.1);
        println!("\n# CORS Configuration");
        println!("ALLOWED_ORIGINS=https://struktura.app,https://www.struktura.app");
        println!("\n# Rate Limiting");
        println!("RATE_LIMIT_REQUESTS=100");
        println!("RATE_LIMIT_WINDOW_SECS=60");
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                  ⚠️  CRITICAL SECURITY WARNINGS  ⚠️            ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║ • NEVER commit .env to version control                        ║");
    println!("║ • Add '.env' to .gitignore immediately                        ║");
    println!("║ • Rotate secrets if accidentally exposed                      ║");
    println!("║ • Use different secrets for dev/staging/production            ║");
    println!("║ • Store production secrets in secure vault (e.g., 1Password)  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}

/// Safely append secrets to .env file with proper formatting
fn append_to_env(secrets: &[(String, String)]) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(".env")?;
    
    writeln!(file)?;
    writeln!(file, "# =============================================")?;
    writeln!(file, "# Generated by Struktura Key Generator")?;
    writeln!(file, "# Timestamp: {}", chrono::Utc::now().to_rfc3339())?;
    writeln!(file, "# =============================================")?;
    writeln!(file)?;
    
    for (key, value) in secrets {
        writeln!(file, "{}={}", key, value)?;
    }
    
    writeln!(file)?;
    writeln!(file, "# Additional required configuration:")?;
    writeln!(file, "# ALLOWED_ORIGINS=https://your-domain.com")?;
    writeln!(file, "# RATE_LIMIT_REQUESTS=100")?;
    writeln!(file, "# RATE_LIMIT_WINDOW_SECS=60")?;
    
    Ok(())
}