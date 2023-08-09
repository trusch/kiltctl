use std::str::FromStr;

use clap::ArgMatches;
use kiltapi::{connect, kilt::KiltConfig, unwrap_or_stdin};
use subxt::tx::SubmittableExtrinsic;

pub fn command() -> clap::Command {
    clap::Command::new("submit")
        .about("Submit a transaction")
        .arg(
            clap::Arg::new("tx")
                .short('t')
                .long("tx")
                .help("Transaction to submit"),
        )
        .arg(
            clap::Arg::new("wait-for")
                .short('w')
                .long("wait-for")
                .help("How long to wait")
                .value_parser(["submitted", "in-block", "finalized"])
                .required(false)
                .default_value("in-block"),
        )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WaitFor {
    Submitted,
    InBlock,
    Finalized,
}

impl FromStr for WaitFor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "submitted" => Ok(WaitFor::Submitted),
            "in-block" => Ok(WaitFor::InBlock),
            "finalized" => Ok(WaitFor::Finalized),
            _ => Err(format!("Invalid wait-for value: {s}")),
        }
    }
}

pub async fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let wait_for = WaitFor::from_str(matches.get_one::<String>("wait-for").unwrap())?;
    let tx = unwrap_or_stdin(matches.get_one("tx").map(|s: &String| s.to_owned()))?;
    let tx = hex::decode(tx.trim_start_matches("0x").trim())?;

    let cli = connect(matches).await?;

    let tx = SubmittableExtrinsic::from_bytes(cli, tx);
    submit_extrinsic(tx, wait_for).await?;
    Ok(())
}

async fn submit_extrinsic(
    tx: SubmittableExtrinsic<KiltConfig, subxt::OnlineClient<KiltConfig>>,
    wait_for: WaitFor,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut progress = tx.submit_and_watch().await?;
    log::info!(
        "Submitted Extrinsic with hash {:?}",
        progress.extrinsic_hash()
    );
    while let Some(Ok(status)) = progress.next_item().await {
        match status {
            subxt::tx::TxStatus::Future => {
                log::info!("Transaction is in the future queue");
            }
            subxt::tx::TxStatus::Ready => {
                log::info!("Extrinsic is ready");
            }
            subxt::tx::TxStatus::Broadcast(peers) => {
                log::info!("Extrinsic broadcasted to {:?}", peers);
                if wait_for == WaitFor::Submitted {
                    return Ok(());
                }
            }
            subxt::tx::TxStatus::InBlock(status) => {
                log::info!("Extrinsic included in block {:?}", status.block_hash());
                let events = status.fetch_events().await?;
                events.iter().for_each(|e| {
                    if let Ok(e) = e {
                        log::info!(
                            "{}.{}: {:#?}",
                            e.pallet_name(),
                            e.variant_name(),
                            e.event_metadata().pallet.docs()
                        );
                    }
                });
                if wait_for == WaitFor::InBlock {
                    return Ok(());
                }
            }
            subxt::tx::TxStatus::Retracted(hash) => {
                log::info!("Extrinsic retracted from block {:?}", hash);
            }
            subxt::tx::TxStatus::Finalized(status) => {
                log::info!("Extrinsic finalized in block {:?}", status.block_hash());
                if wait_for == WaitFor::Finalized {
                    return Ok(());
                }
            }
            subxt::tx::TxStatus::Usurped(hash) => {
                log::info!("Extrinsic usurped in block {:?}", hash);
            }
            subxt::tx::TxStatus::Dropped => {
                log::info!("Extrinsic dropped");
            }
            subxt::tx::TxStatus::Invalid => {
                log::info!("Extrinsic invalid");
            }
            subxt::tx::TxStatus::FinalityTimeout(hash) => {
                log::info!("Extrinsic finality timeout in block {:?}", hash);
            }
        }
    }
    Ok(())
}
