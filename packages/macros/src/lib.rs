use std::str::FromStr;
use proc_macro::TokenStream;

#[proc_macro]
pub fn goto_category(input: TokenStream) -> TokenStream {
    let mut output = String::new();
    let input = input.to_string();
    let input: Vec<&str> = input.split(",").collect();

    let name = input[0];
    
    output.push_str(&format!("match {name} {{"));
    for category in input.into_iter().skip(1) {
        let cat = category.trim().replace("_", "-");
        output.push_str(&format!("\"{cat}\" => rsx!{{ {category}::Category{{}} }},"));
    }
    output.push_str("_ => unreachable!(),");
    output.push_str("}");
    TokenStream::from_str(&output).unwrap()
}
