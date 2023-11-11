mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreNew};
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::{BigDecimal, BigInt};

const TRACKED_CONTRACT: [u8; 20] = hex!("b9a179DcA5a7bf5f8B9E088437B3A85ebB495eFe");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        claimed_draws: blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::contract::events::ClaimedDraw::match_and_decode(log)
                        {
                            return Some(contract::ClaimedDraw {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                draw_id: event.draw_id.to_u64(),
                                payout: event.payout.to_string(),
                                user: event.user,
                            });
                        }

                        None
                    })
            })
            .collect(),
    })
}

#[substreams::handlers::store]
fn total_claimed_store(events: contract::Events, o: StoreAddBigInt) {
    for draw in events.claimed_draws.into_iter() {
        o.add(
            0,
            format!("Draw:{}", &draw.draw_id),
            BigInt::from_str(&draw.payout).unwrap(),
        );
        o.add(
            0,
            format!("Account:{}", Hex(&draw.user).to_string()),
            BigInt::from_str(&draw.payout).unwrap(),
        );
        o.add(
            0,
            format!(
                "AccountDraw:{}:{}",
                Hex(&draw.user).to_string(),
                &draw.draw_id
            ),
            BigInt::from_str(&draw.payout).unwrap(),
        );
        o.add(
            0,
            format!("Aggregate"),
            BigInt::from_str(&draw.payout).unwrap(),
        );
    }
}

#[substreams::handlers::map]
fn graph_out(
    events: contract::Events,
    totals_claimed: StoreGetBigInt,
) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.claimed_draws.into_iter().for_each(|draw| {
        match totals_claimed.get_last(format!("Aggregate")) {
            Some(total) => {
                tables
                    .update_row("Aggregate", format!("Aggregate"))
                    .set("totalClaimed", total);
            }
            None => {
                tables
                    .create_row("Aggregate", format!("Aggregate"))
                    .set("totalClaimed", BigDecimal::from_str(&draw.payout).unwrap());
            }
        }

        match totals_claimed.get_last(format!("Account:{}", Hex(&draw.user).to_string())) {
            Some(total) => {
                tables
                    .update_row("Account", format!("{}", Hex(&draw.user).to_string()))
                    .set("totalClaimed", total);
            }
            None => {
                tables
                    .create_row("Account", format!("{}", Hex(&draw.user).to_string()))
                    .set("totalClaimed", draw.draw_id);
            }
        }

        match totals_claimed.get_last(format!("Draw:{}", &draw.draw_id)) {
            Some(total) => {
                tables
                    .update_row("Draw", format!("{}", &draw.draw_id))
                    .set("totalClaimed", total)
                    .set("updatedAtTimestamp", draw.evt_block_time.as_ref().unwrap());
            }
            None => {
                tables
                    .create_row("Draw", format!("{}", &draw.draw_id))
                    .set("totalClaimed", BigDecimal::from_str(&draw.payout).unwrap())
                    .set("createdAtTimestamp", draw.evt_block_time.as_ref().unwrap())
                    .set("updatedAtTimestamp", draw.evt_block_time.as_ref().unwrap());
            }
        }

        match totals_claimed.get_last(format!(
            "AccountDraw:{}:{}",
            Hex(&draw.user).to_string(),
            &draw.draw_id
        )) {
            Some(total) => {
                tables
                    .update_row(
                        "AccountDraw",
                        format!("{}:{}", Hex(&draw.user).to_string(), &draw.draw_id),
                    )
                    .set("claimed", BigDecimal::from_str(&draw.payout).unwrap())
                    .set("totalClaimed", total)
                    .set(
                        "lastClaimedAtTimestamp",
                        draw.evt_block_time.as_ref().unwrap(),
                    );
            }
            None => {
                tables
                    .create_row(
                        "AccountDraw",
                        format!("{}{}", Hex(&draw.user).to_string(), &draw.draw_id),
                    )
                    .set("account", format!("{}", Hex(&draw.user).to_string()))
                    .set("draw", format!("{}", &draw.draw_id))
                    .set("claimed", BigDecimal::from_str(&draw.payout).unwrap())
                    .set("totalClaimed", BigDecimal::from_str(&draw.payout).unwrap())
                    .set(
                        "firstClaimedAtTimestamp",
                        draw.evt_block_time.as_ref().unwrap(),
                    )
                    .set(
                        "lastClaimedAtTimestamp",
                        draw.evt_block_time.as_ref().unwrap(),
                    );
            }
        }
    });
    Ok(tables.to_entity_changes())
}
