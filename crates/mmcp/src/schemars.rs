use std::borrow::Cow;

#[cfg(feature = "schemars1")]
pub use schemars1 as schemars;
#[cfg(all(feature = "schemars08", not(feature = "schemars1")))]
pub use schemars08 as schemars;

pub use schemars::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Any {}

impl JsonSchema for Any {
    fn schema_name() -> Cow<'static, str> {
        "Any".into()
    }

    fn schema_id() -> Cow<'static, str> {
        concat!(module_path!(), "::Any").into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({})
    }
}

#[test]
fn test_schema() {
    use serde_json::json;
    assert_eq!(
        schema_for!(Any).as_value(),
        &json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Any",
        })
    );
}
