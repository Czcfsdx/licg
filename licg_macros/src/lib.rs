use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro_derive(FromInputUpperFn)]
pub fn from_input_upper(input: TokenStream) -> TokenStream {
    let mut iters = input.into_iter();

    // enum keyword
    loop {
        match iters.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "enum" => break,
            Some(_) => continue,
            None => return "compile_error! expected enum".parse().unwrap(),
        }
    }

    // enum name
    let name = match iters.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => return "compile_error! expected enum name".parse().unwrap(),
    };

    // enum body
    let body = loop {
        match iters.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => break group,
            Some(_) => continue,
            None => return "compile_error! expected enum body".parse().unwrap(),
        }
    };

    // collect enum variants
    let mut variants = Vec::new();
    for token in body.stream() {
        if let TokenTree::Ident(ident) = token {
            variants.push(ident.to_string());
        }
    }
    if variants.is_empty() {
        return "compile_error! expect variants in enum body"
            .parse()
            .unwrap();
    }

    let matches: String = variants
        .iter()
        .map(|v| {
            let upper_v = v.to_uppercase();
            format!("\"{upper_v}\" => Some(Self::{v}),")
        })
        .collect();

    let code = format!(
        r#"
impl {name} {{
    pub fn from_input(input: &str) -> Option<Self> {{
        match input.to_uppercase().as_str() {{
            {matches}
            _ => None,
        }}
    }}
}}
"#
    );

    code.parse().unwrap()
}
