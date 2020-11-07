#![feature(type_name_of_val)]
#![feature(core_intrinsics)]
#![feature(get_mut_unchecked)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;
#[macro_use]
extern crate lazy_static;


pub mod java {

    pub mod io {

        
        use serde::{Serialize, Deserialize};
        use serde::Deserializer;
        use std::any::*;
        use std::collections::HashMap;
        use std::sync::{Mutex, Arc};
        use std::fmt::Debug;
        use std::time::Instant;


        fn type_of<T>(o: &T) -> &'static str {
            type_name_of_val(o)
        }

        fn type_id<T: ?Sized + Any>(_s: &T) -> TypeId {
            TypeId::of::<T>().clone()
        }
        pub trait Serializable {
            fn java_class_name (&self) -> String;
            fn serial_version_uid(&self) -> u64;
            fn get_field<T: Clone + 'static>(s: &Self, field: &str) -> T;
            fn set_field<T: Clone + 'static>(s: &mut Self, field: &str, val : T);
            fn get_fields() -> std::collections::HashMap<String, String>;
        }

        pub struct ObjectOutputStream {
            bout : Vec<u8>
        }

        impl ObjectOutputStream {

            pub fn new() -> ObjectOutputStream {
                return ObjectOutputStream {
                    bout : Vec::new(),
                };
            }

            #[inline]
            pub fn write_object<SER>(&mut self, object: &SER) 
            where SER: Serialize + Deserialize<'static> + Any + Serializable + Debug + Clone {
                self.write_object0(object);
            
            }

            

            #[inline]
            pub fn write_object0<'a, SER>(&mut self, object: &SER) 
            where SER: Serialize + Deserialize<'a> +  Any + Serializable + Debug + Clone  {
                // println!("{:?}", type_of(object));
                // println!("{:?}", type_id(object));

                // if object.type_id() == TypeId::of::<String>() {

                // }

                let object_class = type_of(object);

                println!("{:?}", SER::get_fields());
            }

        }

        pub struct ObjectInputStream {

        }



        
    }

}
