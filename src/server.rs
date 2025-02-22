use std::sync::Arc;
use veilid_core::VeilidUpdate::{AppMessage, Network};
use veilid_core::{
    VeilidConfigBlockStore, VeilidConfigInner, VeilidConfigProtectedStore, VeilidConfigTableStore,
    VeilidUpdate,
};

#[tokio::main]
pub async fn run() {
    println!("Server");
    let update_callback = Arc::new(move |update: VeilidUpdate| {
        match update {
            AppMessage(msg) => {
                println!("Message: {}", String::from_utf8_lossy(msg.message().into()));
            }
            Network(msg) => {
                println!(
                    "Network: Peers {:}, bytes/sec [{} up] [{} down]",
                    msg.peers.iter().count(),
                    msg.bps_up,
                    msg.bps_down
                )
            }
            _ => {
                println!("{:?}", update)
            }
        };
    });

    let config = VeilidConfigInner {
        program_name: "TestDemoGetSet".into(),
        namespace: "veilid-example".into(),
        protected_store: VeilidConfigProtectedStore {
            // avoid prompting for password, don't do this in production
            always_use_insecure_storage: true,
            directory: "./.veilid/block_store".into(),
            ..Default::default()
        },
        block_store: VeilidConfigBlockStore {
            directory: "./.veilid/block_store".into(),
            ..Default::default()
        },
        table_store: VeilidConfigTableStore {
            directory: "./.veilid/table_store".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let veilid = veilid_core::api_startup_config(update_callback, config)
        .await
        .unwrap();
    println!("Starting");
    veilid.attach().await.unwrap();

    let routing_context = veilid.routing_context();


    tokio::signal::ctrl_c().await.unwrap();
    veilid.shutdown().await;
}
