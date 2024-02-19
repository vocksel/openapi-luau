use full_moon::{
    ast::{Ast, Block, Stmt},
    parse,
};
use oas3;
use std::{fs, os::unix::net::SocketAddr, path::Path};
mod types;

fn main() {
    let petstore_spec = Path::new("tests/mocks/commented-schema.yml");
    let spec = oas3::from_path(petstore_spec).expect("Failed to load the OpenAPI spec file");

    let petstore_types = fs::read_to_string("tests/mocks/petstore-types.luau").expect("bad file");
    let tree = full_moon::parse(&petstore_types).expect("bad luau file");
    println!("{:#?}", tree);

    let types = types::generate_luau_types(spec);
    tree = Ast::with_nodes(self, Block::with_stmts(self, stmts));
    println!("{:#?}", tree)
}
