use criterion::{criterion_group, criterion_main, Criterion};
#[macro_use]
extern crate serde;
#[macro_use]
extern crate jvm_macro;

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

fn serilze(_c: &mut Criterion) {
    
    let node_id = ClusterNodeInfo {
        nodeId: String::from("9cb173-beaa-4a11-98b4-efb395b76479"),
        serverID: ServerID{
            port: 45000,
            host: String::from("localhost")
        }
    };


    let mut oos = ObjectOutputStream::new();

    
    _c.bench_function("build_trace", |b| b.iter(|| {
        oos.write_object(&node_id);
    }));
}


criterion_group!(benches, serilze);
criterion_main!(benches);