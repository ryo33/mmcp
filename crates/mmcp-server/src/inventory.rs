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
