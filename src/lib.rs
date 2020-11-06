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
        use serde_reflection::{ContainerFormat, Error, Format, Samples, Tracer, TracerConfig};
        use std::collections::HashMap;
        use std::sync::Mutex;
        

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
            bout : Vec<u8>,
            values: HashMap<String, Tracer>
        }

        impl ObjectOutputStream {

            pub fn new() -> ObjectOutputStream {
                return ObjectOutputStream {
                    bout : Vec::new(),
                    values: HashMap::new()
                };
            }

            #[inline]
            pub fn write_object<SER>(&mut self, object: &SER) 
            where SER: Serialize + Any + Serializable, {
                // println!("{:?}", type_of(object));
                // println!("{:?}", type_id(object));

                // if object.type_id() == TypeId::of::<String>() {

                // }

                        let mut cfg = TracerConfig::default().is_human_readable(false)
                                .record_samples_for_newtype_structs(false)
                                .record_samples_for_structs(true)
                                .record_samples_for_tuple_structs(false);
                
                        let mut tracer = Tracer::new(cfg);
                        let mut samples = Samples::new();
                        tracer.trace_value(&mut samples, object).unwrap();

                
                // let registry = tracer.registry().unwrap();

                // println!("{:?}", time.elapsed());
                // println!("{:?}", samples);

                // let time = std::time::Instant::now();
                // let v = serde_json::to_value(object).unwrap();
                // println!("json value {:?}", time.elapsed());
                // println!("json value {:?}", v);


                // println!("{:?}", object.java_class_name());
                // println!("{:?}", object.serial_version_uid());

                

                
            }

        }

        pub struct ObjectInputStream {

        }


    }

}
