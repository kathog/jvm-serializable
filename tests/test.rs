#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;
use jvm_macro::jvm_object;

#[cfg(test)]
mod tests {

    extern crate serde;
    extern crate jvm_serializable;
    use jvm_serializable::java::io::*;
    
    #[jvm_object]
    struct ServerID {
        port: i32,
        host: String
    }

    // impl Serializable for ServerID {
    //     fn java_class_name (&self) -> String {
    //         "io.vertx.core.net.impl.ServerID".to_string()
    //     }
    // }

    #[jvm_object]
    struct ClusterNodeInfo {
        nodeId: String,
        serverID: ServerID
    }

    // impl Serializable for ClusterNodeInfo {
    //     fn java_class_name (&self) -> String {
    //         "io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo".to_string()
    //     }
    // }


    #[test] 
    fn it_works() {

        let node_id = ClusterNodeInfo {
            nodeId: String::from("9cb173-beaa-4a11-98b4-efb395b76479"),
            serverID: ServerID{
                port: 45000,
                host: String::from("localhost")
            }
        };


        // let java_class = node_id.java_class_name();
        // println! ("{:?}", java_class);
        // node_id.serverID.serialize()

        let oos = ObjectOutputStream::new();
        oos.write_object(&node_id);


        assert_eq!(2 + 2, 4);
    }
}