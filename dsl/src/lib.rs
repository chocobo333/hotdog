
use proc_macro2::TokenStream;
use syn::*;
use syn::punctuated;


// #[derive(Debug)]
// struct Rule {
//     id: Ident,
//     eq: Token![=],
//     expr: Expr,
// }
// #[derive(Debug)]
// struct ParserInformation {
//     rules: punctuated::Punctuated<Rule, Token![,]>
// }
// impl parse::Parse for Rule {
//     fn parse(input: parse::ParseStream) -> syn::Result<Self> {
//         let id = input.parse()?;
//         let eq = input.parse()?;
//         let expr = input.parse()?;
//         Ok(Rule{
//             id, eq, expr
//         })
//     }
// }
// impl parse::Parse for ParserInformation {
//     fn parse(input: parse::ParseStream) -> syn::Result<Self> {
//         let rules = punctuated::Punctuated::new();

//     }
// }

// fn parserimpl(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
//     let res: ExprStruct = parse2(tokens).unwrap();
//     for e in res.fields.clone() {
//         println!("{:?}", e.member);
//     }
//     quote::quote!{
//         pub struct #res
//     }
// }

// #[proc_macro]
// pub fn parser(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = TokenStream::from(tokens);
//     proc_macro::TokenStream::from(parserimpl(input))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parser() {
//         let tokens = quote::quote!{
//             newline = "([ ]*\n)+" + sp0 @ second @ len / gg
//             sp0 = "[ ]*"
//             sp1 = "[ ]+"
//         };
//         println!("{:?}", parserimpl(tokens));
//         assert!(true)
//     }
// }