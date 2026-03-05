#[cfg(not(feature = "use-logos"))]
pub mod classic;

#[cfg(not(feature = "use-logos"))]
pub use classic::*;

#[cfg(feature = "use-logos")]
pub mod logos;

#[cfg(feature = "use-logos")]
pub use logos::*;
