use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro_derive(FromInputUpperFn)]
pub fn derive_from_input_upper_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    // enum keyword
    loop {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "enum" => break,
            Some(_) => continue,
            None => return "compile_error! expected enum".parse().unwrap(),
        }
    }

    // enum name
    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => return "compile_error! expected enum name".parse().unwrap(),
    };

    // enum body
    let body = loop {
        match tokens.next() {
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
            format!("\"{upper_v}\" => Some(Self::{v}),\n")
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

#[proc_macro_derive(ListVariantsWithDocFn)]
pub fn derive_list_variants_with_doc_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    // enum keyword and collect the doc
    let mut enum_docs: Vec<String> = Vec::new();
    loop {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "enum" => break,
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                let mut tokens = group.stream().into_iter();
                if let Some(TokenTree::Ident(ident)) = tokens.next()
                    && ident.to_string() == "doc"
                    && let Some(TokenTree::Punct(punct)) = tokens.next()
                    && punct.as_char() == '='
                    && let Some(TokenTree::Literal(literal)) = tokens.next()
                {
                    let str = literal.to_string().trim_matches('"').trim().to_string();
                    enum_docs.push(str);
                }
            }
            Some(_) => continue,
            None => return "compile_error! expected enum".parse().unwrap(),
        }
    }

    // enum name
    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => return "compile_error! expected enum name".parse().unwrap(),
    };

    // enum body
    let body = loop {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => break group,
            Some(_) => continue,
            None => return "compile_error! expected enum body".parse().unwrap(),
        }
    };

    // collect enum variants
    let mut body_tokens = body.stream().into_iter();
    let mut variants_with_doc = Vec::new();
    let mut docs_variants = Vec::new();
    let mut max_variant_name_len = 0; // for align
    loop {
        match body_tokens.next() {
            Some(TokenTree::Ident(ident)) => {
                let doc = docs_variants.join(" ");
                docs_variants.clear();
                let variant_name = ident.to_string();
                if variant_name.len() > max_variant_name_len {
                    max_variant_name_len = variant_name.len();
                }
                variants_with_doc.push((variant_name, doc));
            }
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                let mut body_tokens = group.stream().into_iter();
                if let Some(TokenTree::Ident(ident)) = body_tokens.next()
                    && ident.to_string() == "doc"
                    && let Some(TokenTree::Punct(punct)) = body_tokens.next()
                    && punct.as_char() == '='
                    && let Some(TokenTree::Literal(literal)) = body_tokens.next()
                {
                    let str = literal.to_string().trim_matches('"').trim().to_string();
                    docs_variants.push(str);
                }
            }
            Some(_) => continue,
            None => break,
        }
    }

    if variants_with_doc.is_empty() {
        return "compile_error! expect variants in enum body"
            .parse()
            .unwrap();
    }

    let print_variants_with_doc: String = variants_with_doc
        .iter()
        .map(|(v, doc)| {
            let variant_name_len = v.len();
            let padding = " ".repeat(max_variant_name_len - variant_name_len);
            format!("println!(\"    {v}: {padding}{doc}\");\n")
        })
        .collect();
    let enum_docs = enum_docs.join(" ");
    let code = format!(
        r#"
impl {name} {{
    pub fn list() -> () {{
        println!("{enum_docs}:");
        {print_variants_with_doc}
    }}
}}
"#
    );

    code.parse().unwrap()
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn print_token(input: TokenStream, indent: usize) {
    for token in input {
        print!("{}", " ".repeat(indent));
        match token {
            TokenTree::Group(g) => {
                print!("Group: ");
                match g.delimiter() {
                    Delimiter::Parenthesis => println!("("),
                    Delimiter::Brace => println!("{{"),
                    Delimiter::Bracket => println!("["),
                    Delimiter::None => println!(" "),
                }
                print_token(g.stream(), indent + 4);
                print!("{}", " ".repeat(indent));
                match g.delimiter() {
                    Delimiter::Parenthesis => println!(")"),
                    Delimiter::Brace => println!("}}"),
                    Delimiter::Bracket => println!("]"),
                    Delimiter::None => println!(" "),
                }
            }
            TokenTree::Ident(i) => println!("Ident: {i}"),
            TokenTree::Punct(p) => println!("Punct: {p}"),
            TokenTree::Literal(l) => println!("Literal: {l}"),
        }
    }
}
