#![feature(type_name_of_val)]
#![feature(core_intrinsics)]
#![feature(get_mut_unchecked)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;

pub mod java {

    pub mod io {

        
        use serde::{Serialize, Deserialize, Serializer};
        use serde::Deserializer;
        use std::any::*;
        use std::collections::HashMap;
        use std::sync::{Mutex, Arc};
        use std::fmt::{Debug, Display};
        use std::time::Instant;
        use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStructVariant, SerializeStruct};
        use std::fmt;
        use std::sync::atomic::{AtomicBool, Ordering};
        use serde::de::Visitor;
        use std::convert::TryInto;


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
            pub fn write_object<SER>(&mut self, object: &SER)
            where SER: Any + Serialize + Debug + Clone + Serializable {
                self.write_object0(object);
            }

            #[inline]
            pub fn write_object0<SER>(&mut self, object: &SER)
            where SER: Any + Serialize + Debug + Clone + Serializable {


                let size = bincode::serialized_size(object).unwrap();

                let mut jvm_ser = JvmSerializer {
                    buf: Vec::with_capacity(size as usize),
                    inner: false,
                    value_buf: Vec::with_capacity(size as usize),
                    metadata_structs : HashMap::new(),
                    read_idx: 0,
                    inner_is_object: false,
                };

                jvm_ser.build_metadata(Some(object));
                jvm_ser.write_head(object);
                let _ = object.serialize(&mut jvm_ser);

                self.bout = jvm_ser.buf;
            }

            pub fn to_byte_array(&self) -> Vec<u8> {
                self.bout.clone()
            }

        }

        pub struct ObjectInputStream {

        }

        impl ObjectInputStream {
            #[inline]
            pub fn read_object<'a, SER>(&mut self, data: Vec<u8>) -> SER
                where SER:  Any + Serialize + Deserialize<'a> + Debug + Clone + Serializable + Default {

                let mut jvm_ser = JvmSerializer {
                    buf: data,
                    inner: false,
                    value_buf: Vec::with_capacity(0),
                    metadata_structs : HashMap::new(),
                    read_idx: 0,
                    inner_is_object: false,
                };

                jvm_ser.read_head::<SER>();
                return SER::deserialize(&mut jvm_ser).unwrap();
            }
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

                        if data.0 != "java.lang.Object" {
                            self.ser.buf.push(115); //TC_OBJECT
                            self.ser.buf.push(114);
                            self.ser.buf.extend_from_slice(&(data.0.len() as i16).to_be_bytes());
                            self.ser.buf.extend_from_slice(data.0.as_bytes());

                            self.ser.buf.extend_from_slice(&data.1.to_be_bytes());
                            self.ser.buf.push(2); // flagi
                            self.ser.buf.extend_from_slice(&data.2.to_be_bytes());
                        }

                        self.ser.inner = true;
                    },
                    None => {

                        match class_name {
                            "i32" => {
                                if self.ser.inner {
                                    self.ser.buf.push(73 as u8);
                                    self.ser.buf.extend_from_slice(&(key.len() as i16).to_be_bytes());
                                    self.ser.buf.extend_from_slice(key.as_bytes());
                                }
                            }
                            "alloc::string::String" => {
                                if self.ser.inner {
                                    //field type - String
                                    if !self.ser.inner_is_object {
                                        self.ser.buf.push(76);
                                        self.ser.buf.extend_from_slice(&(key.len() as i16).to_be_bytes());
                                        self.ser.buf.extend_from_slice(key.as_bytes());
                                    } else {
                                        self.ser.inner_is_object = false;
                                    }


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
                if self.ser.inner {
                    self.ser.buf.push(113);
                    self.ser.buf.push(0);
                    self.ser.buf.push(126);
                    self.ser.buf.push(0);
                    self.ser.buf.push(1);
                    self.ser.buf.push(120); //TC_ENDBLOCKDATA
                    self.ser.buf.push(112); //TC_NULL
                    self.ser.inner = false;
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
            inner: bool,
            value_buf : Vec<u8>,
            inner_is_object: bool,
                                //simple name, (full name, serialuid, num of fields)
            metadata_structs : HashMap<String, (String, i64, i16)>,
            read_idx: usize

        }

        impl JvmSerializer {

            #[inline]
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

            #[inline]
            pub fn read_head<SER>(&mut self) {
                //ignore first 7 bytes
                self.read_idx = 7;
                let class_name_len = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx + 2].try_into().unwrap());
                self.read_idx += 2;
                let _class_name = String::from_utf8_lossy(self.buf[self.read_idx..self.read_idx + class_name_len as usize].try_into().unwrap());
                self.read_idx += class_name_len as usize;
                //serialVersionUID
                self.read_idx += 8;
                //class flags
                self.read_idx += 1;
                //fields len
                let num_fileds = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx + 2].try_into().unwrap());
                self.read_idx += 2;
                //fields
                for i in 0..num_fileds {
                    let f_type = char::from(self.buf[self.read_idx]);
                    self.read_idx += 1;
                    let field_len = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx + 2].try_into().unwrap());
                    self.read_idx += 2;
                    let _field_name = String::from_utf8_lossy(self.buf[self.read_idx..self.read_idx + field_len as usize].try_into().unwrap());
                    self.read_idx += field_len as usize;

                    if f_type == 'L' {
                        let _tc_string = char::from(self.buf[self.read_idx]);
                        self.read_idx += 1;
                        let field_type_len = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx + 2].try_into().unwrap());
                        self.read_idx += 2;
                        let _field_type = String::from_utf8_lossy(self.buf[self.read_idx..self.read_idx + field_type_len as usize].try_into().unwrap());
                        self.read_idx += field_type_len as usize;
                    }
                }
                self.read_idx += 2;//TC_ENDBLOCKDATA, TC_NULL
            }

            #[inline]
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
                        "u8" => {
                            self.buf.push('B' as u8);
                        },
                        "char" => {
                            self.buf.push('C' as u8);
                        },
                        "f64" => {
                            self.buf.push('D' as u8);
                        },
                        "f32" => {
                            self.buf.push('F' as u8);
                        },
                        "i64" => {
                            self.buf.push('J' as u8);
                        },
                        "i16" => {
                            self.buf.push('S' as u8);
                        },
                        "bool" => {
                            self.buf.push('Z' as u8);
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
                                    if data.0 == "java.lang.Object" {
                                        self.inner_is_object = true;
                                    }
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
                if !self.inner_is_object {
                    self.buf.push(120); //TC_ENDBLOCKDATA
                    self.buf.push(112); // TC_NULL
                }
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
                if self.inner {
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
                if self.inner {
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
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
                Ok(Compound { ser: self })
            }

            #[inline]
            fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Display {
                unimplemented!()
            }
        }



        impl<'de, 'a> Deserializer<'de> for &'a mut JvmSerializer {
            type Error = Error;

            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {

                let value = i32::from_be_bytes(self.buf[self.read_idx..self.read_idx + 4].try_into().unwrap());
                self.read_idx += 4;
                visitor.visit_i32(value)
            }

            fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                self.read_idx += 1;
                let string_len = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx+2].try_into().unwrap()) as usize;
                self.read_idx += 2;
                let string_value = String::from_utf8_lossy(self.buf[self.read_idx..self.read_idx + string_len].try_into().unwrap());
                self.read_idx += string_len;

                visitor.visit_string(string_value.to_string())
            }

            fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {

                struct Access<'a> {
                    deserializer: &'a mut JvmSerializer,
                    len: usize,
                }

                impl<'de, 'a> serde::de::SeqAccess<'de> for Access<'a> {
                    type Error = Error;

                    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
                        where T: serde::de::DeserializeSeed<'de>, {
                        if self.len > 0 {
                            self.len -= 1;
                            let value =
                                serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                            Ok(Some(value))
                        } else {
                            Ok(None)
                        }
                    }

                    fn size_hint(&self) -> Option<usize> {
                        Some(self.len)
                    }
                }

                visitor.visit_seq(Access {
                    deserializer: self,
                    len,
                })

            }

            fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {

                let inner_type_code = self.buf[self.read_idx];
                if inner_type_code == 115 { //TC_OBJECT
                    // consume inner object header
                    self.read_idx +=2;
                    let string_len = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx+2].try_into().unwrap()) as usize;
                    self.read_idx += 2;
                    let _string_value = String::from_utf8_lossy(self.buf[self.read_idx..self.read_idx + string_len].try_into().unwrap());
                    self.read_idx += string_len;
                    let _uuid = i64::from_be_bytes(self.buf[self.read_idx..self.read_idx+8].try_into().unwrap());
                    self.read_idx += 8;
                    self.read_idx += 3;

                    for _f in fields {
                        let _code = self.buf[self.read_idx];
                        self.read_idx += 1;
                        let string_len = i16::from_be_bytes(self.buf[self.read_idx..self.read_idx+2].try_into().unwrap()) as usize;
                        self.read_idx += 2;
                        let _string_value = String::from_utf8_lossy(self.buf[self.read_idx..self.read_idx + string_len].try_into().unwrap());
                        self.read_idx += string_len;
                    }
                    self.read_idx += 7;

                    self.inner = true;
                }

                self.deserialize_tuple(fields.len(), visitor)
            }

            fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                unimplemented!()
            }

            fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
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

        impl serde::de::Error for Error {

            fn custom<T>(msg: T) -> Self where
                T: Display {
                unimplemented!()
            }
        }
        
    }
}

