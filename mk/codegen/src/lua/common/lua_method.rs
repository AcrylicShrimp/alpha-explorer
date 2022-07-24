use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Ident, Result as SynResult, Token,
};

struct LuaMethodParam {
    method: Ident,
    as_name: Option<Ident>,
}

impl Parse for LuaMethodParam {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        parenthesized!(content in input);
        let method = content.parse()?;

        if content.peek(Token![as]) {
            content.parse::<Token![as]>()?;
            let as_name = content.parse()?;
            Ok(Self {
                method,
                as_name: Some(as_name),
            })
        } else {
            Ok(Self {
                method,
                as_name: None,
            })
        }
    }
}

pub fn impl_lua_method(input: &DeriveInput) -> TokenStream {
    let mut method_names = Vec::with_capacity(input.attrs.len());
    let mut methods = Vec::with_capacity(input.attrs.len());

    for attr in &input.attrs {
        let ident = if let Some(ident) = attr.path.get_ident() {
            ident
        } else {
            continue;
        };

        if ident != "lua_method" {
            continue;
        }

        let tokens = TokenStream::from(attr.tokens.clone());
        let param = parse_macro_input!(tokens as LuaMethodParam);

        method_names.push(param.as_name.unwrap_or(param.method.clone()).to_string());
        methods.push(param.method);
    }

    TokenStream::from(quote! {
        #(
            methods.add_method(#method_names, |lua, this, param: LuaMultiValue| {
                this.#methods(lua, <_>::from_lua_multi(param, lua)?)
            });
        )*
    })
}
