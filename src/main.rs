use std::sync::Arc;
use veilid_core::VeilidUpdate::{AppMessage, Network};
use veilid_core::{
    VeilidConfigBlockStore, VeilidConfigInner, VeilidConfigProtectedStore, VeilidConfigTableStore,
    VeilidUpdate,
};

fn handle_update(update: VeilidUpdate) {
    {
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
    }
}

fn noop_update(_update: VeilidUpdate) {}

fn mk_config() -> VeilidConfigInner {
    return VeilidConfigInner {
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
}

#[tokio::main]
pub async fn keygen() {
    println!("Keygen");
    let veilid = veilid_core::api_startup_config(Arc::new(noop_update), mk_config()).await.unwrap();
    let crypto_system = veilid.crypto().unwrap();
    let keypair = crypto_system.best().generate_keypair();
    println!("Generated: {}", keypair);
    veilid.shutdown().await;
}

#[tokio::main]
pub async fn server(key: String) {
//    println!("Server");
//
//    let veilid = veilid_core::api_startup_config(UPDATE_CALLBACK, CONFIG)
//        .await
//        .unwrap();
//    println!("Starting");
//    veilid.attach().await.unwrap();
//
//    let routing_context = veilid.routing_context();
//
//    tokio::signal::ctrl_c().await.unwrap();
//    veilid.shutdown().await;
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "cjen1", version = "0.0.0", about = "Test for veilid")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keygen,
    Server {
        #[arg(short, long, required = true)]
        key: String,
    },
    Client {
        #[arg(short, long, required = true)]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Keygen => {
            keygen();
        }
        Commands::Server { key } => todo!(),
        Commands::Client { key } => todo!(),
    }
}
