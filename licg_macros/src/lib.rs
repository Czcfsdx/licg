use proc_macro::{Delimiter, TokenStream, TokenTree};
use std::fs;

#[proc_macro_derive(FromInputUpperFn)]
pub fn derive_from_input_upper_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    // enum keyword
    loop {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "enum" => break,
            Some(_) => continue,
            None => panic!("compile_error! expected enum"),
        }
    }

    // enum name
    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => panic!("compile_error! expected enum name"),
    };

    // enum body
    let body = loop {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => break group,
            Some(_) => continue,
            None => panic!("compile_error! expected enum body"),
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
        panic!("compile_error! expect variants in enum body");
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
            None => panic!("compile_error! expected enum"),
        }
    }

    // enum name
    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => panic!("compile_error! expected enum name"),
    };

    // enum body
    let body = loop {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => break group,
            Some(_) => continue,
            None => panic!("compile_error! expected enum body"),
        }
    };

    // collect enum variants
    let mut body_tokens = body.stream().into_iter();
    struct VariantWithDoc {
        name: String,
        doc: String,
    }
    let mut variants_with_doc: Vec<VariantWithDoc> = Vec::new();
    let mut docs_variant = Vec::new();
    let mut max_variant_name_len = 0; // for align
    loop {
        match body_tokens.next() {
            Some(TokenTree::Ident(ident)) => {
                let name = ident.to_string();
                if name.len() > max_variant_name_len {
                    max_variant_name_len = name.len();
                }
                let doc = docs_variant.join(" ");
                docs_variant.clear();
                variants_with_doc.push(VariantWithDoc { name, doc });
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
                    docs_variant.push(str);
                }
            }
            Some(_) => continue,
            None => break,
        }
    }

    if variants_with_doc.is_empty() {
        panic!("compile_error! expect variants in enum body");
    }

    let print_variants_with_doc: String = variants_with_doc
        .iter()
        .map(|VariantWithDoc { name, doc }| {
            let variant_name_len = name.len();
            let padding = " ".repeat(max_variant_name_len - variant_name_len);
            format!("println!(\"    {name}: {padding}{doc}\");\n")
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

#[proc_macro_derive(FmtLicenseFn, attributes(template))]
pub fn derive_fmt_license_fn(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    // enum keyword
    loop {
        match tokens.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "enum" => break,
            Some(_) => continue,
            None => panic!("compile_error! expected enum"),
        }
    }

    // enum name
    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => panic!("compile_error! expected enum name"),
    };

    // enum body
    let body = loop {
        match tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => break group,
            Some(_) => continue,
            None => panic!("compile_error! expected enum body"),
        }
    };

    // collect enum variants
    let mut body_tokens = body.stream().into_iter();
    struct VariantWithTemplate {
        name: String,
        template: String,
    }
    let mut variants_with_template: Vec<VariantWithTemplate> = Vec::new();
    let mut variant_template: Option<String> = None;
    loop {
        match body_tokens.next() {
            Some(TokenTree::Ident(ident)) => {
                let name = ident.to_string();
                if let Some(template_path) = variant_template {
                    if let Ok(template) = fs::read_to_string(&template_path) {
                        variants_with_template.push(VariantWithTemplate { name, template });
                    } else {
                        panic!(
                            "compile_error! fail to read template of variant {name} from {template_path}"
                        );
                    }
                } else {
                    panic!("compile_error! expect template attribute in variant {name}");
                }
                variant_template = None;
            }
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                let mut body_tokens = group.stream().into_iter();
                if let Some(TokenTree::Ident(ident)) = body_tokens.next()
                    && ident.to_string() == "template"
                    && let Some(TokenTree::Punct(punct)) = body_tokens.next()
                    && punct.as_char() == '='
                    && let Some(TokenTree::Literal(literal)) = body_tokens.next()
                {
                    let template_path = literal.to_string().trim_matches('"').trim().to_string();
                    variant_template = Some(template_path);
                }
            }
            Some(_) => continue,
            None => break,
        }
    }

    if variants_with_template.is_empty() {
        panic!("compile_error! expect variants in enum body");
    }

    let fmt_variants_with_template: String = variants_with_template
        .iter()
        .map(|VariantWithTemplate { name, template }| {
            format!("Self::{name} => {{ write!(f, {template:?}) }}\n")
        })
        .collect();

    let code = format!(
        r#"
impl {name} {{
    pub fn fmt_license(&self, f: &mut fmt::Formatter<'_>, year: i32, holder: &str) -> fmt::Result {{
        match self {{
            {fmt_variants_with_template}
        }}
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
