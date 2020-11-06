#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;

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

    impl ServerID {

        fn test(&mut self) {
            
        }

    }

    #[jvm_object(io.vertx.core.eventbus.impl.clustered.ClusterNodeInfo,453453453454)]
    struct ClusterNodeInfo {
        nodeId: String,
        serverID: ServerID
    }

    #[test] 
    fn it_works() {

       let mut oos = ObjectOutputStream::new();

        for i in 0..10 {
            let node_id = ClusterNodeInfo {
                nodeId: uuid::Uuid::new_v4().to_string(),
                serverID: ServerID{
                    port: 45000,
                    host: String::from("localhost")
                }
            };
            oos.write_object(&node_id);
        }
        


        assert_eq!(2 + 2, 4);
    }
}