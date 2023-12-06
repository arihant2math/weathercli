extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, parse_quote, Expr, Token};

struct Networking {
    url: Expr,
    user_agent: Expr,
    headers: Expr,
    cookies: Expr,
}

impl Parse for Networking {
    fn parse(input: ParseStream) -> Result<Self> {
        let url: Expr = input.parse()?;
        let n = input.parse::<Token![,]>();
        if n.is_ok() {
            let user_agent: Expr = input.parse()?;
            let n = input.parse::<Token![,]>();
            if n.is_ok() {
                let headers: Expr = input.parse()?;
                input.parse::<Token![,]>()?;
                let cookies: Expr = input.parse()?;
                Ok(Networking {
                    url,
                    user_agent,
                    headers,
                    cookies,
                })
            } else {
                let none_syn: Expr = parse_quote! { None };
                let none_syn_borrow: Expr = parse_quote! { &None };
                Ok(Networking {
                        url,
                        user_agent,
                        headers: none_syn,
                        cookies: none_syn_borrow,
                })
            }
        } else {
            let none_syn: Expr = parse_quote! { None };
            let none_syn_borrow: Expr = parse_quote! { &None };
            Ok(Networking {
                url,
                user_agent: none_syn.clone(),
                headers: none_syn,
                cookies: none_syn_borrow,
            })
        }
    }
}

#[proc_macro]
pub fn get(input: TokenStream) -> TokenStream {
    let Networking {
        url,
        user_agent,
        headers,
        cookies
    } = parse_macro_input!(input as Networking);
    let expanded = quote! {
        networking::get_url(
            #url,
            #user_agent,
            #headers,
            #cookies
        )
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn getf(input: TokenStream) -> TokenStream {
    let e: Expr = parse_macro_input!(input as Expr);
    let expanded = quote! {
        networking::get_url(format!(#e), None, None, None)
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn gets(input: TokenStream) -> TokenStream {
    let Networking {
        url,
        user_agent,
        headers,
        cookies
    } = parse_macro_input!(input as Networking);
    let expanded = quote! {
        networking::get_urls(
            #url,
            #user_agent,
            #headers,
            #cookies
        )
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn post(input: TokenStream) -> TokenStream {
    let Networking {
        url,
        user_agent,
        headers,
        cookies
    } = parse_macro_input!(input as Networking);
    let expanded = quote! {
        networking::post_url(
            #url,
            #user_agent,
            #headers,
            #cookies
        )
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn postf(input: TokenStream) -> TokenStream {
    let e: Expr = parse_macro_input!(input as Expr);
    let expanded = quote! {
        networking::post_url(format!(#e), None, None, None)
    };
    TokenStream::from(expanded)
}
