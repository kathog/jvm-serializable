#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;
use jvm_macro::*;

#[cfg(test)]
mod tests {

    extern crate serde;
    extern crate jvm_serializable;
    use jvm_serializable::java::io::*;
    
    #[jvm_object(io.vertx.core.net.impl.ServerID,5435534543543)]
    struct ServerID {
        port: i32,
        host: String
    }

    #[jvm_object(io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo,453453453454)]
    struct ClusterNodeInfo {
        nodeId: String,
        serverID: ServerID
    }

    #[test] 
    fn it_works() {

        let node_id = ClusterNodeInfo {
            nodeId: String::from("9cb173-beaa-4a11-98b4-efb395b76479"),
            serverID: ServerID{
                port: 45000,
                host: String::from("localhost")
            }
        };


        let oos = ObjectOutputStream::new();
        oos.write_object(&node_id);


        assert_eq!(2 + 2, 4);
    }
}