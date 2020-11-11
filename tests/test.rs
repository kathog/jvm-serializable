#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;
extern crate jvm_serializable;


pub mod io {
    pub mod vertx {
        pub mod core {
            pub mod net {
                pub mod _impl {
                    use jvm_serializable::java::io::*;

                    #[jvm_object(io.vertx.core.net.impl.ServerID,5636540499169644934)]
                    pub struct ServerID {
                        pub port: i32,
                        pub host: String
                    }
                }
            }
            pub mod eventbus {
                pub mod _impl {

                    pub mod clustered {
                        use jvm_serializable::java::io::*;
                        use crate::io;

                        #[jvm_object(io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo,1)]
                        pub struct ClusterNodeInfo {
                            pub nodeId: String,
                            pub serverID: io::vertx::core::net::_impl::ServerID,
                        }
                    }

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate serde;

    use jvm_serializable::java::io::*;
    use std::collections::HashMap;
    use std::any::{Any, type_name};
    use std::collections::hash_map::RandomState;
    use traitcast::{TraitcastFrom, Traitcast};
    use serde_json::Value;
    use crate::io::vertx::core::eventbus::_impl::clustered::ClusterNodeInfo;
    use crate::io::vertx::core::net::_impl::ServerID;


    // #[jvm_object(io.vertx.core.net.impl.ServerID,5435534543543)]
    // struct ServerID {
    //     port: i32,
    //     host: String
    // }


    // #[jvm_object(io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo,453453453454)]
    // struct ClusterNodeInfo {
    //     nodeId: String,
    //     serverID: io::vertx::core::net::_impl::ServerID,
    // }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }



    #[test] 
    fn it_works() {

        for i in 0..10 {
            let mut oos = ObjectOutputStream::new();
            let mut node_id = ClusterNodeInfo {
                nodeId: uuid::Uuid::new_v4().to_string(),
                serverID : ServerID {
                    port: 45809,
                    host: String::from("localhost")
                }
            };

            // ServerID::set_field::<i32>(&mut node_id.serverID, "port", 1234);
            // println!("{:?}", &node_id);
            oos.write_object(&node_id);

            // let s = &node_id as &dyn Any + Sized;
        }
    }
}