pub mod schemars;

#[cfg(feature = "macros")]
pub use mmcp_macros::tool;

#[cfg(feature = "serde")]
pub use serde;
#[cfg(feature = "macros")]
pub use serde_json;

#[cfg(feature = "inventory")]
pub mod inventory {
    pub use ::inventory::*;

    use crate::primitives::tool::{BoxedTool, Tool};

    pub struct ToolRegistration {
        constructor: fn() -> BoxedTool,
    }

    impl ToolRegistration {
        pub const fn new<T: Tool + Default + Send + Sync + 'static>() -> Self {
            Self {
                constructor: || Box::new(T::default()),
            }
        }

        pub fn tool(&self) -> BoxedTool {
            (self.constructor)()
        }
    }

    inventory::collect!(ToolRegistration);
}
