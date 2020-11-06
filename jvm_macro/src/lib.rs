extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use syn::*;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn jvm_object(metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Deserialize, Debug, Clone, Serialize, RustcEncodable)]
        #input
    };
   
    let mut string_out = output.to_string();
    let jvm_params = metadata.to_string().replace(" ", "");

    let jvm_params: Vec<&str> = jvm_params.split(',').collect();

    let idx = string_out.find(" {").unwrap();
    let string_name = String::from_utf8_lossy(&string_out.as_bytes()[71..idx]);
    let impl_value = r#"
    impl Serializable for {{struct_name}} {
        fn java_class_name (&self) -> String {
            "{{jvm_class}}".to_string()
        }

        fn serial_version_uid(&self) -> u64 {
            {{jvm_uid}}
        }

    }"#;
    string_out.extend(impl_value.replace("{{struct_name}}", &string_name).replace("{{jvm_class}}", &jvm_params[0]).replace("{{jvm_uid}}", &jvm_params[1]).chars());
    proc_macro::TokenStream::from_str(&string_out).unwrap()
}