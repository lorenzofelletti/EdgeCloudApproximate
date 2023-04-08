use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn table_name(attr: TokenStream, item: TokenStream) -> TokenStream {
    let table_name = attr.to_string();

    let item_fn = parse_macro_input!(item as ItemFn);

    let fn_name = item_fn.sig.ident;
    let fn_in = item_fn.sig.inputs;
    let fn_out = item_fn.sig.output;
    let fn_vis = item_fn.vis;
    let fn_body = item_fn.block;

    let out = quote! {
        #fn_vis fn #fn_name(#fn_in) #fn_out {
            let table_name = #table_name;
            let data = get_table(table_name, config)?;
            #fn_body
        }
    };

    TokenStream::from(out)
}
