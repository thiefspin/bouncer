use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn measure_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a function
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident; // Function name
    let attrs = &input.attrs;    // Function attributes
    let vis = &input.vis;        // Visibility (pub, etc.)
    let sig = &input.sig;        // Function signature
    let block = &input.block;    // Function body

    let is_async = input.sig.asyncness.is_some(); // Check if function is async

    let expanded = if is_async {
        // Async function
        quote! {
            #(#attrs)*
            #vis #sig {
                let start = std::time::Instant::now();
                let result = async move { #block }.await;
                let elapsed = start.elapsed();
                println!("Async function '{}' took {:?}", stringify!(#name), elapsed);
                result
            }
        }
    } else {
        // Sync function
        quote! {
            #(#attrs)*
            #vis #sig {
                let start = std::time::Instant::now();
                let result = (|| #block )();
                let elapsed = start.elapsed();
                println!("Function '{}' took {:?}", stringify!(#name), elapsed);
                result
            }
        }
    };

    TokenStream::from(expanded)
}