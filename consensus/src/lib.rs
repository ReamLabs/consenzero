use alloy_primitives::B256;
use anyhow::ensure;
use tree_hash::TreeHash;

use ream_consensus::deneb::beacon_block::BeaconBlock;
use ream_consensus::beacon_block_header::BeaconBlockHeader;

pub fn partials_process_block_header(slot: u64, latest_block_header: &BeaconBlockHeader, proposer_index: u64, block: &BeaconBlock) -> anyhow::Result<BeaconBlockHeader> {
    // Verify that the slots match
    ensure!(
        slot == block.slot,
        "State slot must be equal to block slot"
    );
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
        block.parent_root == latest_block_header.tree_hash_root(),
        "Block Parent Root must be equal root of latest block header"
    );

    // Cache current block as the new latest block
    let new_latest_block_header = BeaconBlockHeader {
        slot: block.slot,
        proposer_index: block.proposer_index,
        parent_root: block.parent_root,
        state_root: B256::default(), // Overwritten in the next process_slot call
        body_root: block.body.tree_hash_root(),
    };

    // // Verify proposer is not slashed
    // let proposer_slashed = Self::not_slashed(block.proposer_index as usize, "validators_merkle_root", "merkle_path");
    // ensure!(!proposer_slashed, "Block proposer must not be slashed");

    Ok(new_latest_block_header)
}

fn not_slashed(proposer_index: usize, block_state_root: &B256, merkle_path: &str) -> bool {
    true
}
