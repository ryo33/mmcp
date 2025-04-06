pub mod primitives;
pub mod protocol;

#[cfg(feature = "schemars1")]
pub use schemars1 as schemars;
#[cfg(all(feature = "schemars08", not(feature = "schemars1")))]
pub use schemars08 as schemars;
