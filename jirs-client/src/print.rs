fn main() {
    std::fs::create_dir_all("./jirs-client/src/hi/syntax").unwrap();

    use syntect::{dumps::*, parsing::*};
    let syntax_set: SyntaxSet = from_binary(include_bytes!("./hi/newlines.packdump"));
    std::fs::write(
        "./jirs-client/src/hi/syntax_set.cbor",
        serde_cbor::ser::to_vec(&syntax_set).unwrap(),
    )
    .unwrap();
}
