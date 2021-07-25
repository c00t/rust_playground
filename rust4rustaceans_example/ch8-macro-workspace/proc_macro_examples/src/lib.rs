// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

//extern crate proc_macro;
use proc_macro::TokenStream;

/// TRR examples
#[proc_macro]
pub fn make_answer(_item:TokenStream)->TokenStream{
    "fn answer() -> u32 {42}".parse().unwrap()
}

/// TRR derive proc-macro example
#[proc_macro_derive(AnswerFnDerive)]
pub fn derive_answer_fn(_item:TokenStream)->TokenStream{
    "fn answer_derive() -> u32 {42}".parse().unwrap()
}

/// TRR derive macro helper attributes example
#[proc_macro_derive(ZZZZ,attributes(helper1,helper2))]
pub fn derive_macro_helper(_item:TokenStream)->TokenStream{
    TokenStream::new()
}

/// TRR test attribute macros
#[proc_macro_attribute]
pub fn show_streams(_attr:TokenStream,_item:TokenStream)->TokenStream{
    println!("attr: \"{}\"",_attr.to_string());
    println!("item: \"{}\"",_item.to_string());
    _item
}
