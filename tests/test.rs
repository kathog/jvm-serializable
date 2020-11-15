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
    struct ServerID {
        port: i32,
        host: String
    }


    #[jvm_object(io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo,1)]
    struct ClusterNodeInfo {
        nodeId: String,
        serverID: ServerID,
    }

    #[jvm_object(java.lang.Object,0)]
    struct Object {
        key: String,
        value: String,
    }

    #[jvm_object(io.vertx.spi.cluster.zookeeper.impl.ZKSyncMap$KeyValue,6529685098267757690)]
    struct ZKSyncMapKeyValue {
        key: Object,
    }


    #[test] 
    fn it_works() {

        for i in 0..10 {
            let mut oos = ObjectOutputStream::new();
            let mut node_id = ClusterNodeInfo {
                nodeId: "fce2f0d9-4db9-4eba-b044-a061ba4e5743".to_string(),
                serverID : ServerID {
                    port: 46511,
                    host: String::from("localhost")
                },
            };

            oos.write_object(&node_id);

            let mut ois = ObjectInputStream{};

            // println!("{:?}", oos.to_byte_array());

            // let x = [0, 172, 237, 0, 5, 115, 114, 0, 53, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 101, 118, 101, 110, 116, 98, 117, 115, 46, 105, 109, 112, 108, 46, 99, 108, 117, 115, 116, 101, 114, 101, 100, 46, 67, 108, 117, 115, 116, 101, 114, 78, 111, 100, 101, 73, 110, 102, 111, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 76, 0, 6, 110, 111, 100, 101, 73, 100, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 76, 0, 8, 115, 101, 114, 118, 101, 114, 73, 68, 116, 0, 33, 76, 105, 111, 47, 118, 101, 114, 116, 120, 47, 99, 111, 114, 101, 47, 110, 101, 116, 47, 105, 109, 112, 108, 47, 83, 101, 114, 118, 101, 114, 73, 68, 59, 120, 112, 116, 0, 36, 102, 99, 101, 50, 102, 48, 100, 57, 45, 52, 100, 98, 57, 45, 52, 101, 98, 97, 45, 98, 48, 52, 52, 45, 97, 48, 54, 49, 98, 97, 52, 101, 53, 55, 52, 51, 115, 0, 31, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 110, 101, 116, 46, 105, 109, 112, 108, 46, 83, 101, 114, 118, 101, 114, 73, 68, 78, 57, 3, 184, 103, 28, 17, 134, 2, 0, 2, 73, 0, 4, 112, 111, 114, 116, 76, 0, 4, 104, 111, 115, 116, 113, 0, 126, 0, 1, 120, 112, 0, 0, 181, 175, 116, 0, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116];
            // let b = [0, 172, 237, 0, 5, 115, 114, 0, 53, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 101, 118, 101, 110, 116, 98, 117, 115, 46, 105, 109, 112, 108, 46, 99, 108, 117, 115, 116, 101, 114, 101, 100, 46, 67, 108, 117, 115, 116, 101, 114, 78, 111, 100, 101, 73, 110, 102, 111, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 2, 76, 0, 6, 110, 111, 100, 101, 73, 100, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 76, 0, 8, 115, 101, 114, 118, 101, 114, 73, 68, 116, 0, 33, 76, 105, 111, 47, 118, 101, 114, 116, 120, 47, 99, 111, 114, 101, 47, 110, 101, 116, 47, 105, 109, 112, 108, 47, 83, 101, 114, 118, 101, 114, 73, 68, 59, 120, 112, 116, 0, 36, 102, 99, 101, 50, 102, 48, 100, 57, 45, 52, 100, 98, 57, 45, 52, 101, 98, 97, 45, 98, 48, 52, 52, 45, 97, 48, 54, 49, 98, 97, 52, 101, 53, 55, 52, 51, 115, 114, 0, 31, 105, 111, 46, 118, 101, 114, 116, 120, 46, 99, 111, 114, 101, 46, 110, 101, 116, 46, 105, 109, 112, 108, 46, 83, 101, 114, 118, 101, 114, 73, 68, 78, 57, 3, 184, 103, 28, 17, 134, 2, 0, 2, 73, 0, 4, 112, 111, 114, 116, 76, 0, 4, 104, 111, 115, 116, 113, 0, 126, 0, 1, 120, 112, 0, 0, 181, 175, 116, 0, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116];


            let node : ClusterNodeInfo = ois.read_object(oos.to_byte_array());
            println!("{:?}", node);


            let kv = ZKSyncMapKeyValue {
                key : Object {
                    key: "0e4b0367-c5e6-4559-9284-282f27349de7".to_string(),
                    value: "{\"verticles\":[],\"group\":\"__DISABLED__\",\"server_id\":{\"host\":\"localhost\",\"port\":41469}}".to_string()
                }
            };

            let mut oos = ObjectOutputStream::new();
            oos.write_object(&kv);
            println!("{:?}", String::from_utf8_lossy(&oos.to_byte_array()));

        }
    }
}