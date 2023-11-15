extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Expr, Ident, Token, Type, Visibility};

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
                return Ok(Networking {
                    url,
                    user_agent,
                    headers,
                    cookies,
                });
            } else {
                let none_syn: Expr = parse_quote! { None };
                return Ok(Networking {
                        url,
                        user_agent,
                        headers: none_syn.clone(),
                        cookies: none_syn,
                });
            }
        } else {
            let none_syn: Expr = parse_quote! { None };
            return Ok(Networking {
                url,
                user_agent: none_syn.clone(),
                headers: none_syn.clone(),
                cookies: none_syn,
            });
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
            #cookies,
        )
    };
    return TokenStream::from(expanded);
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
            #cookies,
        )
    };
    return TokenStream::from(expanded);
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
        networking::post(
            #url,
            #user_agent,
            #headers,
            #cookies,
        )
    };
    return TokenStream::from(expanded);
}

