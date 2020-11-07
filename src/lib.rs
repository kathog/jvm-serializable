#![feature(type_name_of_val)]
#![feature(core_intrinsics)]
#![feature(get_mut_unchecked)]
#[macro_use]
extern crate serde;
extern crate rustc_serialize;
#[macro_use]
extern crate jvm_macro;
#[macro_use]
extern crate lazy_static;


pub mod java {

    pub mod io {

        
        use serde::{Serialize, Deserialize};
        use serde::Deserializer;
        use std::any::*;
        use serde_reflection::*;
        use std::collections::HashMap;
        use std::sync::{Mutex, Arc};
        use std::fmt::Debug;
        use std::time::Instant;
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
            fn get_field<T: Clone + 'static>(s: &Self, field: &str) -> T;
            fn set_field<T: Clone + 'static>(s: &mut Self, field: &str, val : T);
        }

        lazy_static! {
            static ref traces : Mutex<HashMap<String, Samples>> = Mutex::new(HashMap::new());
        }

        pub struct ObjectOutputStream {
            bout : Vec<u8>,
            values: HashMap<String,  &'static Samples>
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
            where SER: Serialize + Deserialize<'static> + Any + Serializable + Debug + Clone + Encodable, {
                self.write_object0(object);
            
            }

            

            #[inline]
            pub fn write_object0<'a, SER>(&mut self, object: &SER) 
            where SER: Serialize + Deserialize<'a> +  Any + Serializable + Debug + Clone +  Encodable, {
                // println!("{:?}", type_of(object));
                // println!("{:?}", type_id(object));

                // if object.type_id() == TypeId::of::<String>() {

                // }

                // RefCall

                let object_class = type_of(object);
                let mut traces_local = traces.lock().unwrap();
                if let Some(sample) = traces_local.get(object_class) {
                    // println!("from cache {:?}", sample);
                } else {
                    let mut cfg = TracerConfig::default().is_human_readable(false)
                        .record_samples_for_newtype_structs(false)
                        .record_samples_for_structs(true)
                        .record_samples_for_tuple_structs(false);

                    let mut tracer = Tracer::new(cfg);
                    let mut samples = Samples::new();
                    tracer.trace_value(&mut samples, &object).unwrap();

                    let registry = tracer.registry().unwrap();

                    for s in registry {
                        match s.1 {
                            ContainerFormat::Struct(vals) => {
                                for n in vals {
                                    // println!("{:?}", n.value);

                                    // object.get_field(object, n.name);
                                }
                            },
                            _ => {}
                        }

                    }

                    traces_local.insert(object_class.to_owned(), samples);
                }
                // let traces_local = unsafe {Arc::get_mut_unchecked(&mut traces_clone)};



                // let time = Instant::now();
                // let mut json = "".to_owned();
                // {
                //     let mut encoder = Encoder::new(&mut json);
                //     object.encode(&mut encoder).unwrap();
                // }
                //
                // let json = Json::from_str(&json).unwrap();
                // if let Object(object) = json {
                //     let field_names: Vec<_> = object.values().collect();
                //     println!("{:?}", field_names);
                // }
                // println!("{:?}", time.elapsed());
                
                // println!("{:?}", SER::members());
                // let time = Instant::now();
                // let schema = SER::schemata();
                // println!("{:?}", time.elapsed());


                
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
