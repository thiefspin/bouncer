use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Pat};

#[proc_macro_attribute]
pub fn log_user(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident; // Function name
    let attrs = &input.attrs;    // Function attributes
    let vis = &input.vis;        // Visibility (pub, etc.)
    let sig = &input.sig;        // Function signature
    let block = &input.block;    // Function body

    // Find the "user" parameter in the function signature
    let user_param = input.sig.inputs.iter().find_map(|arg| {
        if let FnArg::Typed(pat) = arg {
            if let Pat::Ident(ident) = &*pat.pat {
                if ident.ident == "user" {
                    return Some(ident);
                }
            }
        }
        None
    });

    let log_statement = if let Some(user_ident) = user_param {
        quote! {
            println!(
                "[LOG] User ID: {}, Username: {} - Function: {}",
                #user_ident.user.id, #user_ident.user.email, stringify!(#name)
            );
        }
    } else {
        quote! {} // If no `user` parameter is found, do nothing.
    };

    // Generate new function with logging injected
    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            #log_statement
            #block
        }
    };

    TokenStream::from(expanded)
}