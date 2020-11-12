extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate serde_derive_internals;
#[macro_use]
extern crate serde;

use proc_macro2::TokenStream;
use std::str::FromStr;
use std::collections::HashMap;
use proc_macro2::TokenTree;

use serde_derive_internals::*;
use syn::*;
use std::any::Any;
use std::path::Prefix::Verbatim;

#[proc_macro_attribute]
pub fn jvm_object(metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let input: TokenStream = input.clone().into();

    let tokens = input.clone().into_iter().peekable();
    let mut struct_props = Vec::new();
    let mut inner_types = vec![];

    for t in tokens {

        match t{
            TokenTree::Group(struct_body) => {
                let mut struct_iter = struct_body.stream().into_iter();
                let mut field_name = String::new();
                let mut field_type = String::new();
                loop {
                    let struct_body_token =  match struct_iter.next() {
                        None => break,
                        Some(t) => t,
                    };
                    match &struct_body_token {
                        TokenTree::Ident(ident) => {
                            field_name = ident.to_string();
                        },
                        TokenTree::Punct(punct) => {
                            if punct.as_char() == ':'{
                                let mut brak_ref = 0;
                                loop {
                                    let line_token = match struct_iter.next() {
                                        None => {
                                            if field_type.replace(" ", "").len() > 3 && field_type.trim_start().ne("String") {
                                                inner_types.push((field_name.clone().replace(" ", ""), field_type.clone().replace(" ", "")));
                                            }
                                            struct_props.push ((field_name.clone(), field_type.clone().replace(" ", "")));
                                            break
                                        },
                                        Some(t) => t,
                                    };

                                    match &line_token{
                                        TokenTree::Punct(line_p) => {
                                            match line_p.as_char() {
                                                ',' | '}' => {
                                                    if brak_ref == 0 {
                                                        if field_type.replace(" ", "").len() > 3 && field_type.trim_start().ne("String") {
                                                            inner_types.push((field_name.clone().replace(" ", ""), field_type.replace(" ", "")));
                                                        }
                                                        struct_props.push((field_name, field_type.replace(" ", "")));
                                                        field_name = String::new();
                                                        field_type = String::new();
                                                        break;
                                                    }
                                                },
                                                '<' => { brak_ref +=1; },
                                                '>' => { brak_ref-=1; },
                                                _ => {},
                                            }
                                        },
                                        TokenTree::Ident(_) => {
                                            if !field_type.ends_with('\'') {
                                                field_type.push(' ');
                                            }
                                        },
                                        _ => {},
                                    }
                                    field_type.push_str(&line_token.to_string());
                                }
                            }
                            continue;
                        },
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    let struct_name = input.clone().into_iter().filter(|x| match x {
            TokenTree::Ident(_) => true,
            _ => false,
        }).last().unwrap().to_string();

    let output = quote! {
        #[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
        #input
    }; 
    let mut out_as_string = output.to_string();
    let jvm_params = metadata.to_string().replace(" ", "");
    let jvm_params: Vec<&str> = jvm_params.split(',').collect();

    let mut get_body = String::new();
    get_body.push_str("{ match field \n\r{");
    for (field, _datatype) in &struct_props {
        get_body.push_str(&format!(
            "\"{field}\" => &(s.{field}) as &dyn std::any::Any,\n\r",
            field=field
        ));
    }
    get_body.push_str("_ => panic!(\"Invalid field.\"), }}");

    let mut set_body = String::new();
    set_body.push_str("match field {");
    for (field,datatype) in &struct_props {
        set_body.push_str(&format!("
            \"{field}\" => {{ s.{field} = ((&val as &dyn std::any::Any).downcast_ref::<{datatype}>().unwrap()).to_owned(); }},\r\n",
                                   field=field,
                                   datatype=datatype
        ));

    }
    set_body.push_str("_ => { panic!(\"invalid field\"); } }");


    println!("{:?}", struct_props);

    let mut struct_props_as_string = String::from("[");
    let mut idx = 0;
    for (key, value) in &struct_props {
        struct_props_as_string.push_str(&format!("(\"{}\".to_string(), \"{}\".to_string(), {}),", key, value, idx));
        idx += 1;
    }
    struct_props_as_string.remove(struct_props_as_string.len()-1);
    struct_props_as_string.push_str("].iter().cloned().collect()");

    let mut items = String::new();
    let mut string_of_get_items = String::new();
    let mut count = 1;
    for inner in inner_types {
        string_of_get_items.push_str(&format!(" fn get_item{}(&self) -> Option<&Self::Item{}> ", count, count));
        string_of_get_items.push_str("{");
        string_of_get_items.push_str(&format!(" Some(&self.{}) ", inner.0));
        string_of_get_items.push_str("}\n");

        items.push_str(&format!(" type Item{} = {};\n ", count, inner.1));

        count += 1;
    }
    for i in count..6 {
        string_of_get_items.push_str(&format!(" fn get_item{}(&self) -> Option<&Self::Item{}> ", i, i));
        string_of_get_items.push_str("{ None }\n");
        items.push_str(&format!(" type Item{} = {}; \n ", i, &struct_name));
    }


    let impl_value = r#"
    impl Serializable for {{struct_name}} {

        {{items}}

        #[inline]
        fn java_class_name (&self) -> String {
            "{{jvm_class}}".to_string()
        }

        #[inline]
        fn serial_version_uid(&self) -> u64 {
            {{jvm_uid}}
        }

        fn get_field<T: std::any::Any + Clone + 'static>(s: &Self, field: &str) -> T {{
            let a : &dyn std::any::Any = {{get_body}};
            (a.downcast_ref::<T>().unwrap().clone())
        }}

        fn set_field<T: std::any::Any + Clone + 'static>(s: &mut Self, field: &str, val : T) {{
            {{set_body}}
        }}

        fn get_fields(&self) -> Vec<(String, String, i32)> {
            {{fields_string}}
        }

        {{get_items}}

    }"#;
    out_as_string.push_str(&impl_value.replace("{{struct_name}}", &struct_name)
        .replace("{{jvm_class}}", &jvm_params[0])
        .replace("{{jvm_uid}}", &jvm_params[1])
        .replace("{{set_body}}", &set_body)
        .replace("{{get_body}}", &get_body)
        .replace("{{fields_string}}", &struct_props_as_string)
        .replace("{{get_items}}", &string_of_get_items)
        .replace("{{items}}", &items)
    );
    println!("{:?}", out_as_string);
    proc_macro::TokenStream::from_str(&out_as_string).unwrap()
}