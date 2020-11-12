#![feature(type_name_of_val)]
#![feature(core_intrinsics)]
#![feature(get_mut_unchecked)]
#![feature(optin_builtin_traits)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate traitcast;


pub mod java {

    pub mod io {

        
        use serde::{Serialize, Deserialize, Serializer};
        use serde::Deserializer;
        use std::any::*;
        use std::collections::HashMap;
        use std::sync::{Mutex, Arc};
        use std::fmt::{Debug, Display};
        use std::time::Instant;
        use serde_json::ser::State;
        use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStructVariant, SerializeStruct};
        use serde::export::{Formatter, TryFrom};
        use std::fmt;
        use std::sync::atomic::{AtomicBool, Ordering};




        fn type_of<T>(o: &T) -> &'static str {
            type_name_of_val(o)
        }

        fn type_id<T: ?Sized + Any>(_s: &T) -> TypeId {
            TypeId::of::<T>().clone()
        }

        pub trait Serializable  {

            type Item1 : Serializable + Debug;
            type Item2 : Serializable + Debug;
            type Item3 : Serializable + Debug;
            type Item4 : Serializable + Debug;
            type Item5 : Serializable + Debug;

            fn java_class_name (&self) -> String;
            fn serial_version_uid(&self) -> u64;
            fn get_field<T: Any + Clone + 'static>(s: &Self, field: &str) -> T;
            fn set_field<T: Any + Clone + 'static>(s: &mut Self, field: &str, val : T);
            fn get_fields(&self) -> Vec<(String, String, i32)>;
            fn get_item1(&self) -> Option<&Self::Item1>;
            fn get_item2(&self) -> Option<&Self::Item2>;
            fn get_item3(&self) -> Option<&Self::Item3>;
            fn get_item4(&self) -> Option<&Self::Item4>;
            fn get_item5(&self) -> Option<&Self::Item5>;

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
            pub fn write_object<'a, SER>(&mut self, object: &SER)
            where SER:  Any + Serialize + Deserialize<'a> + Debug + Clone + Serializable {
                self.write_object0(object);
            
            }

            

            #[inline]
            pub fn write_object0<'a, SER>(&mut self, object: &SER) 
            where SER:   Any + Serialize + Deserialize<'a> + Debug + Clone + Serializable {

                let mut jvm_ser = JvmSerializer {
                    buf: Vec::with_capacity(1024),
                    inner: AtomicBool::new(false),
                    value_buf: Vec::with_capacity(1024),
                    metadata_structs : HashMap::new()
                };

                jvm_ser.build_metadata(Some(object));
                jvm_ser.write_head(object);
                object.serialize(&mut jvm_ser);

                // println!("{:?}", String::from_utf8_lossy(&jvm_ser.buff()));
            }

        }

        pub struct ObjectInputStream {

        }
        pub struct Compound<'a> {
            ser: &'a mut JvmSerializer,
        }


        impl <'a>SerializeSeq for Compound<'a> {
            type Ok = ();
            type Error = Error;

            fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }
        }

        impl <'a>SerializeTuple for Compound<'a> {
            type Ok = ();
            type Error = Error;

            fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }
        }

        impl <'a>SerializeTupleStruct for Compound<'a> {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }
        }

        impl <'a>SerializeTupleVariant for Compound<'a> {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }
        }

        impl <'a>SerializeMap for Compound<'a> {
            type Ok = ();
            type Error = Error;

            fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }
        }

        impl <'a>SerializeStruct for Compound<'a> {
            type Ok = ();
            type Error = Error;


            #[inline]
            fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
                T: ?Sized + Serialize {

                let class_name = type_name_of_val(value);
                let jvm_data = self.ser.metadata_structs.get(class_name);
                match jvm_data {
                    Some(data) => {
                        self.ser.buf.push(115); //TC_OBJECT
                        self.ser.buf.extend_from_slice(&(data.0.len() as i16).to_be_bytes());
                        self.ser.buf.extend_from_slice(data.0.as_bytes());

                        self.ser.buf.extend_from_slice(&data.1.to_be_bytes());
                        self.ser.buf.push(2); // flagi
                        self.ser.buf.extend_from_slice(&data.2.to_be_bytes());

                        self.ser.inner.store(true, Ordering::SeqCst);
                    },
                    None => {

                        match class_name {
                            "i32" => {
                                if self.ser.inner.load(Ordering::SeqCst) {
                                    self.ser.buf.push(73 as u8);
                                    self.ser.buf.extend_from_slice(&(key.len() as i16).to_be_bytes());
                                    self.ser.buf.extend_from_slice(key.as_bytes());
                                }
                            }
                            "alloc::string::String" => {
                                if self.ser.inner.load(Ordering::SeqCst) {
                                    //field type - String
                                    self.ser.buf.push(76);
                                    self.ser.buf.extend_from_slice(&(key.len() as i16).to_be_bytes());
                                    self.ser.buf.extend_from_slice(key.as_bytes());
                                }
                            }
                            _ => {

                            }
                        }

                    }
                }
                value.serialize(&mut *self.ser)
            }

            #[inline]
            fn end(self) -> Result<Self::Ok, Self::Error> {
                if self.ser.inner.load(Ordering::SeqCst) {
                    self.ser.buf.push(113);
                    self.ser.buf.push(0);
                    self.ser.buf.push(126);
                    self.ser.buf.push(0);
                    self.ser.buf.push(1);
                    self.ser.buf.push(120); //TC_ENDBLOCKDATA
                    self.ser.buf.push(112); //TC_NULL
                    self.ser.inner.store(false, Ordering::SeqCst);
                }
                self.ser.buf.extend(self.ser.value_buf.iter());
                self.ser.value_buf.clear();
                Ok(())
            }
        }

        impl <'a>SerializeStructVariant for Compound<'a> {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }
        }

        pub struct JvmSerializer {

            buf: Vec<u8>,
            inner: AtomicBool,
            value_buf : Vec<u8>,
                                //simple name, (full name, serialuid, num of fields)
            metadata_structs : HashMap<String, (String, i64, i16)>
        }

        impl JvmSerializer {

            pub fn buff(&self) -> Vec<u8> {
                self.buf.clone()
            }


            fn build_metadata<T>(&mut self, ob: Option<&T>) where T: Serializable + Debug {
                match ob {
                    Some(object) => {
                        let name = type_name_of_val(object);
                        let uid = object.serial_version_uid();
                        let jvm_name = object.java_class_name();
                        let count = object.get_fields().len();
                        let idx = name.rfind("::").unwrap();
                        self.metadata_structs.insert(name.to_owned(), (jvm_name.clone(), uid as i64, count as i16));
                        self.metadata_structs.insert(name[idx+2..].to_owned(), (jvm_name, uid as i64, count as i16));
                        self.build_metadata(object.get_item1());
                        self.build_metadata(object.get_item2());
                        self.build_metadata(object.get_item3());
                        self.build_metadata(object.get_item4());
                        self.build_metadata(object.get_item5());
                    },
                    None => {}
                }
            }

            /*

            \u{0}��\u{0}\u{5}sr\u{0}5io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1}\u{2}\u{0}\u{2}L\u{0}\u{6}nodeIdt\u{0}\u{12}Ljava/lang/String;L\u{0}\u{8}serverID
            t\u{0}!Lio/vertx/core/net/impl/ServerID;xpt\u{0}$5e9c652c-2409-45d5-b598-483b32967e4asr\u{0}\u{1f}io.vertx.core.net.impl.ServerIDN9\u{3}�g\u{1c}\u{11}�\u{2}\u{0}\u{2}I\u{0}\u{4}portL\u{0}\u{4}hostq\u{0}~\u{0}\u{1}xp\u{0}\u{0}��t\u{0}\tlocalhost



            \u{0}��\u{0}\u{5}sr\u{0}5io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1}\u{2}\u{0}\u{2}L\u{0}\u{6}nodeIdt\u{0}\u{12}Ljava/lang/String;L\u{0}\u{8}serverID
            t\u{0}!Lio/vertx/core/net/impl/ServerID;xpt\u{0}$75d6fa4d-b312-4c80-8718-d5fbc05c0f53s\u{0}\u{1f}io.vertx.core.net.impl.ServerIDN9\u{3}�g\u{1c}\u{11}�\u{2}\u{0}\u{2}I\u{0}\u{4}portL\u{0}\u{4}hostq\u{0}~\u{0}\u{1}xp\u{0}\u{0}��t\u{0}\tlocalhost





             * <blockquote><pre>
             * B            byte
             * C            char
             * D            double
             * F            float
             * I            int
             * J            long
             * L            class or interface
             * S            short
             * Z            boolean
             * [            array
             * </pre></blockquote>
             */

            pub fn write_head<SER>(&mut self, ob : &SER)
            where SER: Serializable {

                self.buf.push(0); // flaga
                self.buf.extend_from_slice(&(-21267 as i16).to_be_bytes()); //STREAM_MAGIC
                self.buf.extend_from_slice(&(5 as i16).to_be_bytes()); //STREAM_MAGIC
                self.buf.push(115); //TC_OBJECT
                self.buf.push(114); //TC_CLASSDESC
                // class_name
                self.buf.extend_from_slice(&(ob.java_class_name().len() as i16).to_be_bytes());
                self.buf.extend_from_slice(ob.java_class_name().as_bytes());
                //serialVersionUID
                self.buf.extend_from_slice(&ob.serial_version_uid().to_be_bytes());
                //class flags
                self.buf.push(0|2);
                //fields len
                let fields = ob.get_fields();
                self.buf.extend_from_slice(&(fields.len() as i16).to_be_bytes());
                //fields
                for (name, type_, idx) in fields {
                    let mut jvm_type_name = String::new();
                    match type_.as_str() {
                        "i32" => {
                            self.buf.push('I' as u8);
                        },
                        "String" => {
                            self.buf.push('L' as u8);
                            jvm_type_name.push_str("Ljava/lang/String;")
                        }
                        _ => {
                            self.buf.push('L' as u8);
                            jvm_type_name.push('L');
                            let jvm_data = self.metadata_structs.get(&type_);
                            match jvm_data {
                                Some(data) => {
                                    jvm_type_name.push_str(&data.0.replace(".", "/"));
                                },
                                None => {}
                            }

                            jvm_type_name.push(';');
                        }
                    };
                    self.buf.extend_from_slice(&(name.len() as i16).to_be_bytes());
                    self.buf.extend_from_slice(name.as_bytes());
                    if !jvm_type_name.is_empty() {
                        self.buf.push(116); //TC_STRING
                        self.buf.extend_from_slice(&(jvm_type_name.len() as i16).to_be_bytes());
                        self.buf.extend_from_slice(jvm_type_name.as_bytes());
                    }
                }
                self.buf.push(120); //TC_ENDBLOCKDATA
                self.buf.push(112); // TC_NULL
            }

        }



        impl<'a> Serializer for &'a mut JvmSerializer {
            type Ok = ();
            type Error = Error;
            type SerializeSeq = Compound<'a>;
            type SerializeTuple = Compound<'a>;
            type SerializeTupleStruct = Compound<'a>;
            type SerializeTupleVariant = Compound<'a>;
            type SerializeMap = Compound<'a>;
            type SerializeStruct = Compound<'a>;
            type SerializeStructVariant = Compound<'a>;

            #[inline]
            fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
                if v {
                    self.buf.push(1);
                } else {
                    self.buf.push(0);
                }
                Ok(())
            }

            #[inline]
            fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
                if self.inner.load(Ordering::SeqCst) {
                    self.value_buf.extend_from_slice(&v.to_be_bytes());
                } else {
                    self.buf.extend_from_slice(&v.to_be_bytes());
                }
                Ok(())
            }

            #[inline]
            fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
                self.buf.push(v as u8);
                Ok(())
            }

            #[inline]
            fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
                if self.inner.load(Ordering::SeqCst) {
                    self.value_buf.push(116);//TC_STRING
                    self.value_buf.extend_from_slice(&(v.len() as i16).to_be_bytes());
                    self.value_buf.extend_from_slice(v.as_bytes());
                } else {
                    self.buf.push(116);//TC_STRING
                    self.buf.extend_from_slice(&(v.len() as i16).to_be_bytes());
                    self.buf.extend_from_slice(v.as_bytes());
                }
                Ok(())
            }

            #[inline]
            fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(v);
                Ok(())
            }

            #[inline]
            fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            #[inline]
            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            #[inline]
            fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
                T: Serialize {
                unimplemented!()
            }

            #[inline]
            fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
                T: Display {
                unimplemented!()
            }
        }

        #[derive(Debug)]
        pub struct Error {
            err: Box<ErrorImpl>,
        }

        #[derive(Debug)]
        struct ErrorImpl {
            line: usize,
            column: usize,
        }

        impl Display for ErrorImpl {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // if self.line == 0 {
                //     Display::fmt(&self.code, f)
                // } else {
                    write!(f, "Error at line {} column {}", self.line, self.column)
                // }
            }
        }

        impl Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Display::fmt(&*self.err, f)
            }
        }

        impl serde::ser::StdError for Error {

        }

        impl serde::ser::Error for Error {

            fn custom<T>(msg: T) -> Self where
                T: Display {
                unimplemented!()
            }
        }
        
    }
}

