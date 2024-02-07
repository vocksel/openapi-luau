use full_moon::{
    ast::types::{ExportedTypeDeclaration, TypeDeclaration, TypeInfo},
    tokenizer::{Token, TokenReference},
    ShortString,
};
fn main() {
    // let petstore = fs::read_to_string("./mocks/petstore.yml");

    // okay yeah it looks like it would be really nice to build the types file this way.
    let pet = ExportedTypeDeclaration::new(TypeDeclaration::new(
        "Pet".parse::<TokenReference>().unwrap(),
        TypeInfo::Basic(TokenReference::symbol("string").unwrap()),
    ));
    println!("{:?}", pet);
    println!("Hello, world!");
}
