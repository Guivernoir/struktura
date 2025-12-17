pub mod beginner;
pub mod contractor;
pub mod engineer;

// Re-export commonly used types from beginner module for convenience
pub use beginner::*;
pub use engineer::*;
pub use contractor::*;
