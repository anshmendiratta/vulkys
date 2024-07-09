extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn info(info_text: TokenStream) -> TokenStream {
    format!("info: {info_text}").parse().unwrap()
}
