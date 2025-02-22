use std::sync::Arc;
use tokio::io::AsyncBufReadExt;
use veilid_core::VeilidUpdate::{AppMessage, Network};
use veilid_core::{
    DHTSchema, VeilidConfigBlockStore, VeilidConfigInner, VeilidConfigProtectedStore, VeilidConfigTableStore, VeilidUpdate
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

fn mk_config(prefix: &str) -> VeilidConfigInner {
    return VeilidConfigInner {
        program_name: "TestDemoGetSet".into(),
        namespace: "veilid-example".into(),
        protected_store: VeilidConfigProtectedStore {
            // avoid prompting for password, don't do this in production
            always_use_insecure_storage: true,
            directory: format!("./{}/.veilid/block_store", prefix),
            ..Default::default()
        },
        block_store: VeilidConfigBlockStore {
            directory: format!("./{}/.veilid/block_store", prefix),
            ..Default::default()
        },
        table_store: VeilidConfigTableStore {
            directory: format!("./{}/.veilid/table_store", prefix),
            ..Default::default()
        },
        ..Default::default()
    };
}

#[tokio::main]
pub async fn keygen() {
    println!("Keygen");
    let veilid = veilid_core::api_startup_config(Arc::new(noop_update), mk_config("keygen"))
        .await
        .unwrap();
    let crypto_system = veilid.crypto().unwrap();
    let keypair = crypto_system.best().generate_keypair();
    println!("Generated: {} {}", keypair.secret, keypair.key);
    veilid.shutdown().await;
}

#[tokio::main]
pub async fn server(pubkey: &String, privk: &String) {
    println!("Server");

    let keypair = veilid_core::KeyPair::new(pubkey, privk);

    let veilid = veilid_core::api_startup_config(Arc::new(handle_update), mk_config("server"))
        .await
        .unwrap();
    veilid.attach().await.unwrap();

    let routing_context = veilid.routing_context().unwrap();

    let schema = DHTSchema::smpl(0, vec![veilid_core::DHTSchemaSMPLMember{m_key: keypair.secret, m_cnt: 1}]).unwrap();

    // No owner key passed => only schema members can write
    let record = routing_context.create_dht_record(schema, None, None).await.unwrap();
    println!("Record key is: {}", record.key());

    routing_context.close_dht_record(*record.key()).await;

    routing_context.open_dht_record(*record.key(), Some(keypair));

    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut buffer = Vec::new();
    reader.read_until(b'\n', &mut buffer).await.unwrap();
    routing_context.set_dht_value(*record.key(), 0, buffer, None);
    routing_context.close_dht_record(*record.key());

    tokio::signal::ctrl_c().await.unwrap();
    veilid.shutdown().await;
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
        pubkey: String,
        #[arg(short, long, required = true)]
        privk: String,
    },
    Client {
        #[arg(short, long, required = true)]
        dhtkey: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Keygen => {
            keygen();
        }
        Commands::Server { pubkey, privk } => server(pubkey, privk),
        Commands::Client { dhtkey } => todo!(),
    }
}
