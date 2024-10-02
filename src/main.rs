mod cluster;
use crate::cluster::Cluster;
use anyhow::anyhow;
use clap::{arg, Parser};
use env_logger::{Builder, Env};
use log::info;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 'u',
        long,
        env,
        default_value = "https://api.mainnet-beta.solana.com"
    )]
    pub rpc_url: String,

    #[arg(long = "commitment", default_value = "confirmed")]
    pub commitment: CommitmentLevel,

    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[arg(short = 'f', long)]
    pub first_epoch: Option<u64>,

    #[arg(short = 'l', long)]
    pub last_epoch: Option<u64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    // init log
    let verbosity = if args.verbose { "debug" } else { "info" };
    let mut builder = Builder::from_env(Env::default().default_filter_or(verbosity));
    builder.init();

    // rpc client
    let rpc_url = args.rpc_url.clone();
    let rpc_cluster_info = Cluster::from_str(&rpc_url)
        .map_err(|e| anyhow!("Could not parse JSON RPC url `{:?}`: {}", rpc_url, e))?;
    let rpc_client = Arc::new(RpcClient::new_with_commitment(
        rpc_cluster_info.to_string(),
        CommitmentConfig {
            commitment: args.commitment,
        },
    ));

    let (first_epoch, last_epoch) = if let (Some(f), Some(l)) = (args.first_epoch, args.last_epoch)
    {
        (f, l)
    } else if let Some(f) = args.first_epoch {
        info!("Last epoch not provided, using current epoch");
        let epoch_info = rpc_client.get_epoch_info().await?;
        (f, epoch_info.epoch)
    } else if let Some(l) = args.last_epoch {
        info!("First epoch not provided, using current epoch");
        let epoch_info = rpc_client.get_epoch_info().await?;
        (epoch_info.epoch, l)
    } else {
        let epoch_info = rpc_client.get_epoch_info().await?;
        (epoch_info.epoch, epoch_info.epoch)
    };

    if first_epoch > last_epoch {
        return Err(anyhow!("Loaded or provided epoch span does not match. The first epoch [{first_epoch}] must be less than last epoch [{last_epoch}]"));
    }

    println!(
        "{0: <10} | {1: <10} | {2: <10}",
        "Epoch", "Start slot", "Last slot"
    );

    for epoch in first_epoch..=last_epoch {
        let epoch_schedule = rpc_client.get_epoch_schedule().await?;
        let start_slot = epoch_schedule.get_first_slot_in_epoch(epoch);
        let last_slot = epoch_schedule.get_last_slot_in_epoch(epoch);
        println!(
            "{0: <10} | {1: <10} | {2: <10}",
            epoch, start_slot, last_slot
        );
    }

    Ok(())
}
