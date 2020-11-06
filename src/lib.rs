#![feature(type_name_of_val)]
#[macro_use]
extern crate serde;
extern crate rustc_serialize;
#[macro_use]
extern crate jvm_macro;

pub mod java {

    pub mod io {

        
        use serde::Serialize;
        use std::any::*;
        use rustc_serialize::json::{Encoder, Json};
        use rustc_serialize::json::Json::Object;
        use rustc_serialize::Encodable;
        

        fn type_of<T>(o: &T) -> &'static str {
            type_name_of_val(o)
        }

        fn type_id<T: ?Sized + Any>(_s: &T) -> TypeId {
            TypeId::of::<T>().clone()
        }
        

        pub trait Serializable {
            fn java_class_name (&self) -> String;
            fn serial_version_uid(&self) -> u64;
        }

        pub struct ObjectOutputStream {
            bout : Vec<u8>

        }

        impl ObjectOutputStream {

            pub fn new() -> ObjectOutputStream {
                return ObjectOutputStream {
                    bout : Vec::new()
                };
            }

            pub fn write_object<SER>(&self, object: &SER) 
            where SER: Serialize + Any + Encodable + Serializable, {
                println!("{:?}", type_of(object));
                println!("{:?}", type_id(object));

                let mut json = "".to_owned();
                {
                    let mut encoder = Encoder::new(&mut json);
                    object.encode(&mut encoder).unwrap();
                }
                let json = Json::from_str(&json).unwrap();
                if let Object(object) = json {
                    let field_names: Vec<_> = object.keys().collect();
                    println!("{:?}", field_names);
                }

                println!("{:?}", object.java_class_name());
                println!("{:?}", object.serial_version_uid());

            }

        }

        pub struct ObjectInputStream {

        }


    }

}
