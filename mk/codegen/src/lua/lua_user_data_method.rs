use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{__private::ext::RepToTokensExt, format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated,  FnArg, Ident, ImplItem, ItemImpl,
    Lit, Meta, NestedMeta, Token, Type,
};

pub fn lua_user_data_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    let mut method_impls = Vec::new();
    let mut static_method_impls = Vec::new();

    for item in &impl_block.items {
        let method = if let ImplItem::Method(method) = &item {
            method
        } else {
            continue;
        };

        if method
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("hidden"))
            .is_some()
        {
            continue;
        }

        let ident = &method.sig.ident;
        let lua_ident = if let Some(attr) = method
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("rename"))
        {
            let meta = attr.parse_meta().unwrap();
            match meta {
                Meta::List(list) => {
                    if list.nested.len() != 1 {
                        abort_call_site!("rename attribute must have exactly one argument")
                    }

                    match list.nested.first().unwrap().next().unwrap() {
                        NestedMeta::Lit(lit) => match lit {
                            Lit::Str(str) => str.value(),
                            _ => {
                                abort_call_site!(
                                    "the argument of rename attribute must be a string"
                                )
                            }
                        },
                        _ => abort_call_site!("the argument of rename attribute must be a string"),
                    }
                }
                _ => ident.to_string(),
            }
        } else {
            ident.to_string()
        };

        let params = method
            .sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Receiver(_) => None,
                FnArg::Typed(typed) => Some(typed),
            })
            .collect::<Vec<_>>();
        let param_idents = (0..params.len())
            .map(|i| format_ident!("param_{}", i))
            .collect::<Punctuated<Ident, Token![,]>>();
        let param_types = (0..params.len())
            .map(|_| quote! { mlua::Value })
            .collect::<Punctuated<_, Token![,]>>();
        let arg_impls = params
            .iter()
            .enumerate()
            .map(|(i, typed)| {
                let param_ident = format_ident!("param_{}", i);
                let arg_ident = format_ident!("arg_{}", i);
                let ty = if let Type::Reference(reference) = typed.ty.as_ref() { 
                    reference.elem.as_ref()
                } else {
                    typed.ty.as_ref()
                };
                let mutability = if let Type::Reference(reference) = typed.ty.as_ref() {
                    reference.mutability
                } else {
                    None
                };
                quote! {
                    let #mutability #arg_ident = <#ty as crate::script::ConversionByValue>::perform_conversion_from_lua(#param_ident, lua)?;
                }
            }).collect::<Vec<_>>();
        let arg_forward_impls = params
            .iter()
            .enumerate()
            .map(|(i, typed)| {
                let arg_ident = format_ident!("arg_{}", i);

                if let Type::Reference(reference) = typed.ty.as_ref() {
                    if reference.mutability.is_some() {
                        quote! {
                            &mut #arg_ident
                        }
                    } else {
                        quote! {
                            &#arg_ident
                        }
                    }
                } else {
                    quote! {
                        #arg_ident
                    }
                }
            })
            .collect::<Punctuated<_, Token![,]>>();

        let no_except = method
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("no_except"))
            .is_some();
        let return_impl = if no_except {
            quote! {
                crate::script::ConversionByValueReadOnly::perform_convertion_to_lua(&return_value, lua)
            }
        } else {
            quote! {
                crate::script::ConversionByValueReadOnly::perform_convertion_to_lua(&return_value?, lua)
            }
        };

        match MethodType::from_method_inputs(&method.sig.inputs) {
            MethodType::Static => {
                static_method_impls.push(quote! {
                    table.set(#lua_ident, lua.create_function(|lua, (#param_idents): (#param_types)| {
                        #(#arg_impls)*
                        let return_value = Self::#ident(#arg_forward_impls);
                        #return_impl
                    })?)?;
                });
            }
            MethodType::Ref => {
                method_impls.push(quote! {
                    _methods.add_method(#lua_ident, |lua, this, (#param_idents): (#param_types)| {
                        #(#arg_impls)*
                        let return_value = this.#ident(#arg_forward_impls);
                        #return_impl
                    });
                });
            }
            MethodType::Mut => {
                method_impls.push(quote! {
                    _methods.add_method_mut(#lua_ident, |lua, this, (#param_idents): (#param_types)| {
                        #(#arg_impls)*
                        let return_value = this.#ident(#arg_forward_impls);
                        #return_impl
                    });
                });
            }
            MethodType::Take => {
                method_impls.push(quote! {
                    _methods.add_function(#lua_ident, |lua, (this, #param_idents): (mlua::AnyUserData, #param_types)| {
                        #(#arg_impls)*
                        let return_value = <Self as crate::script::UserDataIntoSelf>::into_self(this)?.#ident(#arg_forward_impls);
                        #return_impl
                    });
                });
            }
        }
    }

    let impl_to_string = if impl_block.attrs.iter().find(|attr| attr.path.is_ident("ops_to_string")).is_some() {
        quote! {
            _methods.add_meta_method(mlua::MetaMethod::ToString, |_lua, this, ()| {
                Ok(<_ as ToString>::to_string(this))
            });
        }
    } else {
        quote! {}
    };

    let ty_name = &impl_block.self_ty;
    let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();
    let methods_impl = quote! {
        impl #impl_generics crate::script::UserDataMethodProvider for #ty_name #ty_generics #where_clause {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {
                #(#method_impls)*
                #impl_to_string
                <Self as crate::script::UserDataOpsProvider>::add_ops(_methods);
            }
        }
    };
    let ops_impl = if impl_block.attrs.iter().find(|attr| attr.path.is_ident("ops_extra")).is_some() {
        quote! {}
    } else {  
        quote! {
            impl #impl_generics crate::script::UserDataOpsProvider for #ty_name #ty_generics #where_clause {}
        }
    };
    let static_methods_impl = quote! {
        impl #impl_generics crate::script::LuaApiTable for #ty_name #ty_generics #where_clause {
            fn create_api_table<'lua>(lua: &'lua mlua::Lua) -> mlua::Result<mlua::Table<'lua>> {
                let table = lua.create_table()?;
                #(#static_method_impls)*
                Ok(table)
            }
        }
    };

    TokenStream::from(quote! {
        #impl_block

        #methods_impl

        #ops_impl

        #static_methods_impl
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MethodType {
    Static,
    Ref,
    Mut,
    Take,
}

impl MethodType {
    pub fn from_method_inputs(inputs: &Punctuated<FnArg, Token![,]>) -> Self {
        let first = if let Some(first) = inputs.first() {
            first
        } else {
            return Self::Static;
        };

        let receiver = if let FnArg::Receiver(receiver) = first {
            receiver
        } else {
            return Self::Static;
        };

        if receiver.reference.is_none() {
            return Self::Take;
        }

        match receiver.mutability {
            Some(_) => Self::Mut,
            None => Self::Ref,
        }
    }
}
