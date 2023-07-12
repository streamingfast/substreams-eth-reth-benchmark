mod abi;

use abi::erc20::v1::events;
use substreams::errors::Error;
use substreams::{hex, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DbTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityTables;
use substreams_ethereum::pb::eth;
use substreams_ethereum::Event;

pub const CONTRACT: [u8; 20] = hex!("ae78736Cd615f374D3085123A210448E74Fc6393");

#[substreams::handlers::map]
fn graph_out(blk: eth::v2::Block) -> Result<EntityChanges, Error> {
    let mut tables = EntityTables::new();

    let blk_timestamp = blk.timestamp_seconds();

    for trx in blk.transaction_traces {
        if trx.status != eth::v2::TransactionTraceStatus::Succeeded as i32 {
            continue;
        }

        for call in trx.calls {
            if call.state_reverted {
                continue;
            }

            for log in call.logs {
                if log.address != CONTRACT {
                    continue;
                }

                if let Some(event) = events::Approval::match_and_decode(&log) {
                    tables
                        .create_row("Approval", format!("{}-{}", Hex(&trx.hash), log.index))
                        .set("timestamp", blk_timestamp)
                        .set("block_number", blk.number)
                        .set("log_index", log.index)
                        .set("tx_hash", &trx.hash)
                        .set("value", event.value)
                        .set("spender", event.spender)
                        .set("owner", event.owner);

                    continue;
                }

                if let Some(event) = events::Transfer::match_and_decode(&log) {
                    tables
                        .create_row("Transfer", format!("{}-{}", Hex(&trx.hash), log.index))
                        .set("timestamp", blk_timestamp)
                        .set("block_number", blk.number)
                        .set("log_index", log.index)
                        .set("tx_hash", &trx.hash)
                        .set("amount", event.value)
                        .set("sender", event.from)
                        .set("receiver", event.to);

                    continue;
                }
            }
        }
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
fn db_out(blk: eth::v2::Block) -> Result<DatabaseChanges, Error> {
    let mut tables = DbTables::new();

    let blk_timestamp = blk.timestamp_seconds();

    for trx in blk.transaction_traces {
        if trx.status != eth::v2::TransactionTraceStatus::Succeeded as i32 {
            continue;
        }

        for call in trx.calls {
            if call.state_reverted {
                continue;
            }

            for log in call.logs {
                if log.address != CONTRACT {
                    continue;
                }

                if let Some(event) = events::Approval::match_and_decode(&log) {
                    tables
                        .create_row("approval", format!("{}-{}", Hex(&trx.hash), log.index))
                        .set("timestamp", blk_timestamp)
                        .set("block_number", blk.number)
                        .set("log_index", log.index)
                        .set("tx_hash", &trx.hash)
                        .set("value", event.value)
                        .set("spender", event.spender)
                        .set("owner", event.owner);

                    continue;
                }

                if let Some(event) = events::Transfer::match_and_decode(&log) {
                    tables
                        .create_row("transfer", format!("{}-{}", Hex(&trx.hash), log.index))
                        .set("timestamp", blk_timestamp)
                        .set("block_number", blk.number)
                        .set("log_index", log.index)
                        .set("tx_hash", &trx.hash)
                        .set("amount", event.value)
                        .set("sender", event.from)
                        .set("receiver", event.to);

                    continue;
                }
            }
        }
    }

    Ok(tables.to_database_changes())
}
