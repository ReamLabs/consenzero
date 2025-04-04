use alloy_primitives::FixedBytes;
use anyhow::ensure;
use ssz_rs::HashTreeRoot;

use crate::{beacon_block::BeaconBlock, beacon_block_header::BeaconBlockHeader};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProcessBlockHeaderOutput {
    pub pre_state_root: FixedBytes<32>,
    pub beacon_block_root: FixedBytes<32>,
    pub new_beacon_block_header_root: FixedBytes<32>,
}

pub fn process_block_header(
    slot: u64,
    latest_block_header: &BeaconBlockHeader,
    validator_slashed: bool,
    proposer_index: u64,
    block: &BeaconBlock,
) -> anyhow::Result<BeaconBlockHeader> {
    // Authenticates the arguments against the provided root

    // Verify that the slots match
    ensure!(slot == block.slot, "State slot must be equal to block slot");
    // Verify that the block is newer than latest block header
    ensure!(
        block.slot > latest_block_header.slot,
        "Block slot must be greater than latest block header slot of state"
    );
    // Verify that proposer index is the correct index
    ensure!(
        block.proposer_index == proposer_index,
        "Block proposer index must be equal to beacon proposer index"
    );
    // Verify that the parent matches
    ensure!(
        block.parent_root == latest_block_header.hash_tree_root()?,
        "Block Parent Root must be equal root of latest block header"
    );

    // Cache current block as the new latest block
    let new_latest_block_header = BeaconBlockHeader {
        slot: block.slot,
        proposer_index: block.proposer_index,
        parent_root: block.parent_root,
        state_root: FixedBytes::<32>::ZERO, // Overwritten in the next process_slot call
        body_root: block.body.hash_tree_root()?,
    };

    // Verify proposer is not slashed
    ensure!(!validator_slashed, "Block proposer must not be slashed");

    Ok(new_latest_block_header)
}
