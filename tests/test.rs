#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;
extern crate jvm_serializable;


#[cfg(test)]
mod tests {

    extern crate serde;

    use jvm_serializable::java::io::*;
    use std::collections::HashMap;
    use std::any::{type_name};
    use std::collections::hash_map::RandomState;

    #[jvm_object(io.vertx.core.net.impl.ServerID,5636540499169644934)]
    struct ServerID_tmp {
        port: i32,
        host: String
    }


    #[jvm_object(io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo,1)]
    struct ClusterNodeInfo_tmp {
        nodeId: String,
        serverID: ServerID,
    }


    #[test] 
    fn it_works() {

        for i in 0..10 {
            let mut oos = ObjectOutputStream::new();
            let mut node_id = ClusterNodeInfo_tmp {
                nodeId: uuid::Uuid::new_v4().to_string(),
                serverID : ServerID_tmp {
                    port: 45809,
                    host: String::from("localhost")
                },
            };

            oos.write_object(&node_id);

            let mut ois = ObjectInputStream{};
            let node : ClusterNodeInfo = ois.read_object(oos.to_byte_array());
            println!("{:?}", node);
        }
    }
}