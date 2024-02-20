use full_moon::ast::{Block, Stmt};
use oas3;
use std::{fs, path::Path, vec};
mod types;

fn main() {
    let petstore_spec = Path::new("tests/mocks/commented-schema.yml");
    let spec = oas3::from_path(petstore_spec).expect("Failed to load the OpenAPI spec file");

    let types = types::generate_luau_types(spec);
    let mut stmts = vec![];
    for type_decl in types {
        stmts.push((Stmt::ExportedTypeDeclaration(type_decl), None))
    }
    let block = Block::new().with_stmts(stmts);
    let tree = full_moon::parse("").unwrap().with_nodes(block);

    let formatted_tree = stylua_lib::format_ast(
        tree,
        stylua_lib::Config::new(),
        None,
        stylua_lib::OutputVerification::None,
    )
    .expect("failed to format");

    let formatted_str = full_moon::print(&formatted_tree);
    println!("{}", formatted_str);

    fs::write("types.luau", formatted_str).expect("failed to write file");
}
