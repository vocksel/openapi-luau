use oas3;
use std::path::Path;
mod types;

fn main() {
    let petstore_spec = Path::new("tests/mocks/commented-schema.yml");
    let spec = oas3::from_path(petstore_spec).expect("Failed to load the OpenAPI spec file");

    let tree = types::generate_luau_types(spec);
    println!("{:?}", tree);
}
