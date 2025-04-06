use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

use generator::TypeGeneratorConfig;

mod generator;
mod json_schema;
mod type_registry;

fn main() {
    let mut config = TypeGeneratorConfig::default().with_root_schema(
        serde_json::from_reader::<_, json_schema::RootSchema>(BufReader::new(
            File::open("schemas/mcp-2025-03-26.json").unwrap(),
        ))
        .unwrap(),
    );
    // Generate Rust code
    let tokens = syn::parse2(config.generate()).unwrap();
    let code = prettyplease::unparse(&tokens);

    // Write the generated code to a file
    let output_path = Path::new("crates/mmcp/src/protocol/mcp.rs");
    fs::write(output_path, code).unwrap();

    println!("Generated code written to {:?}", output_path);
}
