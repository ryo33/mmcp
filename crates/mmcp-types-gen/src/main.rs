use std::fs::{self, File};
use std::io::BufReader;

use generator::TypeGeneratorConfig;

mod generator;
mod json_schema;
mod type_registry;

fn main() {
    generate(
        "schemas/mcp-2025-03-26.json",
        "crates/mmcp-protocol/src/mcp.rs",
    );
}

fn generate(schema_path: &str, output_path: &str) {
    let mut config = TypeGeneratorConfig::default().with_root_schema(
        serde_json::from_reader::<_, json_schema::RootSchema>(BufReader::new(
            File::open(schema_path).unwrap(),
        ))
        .unwrap(),
    );
    // Generate Rust code
    let tokens = syn::parse2(config.generate()).unwrap();
    let code = prettyplease::unparse(&tokens);

    // Write the generated code to a file
    fs::write(output_path, code).unwrap();

    println!("Generated code written to {:?}", output_path);
}
