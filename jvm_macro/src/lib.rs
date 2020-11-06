extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;


use proc_macro2::TokenStream;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn jvm_object(metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
        #input
    }; 
    let mut out_as_string = output.to_string();
    let jvm_params = metadata.to_string().replace(" ", "");
    let jvm_params: Vec<&str> = jvm_params.split(',').collect();

    // println!("{:?}", "#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)] struct\n".len());

    let idx = out_as_string.find("{").unwrap();
    let name_of_struct = String::from_utf8_lossy(&out_as_string.as_bytes()[70..idx]);
    let impl_value = r#"
    impl Serializable for {{struct_name}} {
        #[inline]
        fn java_class_name (&self) -> String {
            "{{jvm_class}}".to_string()
        }

        #[inline]
        fn serial_version_uid(&self) -> u64 {
            {{jvm_uid}}
        }

    }"#;
    out_as_string = out_as_string.clone() + &impl_value.replace("{{struct_name}}", &name_of_struct).replace("{{jvm_class}}", &jvm_params[0]).replace("{{jvm_uid}}", &jvm_params[1]);
    proc_macro::TokenStream::from_str(&out_as_string).unwrap()
}