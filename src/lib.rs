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

pub mod java {

    pub mod io {

        
        use serde::{Serialize, Deserialize, Serializer};
        use serde::Deserializer;
        use std::any::*;
        use std::collections::HashMap;
        use std::sync::{Mutex, Arc};
        use std::fmt::{Debug, Display};
        use std::time::Instant;
        use traitcast::TraitcastFrom;
        use serde_json::ser::State;
        use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStructVariant, SerializeStruct};
        use serde::export::Formatter;
        use std::fmt;
        use std::sync::atomic::{AtomicBool, Ordering};


        fn type_of<T>(o: &T) -> &'static str {
            type_name_of_val(o)
        }

        fn type_id<T: ?Sized + Any>(_s: &T) -> TypeId {
            TypeId::of::<T>().clone()
        }

        pub trait Serializable  {
            fn java_class_name (&self) -> String;
            fn serial_version_uid(&self) -> u64;
            fn get_field<T: Any + Clone + 'static>(s: &Self, field: &str) -> T;
            fn set_field<T: Any + Clone + 'static>(s: &mut Self, field: &str, val : T);
            fn get_fields() -> std::collections::HashMap<String, String>;
            fn get_field_as_value(s: &Self, field: &str) -> serde_json::Value;
            // fn serialize(&self) -> Vec<u8>;
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
                    buf: vec![],
                    inner: AtomicBool::new(false)
                };

                jvm_ser.write_head(object);
                // object.serialize(&mut jvm_ser);

                let set_data = serde_json::to_value(object);
                // println!("{:?}", set_data);

                // println!("{:?}", jvm_ser.buff());
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

                let mut jvm_class_name = String::new();
                let class_name = type_name_of_val(value);

                if !class_name.starts_with("alloc") {
                    let idx = class_name.find("::");
                    match idx {
                        Some(idx0) => {
                            jvm_class_name.push_str(&class_name[idx0+2..].replace("::", ".").replace("_impl", "impl"));
                        }
                        None => {
                            if self.ser.inner.load(Ordering::SeqCst) {

                                match class_name {
                                    "i32" => {
                                        self.ser.buf.push(73);
                                    }
                                    _ => {

                                    }
                                }

                                //field type
                                self.ser.buf.extend_from_slice(&(key.len() as i16).to_be_bytes());
                                self.ser.buf.extend_from_slice(key.as_bytes());
                            }
                        }
                    }

                } else {
                    if self.ser.inner.load(Ordering::SeqCst) {
                        //field type
                        self.ser.buf.push(76);
                        self.ser.buf.extend_from_slice(&(key.len() as i16).to_be_bytes());
                        self.ser.buf.extend_from_slice(key.as_bytes());
                    }

                }

                if !jvm_class_name.is_empty() {
                    self.ser.buf.push(115); //TC_OBJECT
                    self.ser.buf.extend_from_slice(&(jvm_class_name.len() as i16).to_be_bytes());
                    self.ser.buf.extend_from_slice(jvm_class_name.as_bytes());


                    //todo static to change
                    self.ser.buf.extend_from_slice(&(5636540499169644934 as i64).to_be_bytes());
                    self.ser.buf.push(2); // flagi
                    self.ser.buf.extend_from_slice(&(2 as i16).to_be_bytes()); // liczba pol


                    self.ser.inner.store(true, Ordering::SeqCst);
                }

                value.serialize(&mut *self.ser)
            }

            #[inline]
            fn end(self) -> Result<Self::Ok, Self::Error> {
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
        }

        impl JvmSerializer {

            pub fn buff(&self) -> Vec<u8> {
                self.buf.clone()
            }

            /*
            [0, 172, 237, 0, 5, 115, 114, 0, 53, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 101, 118, 101, 110, 116, 98, 117, 115, 46, 105, 109, 112, 108, 46, 99, 108, 117, 115, 116, 101, 114, 101, 100, 46, 67, 108, 117, 115, 116, 101, 114, 78, 111, 100, 101, 73, 110, 102, 111,
            0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 76, 0, 6, 110, 111, 100, 101, 73, 100, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 76, 0, 8, 115, 101, 114, 118, 101, 114, 73, 68, 116, 0, 33, 76, 105, 111, 47, 118, 101, 114, 116, 120, 47, 99, 111, 114, 101,
            47, 110, 101, 116, 47, 105, 109, 112, 108, 47, 83, 101, 114, 118, 101, 114, 73, 68, 59, 120, 112, 116, 0, 36, 53, 101, 57, 99, 54, 53, 50, 99, 45, 50, 52, 48, 57, 45, 52, 53, 100, 53, 45, 98, 53, 57, 56, 45, 52, 56, 51, 98, 51, 50, 57, 54, 55, 101, 52, 97, 115, 114, 0, 31, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 110, 101, 116, 46, 105, 109, 112, 108, 46, 83, 101, 114, 118, 101, 114, 73, 68, 78, 57, 3, 184, 103, 28, 17, 134, 2, 0, 2, 73, 0, 4, 112, 111, 114, 116, 76, 0, 4, 104, 111, 115, 116, 113, 0, 126, 0, 1, 120, 112, 0, 0, 178, 241, 116, 0, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116]

            \u{0}��\u{0}\u{5}sr\u{0}5io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1}\u{2}\u{0}\u{2}L\u{0}\u{6}nodeIdt\u{0}\u{12}Ljava/lang/String;L\u{0}\u{8}serverID
            t\u{0}!Lio/vertx/core/net/impl/ServerID;xpt\u{0}$5e9c652c-2409-45d5-b598-483b32967e4asr\u{0}\u{1f}io.vertx.core.net.impl.ServerIDN9\u{3}�g\u{1c}\u{11}�\u{2}\u{0}\u{2}I\u{0}\u{4}portL\u{0}\u{4}hostq\u{0}~\u{0}\u{1}xp\u{0}\u{0}��t\u{0}\tlocalhost

            [0, 172, 237, 0, 5, 115, 114, 0, 53, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 101, 118, 101, 110, 116, 98, 117, 115, 46, 105, 109, 112, 108, 46, 99, 108, 117, 115, 116, 101, 114, 101, 100, 46, 67, 108, 117, 115, 116, 101, 114, 78, 111, 100, 101, 73, 110, 102, 111,
            0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 76, 0, 6, 110, 111, 100, 101, 73, 100, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 76, 0, 8, 115, 101, 114, 118, 101, 114, 73, 68, 116, 0, 33, 76, 105, 111, 47, 118, 101, 114, 116, 120, 47, 99, 111, 114, 101,
            47, 110, 101, 116, 47, 105, 109, 112, 108, 47, 83, 101, 114, 118, 101, 114, 73, 68, 59, 120, 112, 116, 0, 36, 54, 49, 97, 97, 101, 54, 98, 49, 45, 57, 48, 100, 54, 45, 52, 52, 48, 54, 45, 56, 101, 57, 55, 45, 48, 53, 97, 102, 101, 99, 101, 51, 99, 48, 100, 50, 115, 0, 31, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 110, 101, 116, 46, 105, 109, 112, 108, 46, 83, 101, 114, 118, 101, 114, 73, 68, 0, 0, 175, 200, 116, 0, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116]


            \u{0}��\u{0}\u{5}sr\u{0}5io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{1}\u{2}\u{0}\u{2}L\u{0}\u{6}nodeIdt\u{0}\u{12}Ljava/lang/String;L\u{0}\u{8}serverID
            t\u{0}!Lio/vertx/core/net/impl/ServerID;xpt\u{0}$63f3dd04-6c9c-4686-856d-64312eae657as\u{0}\u{1f}io.vertx.core.net.impl.ServerIDN9\u{3}�g\u{1c}\u{11}�\u{2}\u{0}\u{2}I\u{0}\u{4}port\u{0}\u{0}��L\u{0}\u{4}hostt\u{0}\tlocalhost
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
                self.buf.push((0|2));
                //fields len
                let fields = SER::get_fields();
                self.buf.extend_from_slice(&(fields.len() as i16).to_be_bytes());
                //fields
                for (name, type_) in fields {
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
                            jvm_type_name.push_str(&type_.replace(":: ", "/").replace("_impl", "impl").trim());
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
                // self.buf.push(115); //TC_CLASS

                // println!("{:?}", );


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
                unimplemented!()
            }

            #[inline]
            fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
                self.buf.extend_from_slice(&v.to_be_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
            }

            #[inline]
            fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
                self.buf.push(116);//TC_STRING
                self.buf.extend_from_slice(&(v.len() as i16).to_be_bytes());
                self.buf.extend_from_slice(v.as_bytes());
                Ok(())
            }

            #[inline]
            fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
                unimplemented!()
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

