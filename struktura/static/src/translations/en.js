export const en = {
  pageTitle: "Struktura - Precision Material & Structural Calculator",
  tagline: "See the big picture. Nail the details.",
  headline: "Build With Precision. Waste Nothing.",
  subhead:
    "From DIY decks to skyscraper foundations. The only calculator that adapts to your expertise level. Powered by Rust. Accurate to 99.8%.",
  login: "Log In",
  signup: "Sign Up",
  guide: "Guide",
  logout: "Log Out",
  dashboard: "Dashboard",
  language: "Language",
  theme: "Theme",
  hero_trust_1: "Professional Grade",
  hero_trust_2: "Privacy Focused",
  ticker:
    "Concrete • Lumber • Steel • Roofing • HVAC • Loads • Costs • Fluid Dynamics",
  mode_cards: {
    title_prefix: "Choose Your",
    title_highlight: "Experience Level",
    subtitle:
      "From weekend warriors to licensed professionals, Struktura adapts to your needs.",
    select_mode: "Select Mode",
    cta_text:
      "Not sure which mode to choose? Start with DIY and upgrade anytime.",
    cta_link: "Read the full guide",
  },
  device_mockups: {
    load_a: "Load A",
    value_load_a: "12.4 kN",
    shear: "Shear",
    value_shear: "4.2 MPa",
    total_cost_prefix: "Total Cost",
    value_total_cost: "$420.50",
  },
  modes: {
    beginner: {
      title: "DIY & Homeowner",
      desc: "Material lists & shopping carts. Build decks, walls, and sheds without the math headache.",
      link: "beginner.html",
    },
    builder: {
      title: "Contractor",
      desc: "Rapid estimating, unit conversion, and bid management for professionals on site.",
      link: "medium.html",
    },
    professional: {
      title: "Engineer",
      desc: "Full structural load analysis, beam deflection, and physics simulation.",
      link: "advanced.html",
    },
  },
  beginner: {
    // Header Section
    badge: "DIY & Home Projects",
    title: "The DIY Corner: Get Started",
    subtitle:
      "Material lists & shopping carts. Build decks, walls, and sheds without the math headache.",
    note_label: "Note",

    // Categories (already exist in your files)
    categories: {
      outdoors: "Outdoors & Structures",
      interiors: "Interiors & Finishes",
      utilities: "Tools & Utilities",
      garden: "Landscaping & Garden",
    },

    // Calculators (already exist in your files)
    calcs: {
      deck: "Deck Framing Materials",
      fence: "Fence Post Count",
      concrete_slab: "Concrete Slab Volume",
      wall_framing: "Wall Framing Lumber",
      drywall_count: "Drywall Sheet Count",
      paint_coverage: "Paint Gallons Needed",
      tile_count: "Floor Tile Count",
      planter_box: "Planter Box Wood & Soil",
      soil_volume: "Soil Volume (M³)",
    },

    // Form Section
    form: {
      title: "Your Project Dimensions",
      select_project: "What are you building?",
      choose_project: "Choose a project...",
      dimensions: "Dimensions",
      typical_range: "Typical",
      calculating: "Calculating...",
      calculate: "Calculate Materials",
      help_text: "Enter your project dimensions above and click Calculate",
    },

    // Results Section
    results: {
      // Loading & Error States
      calculating_needs: "Calculating your project needs...",
      calculation_error: "Calculation Error",

      // Empty State
      ready_to_calc: "Ready to Build!",
      get_started: "Select a project and enter dimensions to get started!",
      perfect_for: "Perfect for:",

      // Success State
      success: "Your Project Needs",
      needs_subtitle: "Here's what you'll need for this project",
      important_notes: "Important Considerations",

      // Shopping List
      shopping_list: "Shopping List",
      round_up: "Round up when purchasing",
      print_list: "Print List",
      copy_list: "Copy List",

      // Next Steps
      next_steps: "Next Steps",
      step_1: "Purchase materials from your shopping list",
      step_2: "Check local building codes and get permits if needed",
      step_3: "Gather tools and safety equipment before starting",
    },

    // Units (already exist in your files)
    units: {
      meters: "m",
      pieces: "pcs",
      m3: "m³",
    },

    // Tips (already exist in your files)
    tips: {
      title: "Quick Tips Before You Build",
      local_codes:
        "Always check local building codes—it saves headaches later!",
      seasonal_pricing:
        "Material prices can change seasonally; check local supplier bulk rates.",
      material_waste:
        "Order 10-15% extra material to account for waste and mistakes.",
    },

    // Info Cards (bottom of page)
    info_cards: {
      tips: {
        title: "Pro Tips",
        item_1: "Always check local building codes before starting",
        item_2: "Add 10-15% extra for material waste",
        item_3: "Consider seasonal pricing variations",
      },
      common_uses: "Common Uses",
      safety: {
        title: "Safety First",
        text: "These calculations are estimates. For structural projects or permits, consult a licensed professional.",
      },
    },

    // Visualization Section
    visualization: {
      title: "3D Visualization",
      rotate_zoom: "Rotate and zoom to inspect dimensions.",
      unavailable_item: "3D preview not available for this item.",
      not_available: "No visualization available",
    },

    // Loading and misc (already exist in your files)
    loading: "Calculating...",
    no_results: "Enter your dimensions and click 'Calculate' to see results.",
  },
  disclaimer: {
    title: "Disclaimer",
    text: "Struktura is for estimation purposes only. Always consult a licensed engineer for final structural plans and comply with local building codes.",
  },
  footer: {
    tagline:
      "Precision material and structural calculations for professionals and DIY builders worldwide.",
    status: "All Systems Operational",
    product: {
      title: "Product",
      features: "Features",
      pricing: "Pricing",
      changelog: "Changelog",
      documentation: "Documentation",
    },
    company: {
      title: "Company",
      about: "About",
      blog: "Blog",
      careers: "Careers",
      contact: "Contact",
    },
    legal: {
      title: "Legal",
      privacy: "Privacy",
      terms: "Terms",
      security: "Security",
      cookies: "Cookies",
    },
    copyright: "All rights reserved.",
    social: {
      twitter: "Twitter",
      github: "GitHub",
      linkedin: "LinkedIn",
    },
  },
  contractor: {
    title: "Professional Contractor Tools",
    subtitle: "Bid accurately. Estimate quickly. Manage efficiently.",
    header: {
      badge: "Professional Suite",
    },
    categories: {
      bidding: "Bidding & Tenders",
      scheduling: "Scheduling",
      estimation: "Cost Estimation",
      management: "Site Management",
    },
    form: {
      title: "Project Parameters",
      select_calc: "Select Calculator",
      choose_calc: "Choose a calculator...",
      calculate: "Calculate",
      calculating: "Processing...",
      output_format: "Output Format",
    },
    formats: {
      standard: "Standard",
      detailed: "Detailed",
      summary: "Summary",
    },
    sections: {
      dimensions: "Dimensions",
      material: "Material Properties",
      resources: "Resource Requirements",
      safety_factors: "Safety Factors",
      additional: "Additional Parameters",
      project_info: "Project Information",
    },
    results: {
      calculating: "Processing contracting calculations...",
      error_title: "Calculation Error",
      ready_title: "Ready to Calculate",
      ready_desc:
        "Input parameters and click Calculate to generate contracting estimates.",
      critical: "Critical Parameters",
      calculated: "Calculated Results",
    },
    analysis: {
      title: "Project Analysis",
      cost: "Total Cost",
      duration: "Duration",
      risk: "Risk Level",
      compliance: "Compliance",
    },
    meta: {
      codes: "Applicable Codes",
      complexity: "Complexity",
      certification_required: "Certification Required",
      certification_desc:
        "Professional certification review required before project execution.",
    },
    actions: {
      print: "Print",
      export: "Export CSV",
    },
  },
  engineer: {
    // Header Section
    badge: "Advanced Engineering Analysis",
    title: "Structural & Material Calculators",
    subtitle:
      "Precision tools for structural integrity, materials science, and civil planning.",
    ready_to_calc: "Ready to Calculate",

    // Output Format Selector
    output_format: {
      standard: "Standard Output",
      detailed: "Detailed Analysis",
      summary: "Summary Report",
    },

    // Category Names (already exist, but ensure they're here)
    categories: {
      civil: "Civil",
      structural: "Structural",
      mechanical: "Mechanical",
      production: "Production",
    },

    // Form Section
    form: {
      title: "Input Parameters",
      select_calculator: "Select Calculator",
      choose_calculator: "Choose a calculator...",
      select_placeholder: "Select...",
      design_code: "Design Code",
      select_design_code: "Select Design Code",
      calculating: "Calculating...",
      calculate: "Calculate",
      required_fields: "Required fields must be filled",

      sections: {
        dimensions: "Dimensions",
        material: "Material Properties",
        loads: "Load Cases",
        safety_factors: "Safety Factors",
        additional: "Additional Parameters",
      },
    },

    // Results Section
    results: {
      // Loading & Error States
      loading_analysis: "Analyzing Structure...",
      running_analysis: "Running structural analysis...",
      calculation_error: "Calculation Error",

      // Empty State
      ready_to_calc: "Ready to Calculate",
      input_prompt:
        "Input parameters and click Calculate to generate technical specifications.",
      typical_applications: "Typical Applications:",

      // Results Display
      critical_parameters: "Critical Design Parameters",
      compliance_title: "Design & Compliance Analysis",
      recommendations_title: "Recommendations",
      calculated_results: "Calculated Results",
      data_analysis: "Data Analysis",

      // Actions
      print_spec: "Print Spec",
      export_csv: "Export CSV",

      // Warning Severity Levels
      severity: {
        critical: "Critical Safety Issue",
        high: "High Priority Warning",
        medium: "Advisory Notice",
        low: "Informational",
      },
    },

    // Visualization Section
    visualization: {
      title: "3D Visualization",
      unavailable: "3D preview unavailable for this selected calculation.",
      panel_title: "Project Visualization",
      preview_subtitle: "3D preview of project dimensions",
      area: "Area",
      volume: "Volume",
      material: "Material",
    },
  },
};
