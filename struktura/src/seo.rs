use axum::{
    response::{Html, IntoResponse},
    extract::Query,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LangQuery {
    pub lang: Option<String>,
}

struct SeoMetadata {
    title: &'static str,
    desc: &'static str,
    og_locale: &'static str,
}

pub async fn index_handler(Query(params): Query<LangQuery>) -> impl IntoResponse {
    // Ideally, load this once at startup in main.rs and pass via State to avoid I/O on every request, 
    // but for now, we keep it here.
    let raw_html = include_str!("../static/dist/index.html");
    let lang_code = params.lang.as_deref().unwrap_or("en");
    let base_url = "https://struktura.fly.dev";

    // CSO Strategy: High-Impact Copywriting Map
    // We use "Zero Waste" and "Precision" as universal hooks for engineers.
    let meta = match lang_code {
        "pt" => SeoMetadata {
            title: "Struktura | Calculadora de Engenharia & Precisão Civil",
            desc: "Elimine o desperdício. Calcule estruturas com precisão absoluta. A ferramenta definitiva para engenheiros civis e empreiteiros modernos.",
            og_locale: "pt_BR",
        },
        "fr" => SeoMetadata {
            title: "Struktura | Calculateur de Précision Génie Civil",
            desc: "Construisez sans erreur. Zéro déchet. La suite d'outils essentielle pour l'ingénierie structurelle et le calcul de matériaux.",
            og_locale: "fr_FR",
        },
        "es" => SeoMetadata {
            title: "Struktura | Calculadora de Precisión Estructural",
            desc: "Ingeniería sin desperdicios. Cálculos exactos para obras civiles. La plataforma definitiva para el ingeniero moderno.",
            og_locale: "es_ES",
        },
        "de" => SeoMetadata {
            title: "Struktura | Präzisionsrechner für Bauingenieurwesen",
            desc: "Bauen Sie mit absoluter Präzision. Keine Verschwendung. Das professionelle Werkzeug für Struktur- und Materialberechnungen.",
            og_locale: "de_DE",
        },
        "it" => SeoMetadata {
            title: "Struktura | Calcolatore di Precisione Ingegneristica",
            desc: "Costruisci con rigore matematico. Zero sprechi. Lo standard definitivo per l'ingegneria civile e strutturale.",
            og_locale: "it_IT",
        },
        "ru" => SeoMetadata {
            title: "Struktura | Инженерный Калькулятор Высокой Точности",
            desc: "Стройте с математической точностью. Нулевые отходы. Профессиональный инструмент для расчета конструкций и материалов.",
            og_locale: "ru_RU",
        },
        _ => SeoMetadata { // Default to EN
            title: "Struktura | The Zero-Waste Structural Calculator",
            desc: "Build with absolute precision. Waste nothing. The ultimate engineering toolkit for material estimates and structural integrity.",
            og_locale: "en_US",
        },
    };

    // 1. JSON-LD Structured Data (The "Secret Weapon" for Google Rich Snippets)
    let json_ld = format!(
        r#"<script type="application/ld+json">
        {{
            "@context": "https://schema.org",
            "@type": "SoftwareApplication",
            "name": "Struktura",
            "applicationCategory": "DesignApplication",
            "operatingSystem": "Web Browser",
            "offers": {{
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "USD"
            }},
            "description": "{}"
        }}
        </script>"#, 
        meta.desc
    );

    // 2. Hreflang Tags (Critical for targeting specific regions without penalty)
    let hreflang_tags = format!(
        r#"
        <link rel="alternate" hreflang="en" href="{base}/?lang=en" />
        <link rel="alternate" hreflang="pt" href="{base}/?lang=pt" />
        <link rel="alternate" hreflang="fr" href="{base}/?lang=fr" />
        <link rel="alternate" hreflang="es" href="{base}/?lang=es" />
        <link rel="alternate" hreflang="de" href="{base}/?lang=de" />
        <link rel="alternate" hreflang="it" href="{base}/?lang=it" />
        <link rel="alternate" hreflang="ru" href="{base}/?lang=ru" />
        <link rel="alternate" hreflang="x-default" href="{base}/?lang=en" />
        "#,
        base = base_url
    );

    // 3. Social Graph & Meta Tags Injection
    // We construct a 'head_block' to inject everything at once, cleaner than multiple replaces.
    let head_injection = format!(
        r#"
        {hreflang}
        <meta name="description" content="{desc}">
        <link rel="canonical" href="{base}/?lang={lang}" />
        
        <meta property="og:type" content="website" />
        <meta property="og:url" content="{base}/?lang={lang}" />
        <meta property="og:title" content="{title}" />
        <meta property="og:description" content="{desc}" />
        <meta property="og:locale" content="{og_locale}" />
        <meta property="og:site_name" content="Struktura" />
        
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="{title}" />
        <meta name="twitter:description" content="{desc}" />
        
        {json_ld}
        "#,
        hreflang = hreflang_tags,
        desc = meta.desc,
        base = base_url,
        lang = lang_code,
        title = meta.title,
        og_locale = meta.og_locale,
        json_ld = json_ld
    );

    // 4. HTML Injection
    let modified_html = raw_html
        .replace("<html lang=\"en\">", &format!("<html lang=\"{}\">", lang_code))
        .replace("<title>Struktura</title>", &format!("<title>{}</title>", meta.title)) // Matches default Vite/React title usually
        .replace("<title>Vite App</title>", &format!("<title>{}</title>", meta.title)) // Common default
        .replace("</head>", &format!("{}</head>", head_injection)); // Inject just before head closes

    Html(modified_html)
}

pub async fn sitemap_handler() -> impl IntoResponse {
    let base_url = "https://struktura.fly.dev"; 
    let langs = vec!["en", "pt", "fr", "es", "de", "it", "ru"];
    
    // We use a dedicated buffer for performance
    let mut xml = String::with_capacity(2000);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n");
    
    // Generate main entries with localized alternates inside them (Google best practice)
    for lang in langs.iter() {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}/?lang={}</loc>\n", base_url, lang));
        xml.push_str("    <changefreq>daily</changefreq>\n");
        xml.push_str("    <priority>0.9</priority>\n");
        
        // Self-referencing xhtml link is required by spec? Usually separate url entries are enough,
        // but adding xhtml:link inside <url> is the strict Google way for "localized versions".
        for sub_lang in langs.iter() {
            xml.push_str(&format!(
                "    <xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{}/?lang={}\" />\n",
                sub_lang, base_url, sub_lang
            ));
        }
        xml.push_str("  </url>\n");
    }
    
    xml.push_str("</urlset>");

    (
        [(axum::http::header::CONTENT_TYPE, "application/xml")],
        xml,
    )
}