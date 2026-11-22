use tree_sitter::Parser;

pub fn has_syntax_errors(source: &str) -> Result<bool, String> {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::LANGUAGE.into();

    parser
        .set_language(&language)
        .map_err(|error| error.to_string())?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| "Rust code parse nahi ho paya".to_string())?;

    Ok(tree.root_node().has_error())
}
