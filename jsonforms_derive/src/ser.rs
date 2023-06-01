use crate::internals::symbol::*;
use itertools::join;
use proc_macro2::{Span, TokenStream};
use syn::{meta::ParseNestedMeta, spanned::Spanned, DeriveInput};

pub(crate) fn expand_derive_jsonforms(
    input: &mut DeriveInput,
) -> Result<TokenStream, Vec<syn::Error>> {
    let ident = &input.ident;
    let mut debug = false;
    let attrs = &input.attrs;
    for attr in attrs {
        if attr.path() != JSONFORMS {
            continue;
        }

        if let syn::Meta::List(meta) = &attr.meta {
            if meta.tokens.is_empty() {
                continue;
            }
        }

        if let Err(err) = attr.parse_nested_meta(|meta| {
            if meta.path == DEBUG {
                debug = true;
            }
            Ok(())
        }) {
            return Err(vec![syn::Error::new(
                input.span(),
                format!(
                    "expand_derive_jsonforms: invalid meta parse for jsonforms attribute {:?}",
                    err
                ),
            )]);
        }
    }
    let scope = String::from("#/properties");
    let (props, uiprops) = expand_props(&input.data, input.span(), scope)?;

    let quote = quote! {
        impl JsonFormsSerializable for #ident {
            fn jsonforms_schema() -> (String,String) {
                let mut out_str = String::new();
                #props
                let mut uiout_str = String::new();
                #uiprops
                (out_str,uiout_str)
            }
        }
    };
    if debug {
        println!("{} derive JsonForms\n\n{}\n\n", ident, quote);
    }
    Ok(quote)
}

fn expand_props(
    data: &syn::Data,
    span: Span,
    scope: String,
) -> Result<(TokenStream, TokenStream), Vec<syn::Error>> {
    let out;
    let uiout;
    match data {
        syn::Data::Struct(s) => {
            let (outs, uiouts) = expand_struct(s, scope)?;
            out = outs;
            uiout = uiouts;
        }
        syn::Data::Enum(_) => {
            return Err(vec![syn::Error::new(
                span,
                "expand_props: Enum not supported",
            )]);
        }
        syn::Data::Union(_) => {
            return Err(vec![syn::Error::new(
                span,
                "expand_props: Union not supported",
            )]);
        }
    }
    Ok((out, uiout))
}

fn expand_struct(
    s: &syn::DataStruct,
    scope: String,
) -> Result<(TokenStream, TokenStream), Vec<syn::Error>> {
    let mut out_tokens: TokenStream = TokenStream::new();
    let mut uiout_tokens: TokenStream = TokenStream::new();
    let mut out = String::new();
    let mut uiout = String::new();
    let mut sep = false;
    let mut uisep = false;
    let mut has_init_layout = false;
    let mut required = Vec::<&syn::Ident>::new();
    out += r#"{"type":"object","#;
    out += r#""properties":{"#;
    for f in &s.fields {
        let mut skip = false;
        let MyType(f_type, array_type, is_option) = get_type(&f.ty);
        if f_type.is_none() {
            continue;
        }

        let mut end_layout = false;
        let mut schemas: Vec<String> = Vec::new();
        let mut uischemas: Vec<String> = Vec::new();
        let attrs = &f.attrs;
        for attr in attrs {
            if attr.path() != JSONFORMS {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SKIP {
                    skip = true;
                } else if meta.path == SCHEMA {
                    if let Some(lit) = get_lit_str(&meta).unwrap() {
                        schemas.push(lit.value());
                    }
                } else if meta.path == UISCHEMA {
                    if let Some(lit) = get_lit_str(&meta).unwrap() {
                        uischemas.push(lit.value());
                    }
                } else if meta.path == HLAYOUT {
                    if uisep {
                        uiout += ",";
                    }
                    uiout += r#"{"type":"HorizontalLayout","elements":["#;
                    uisep = false;
                    has_init_layout = true;
                } else if meta.path == VLAYOUT {
                    if uisep {
                        uiout += ",";
                    }
                    uiout += r#"{"type":"VerticalLayout","elements":["#;
                    uisep = false;
                    has_init_layout = true;
                } else if meta.path == ELAYOUT {
                    end_layout = true;
                }
                Ok(())
            }) {
                return Err(vec![syn::Error::new(
                    f.span(),
                    format!(
                        "expand_struct: invalid meta parse for jsonforms attribute {:?}",
                        err
                    ),
                )]);
            }
        }
        if skip {
            continue;
        }
        if let Some(id) = &f.ident {
            if sep {
                out += ",";
            } else {
                sep = true;
            }
            out += &format!(r#""{}":"#, id);
            out += r#"{"type":""#;
            let f_type_str = f_type.unwrap();
            out += f_type_str;
            out += "\"";
            for sch in schemas {
                out += ",";
                out += &sch;
            }
            if let Some(arr_type) = array_type {
                out_tokens.extend(quote!(
                    {
                        out_str += #out;
                        out_str += r#","items":"#;
                        let (item_obj, _) = #arr_type::jsonforms_schema();
                        out_str += &item_obj;
                    }
                ));
                out.clear();
            }
            out += "}";

            if uisep {
                uiout += ",";
            } else {
                if !has_init_layout {
                    //default Horizontal Layout if MyType::default() specified
                    uiout += r#"{"type":"VerticalLayout","elements":["#;
                    has_init_layout = true;
                }
                uisep = true;
            }
            uiout += &format!(r#"{{"type":"Control","scope":"{}/{}""#, &scope, id);
            for uisch in uischemas {
                uiout += ",";
                uiout += &uisch;
            }
            uiout += "}";
            if end_layout {
                uiout += "]}";
                uisep = true;
            }
            if !is_option {
                required.push(id);
            }
        }
    }
    out += "}";
    if !required.is_empty() {
        out += r#","required": ["#;
        out += &join(required.iter().map(|id| format!(r#""{}""#, id)), ",");
        out += "]";
    }
    out += "}";
    uiout += "]}";
    out_tokens.extend(quote!( out_str += #out; ));
    uiout_tokens.extend(quote!(uiout_str += #uiout; ));
    Ok((out_tokens, uiout_tokens))
}

#[derive(Default)]
struct MyType<'a>(Option<&'static str>, Option<&'a syn::Type>, bool);

fn get_type(ty: &syn::Type) -> MyType {
    match ty {
        syn::Type::Path(ty) => {
            if let Some(seg) = ty.path.segments.last() {
                let last_name = seg.ident.to_string();
                let arguments = &seg.arguments;
                match last_name.as_str() {
                    "String" => MyType(Some("string"), None, false),
                    "i32" | "u32" | "u64" | "i64" => MyType(Some("integer"), None, false),
                    "f32" | "f64" => MyType(Some("number"), None, false),
                    "bool" => MyType(Some("boolean"), None, false),
                    "Vec" => {
                        match arguments {
                            syn::PathArguments::None => MyType::default(),
                            syn::PathArguments::AngleBracketed(arguments) => {
                                let mut ret: MyType = MyType::default();
                                for arg in &arguments.args {
                                    if let syn::GenericArgument::Type(arg) = arg {
                                        ret = MyType(Some("array"), Some(arg), false);
                                        break;
                                    }
                                }
                                ret
                            }
                            syn::PathArguments::Parenthesized(arguments) => {
                                for _argument in &arguments.inputs {
                                    // self.visit_type(argument);
                                }
                                // self.visit_return_type(&arguments.output);
                                MyType::default()
                            }
                        }
                    }
                    "Option" => {
                        match arguments {
                            syn::PathArguments::None => MyType::default(),
                            syn::PathArguments::AngleBracketed(arguments) => {
                                let mut ret: MyType = MyType::default();
                                for arg in &arguments.args {
                                    if let syn::GenericArgument::Type(arg) = arg {
                                        ret = get_type(arg);
                                        ret.2 = true;
                                        break;
                                    }
                                }
                                ret
                            }
                            syn::PathArguments::Parenthesized(arguments) => {
                                for _argument in &arguments.inputs {
                                    // self.visit_type(argument);
                                }
                                // self.visit_return_type(&arguments.output);
                                MyType::default()
                            }
                        }
                    }
                    _ => MyType::default(),
                }
            } else {
                MyType::default()
            }
        }
        _ => MyType::default(),
    }
}

fn get_lit_str(meta: &ParseNestedMeta) -> syn::Result<Option<syn::LitStr>> {
    let expr: syn::Expr = meta.value()?.parse()?;
    let mut value = &expr;
    while let syn::Expr::Group(e) = value {
        value = &e.expr;
    }
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit),
        ..
    }) = value
    {
        Ok(Some(lit.clone()))
    } else {
        Ok(None)
    }
}
