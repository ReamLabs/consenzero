use risc0_zkvm::guest::env;

use alloy_primitives::FixedBytes;
use consensus_common::{
    beacon_block::BeaconBlock,
    beacon_block_header::BeaconBlockHeader,
    funcs::{process_block_header, ProcessBlockHeaderOutput},
    proof::Proof,
};
use ssz_rs::prelude::*;

fn main() {
    // let count_start = env::cycle_count();

    let pre_state_root: FixedBytes<32> = env::read();

    let slot: u64 = env::read();
    let slot_proof: Proof = env::read();
    let latest_block_header_ssz: Vec<u8> = env::read();
    let latest_block_header: BeaconBlockHeader =
        BeaconBlockHeader::deserialize(&mut latest_block_header_ssz.as_slice()).unwrap();
    let latest_block_header_proof: Proof = env::read();
    // TODO: pass in validator_slashed
    // let validator_slashed: bool = env::read();
    // let validator_slashed_proof: Proof = env::read();

    let proposer_index: u64 = env::read();
    // TODO: pass in proposer_index_proof

    let block_ssz: Vec<u8> = env::read();
    let block = BeaconBlock::deserialize(&mut block_ssz.as_slice()).unwrap();

    // Verify proofs
    slot_proof.verify().unwrap();
    assert_eq!(slot_proof.index, 34);
    assert_eq!(slot_proof.witness, pre_state_root);
    assert_eq!(slot_proof.leaf, slot.hash_tree_root().unwrap());
    latest_block_header_proof.verify().unwrap();
    assert_eq!(latest_block_header_proof.index, 36);
    assert_eq!(latest_block_header_proof.witness, pre_state_root);
    assert_eq!(
        latest_block_header_proof.leaf,
        latest_block_header.hash_tree_root().unwrap()
    );
    // validator_slashed_proof.verify().unwrap();
    // TODO: assert_eq for generalized index
    // assert_eq!(validator_slashed_proof.witness, pre_state_root);
    // assert_eq!(validator_slashed_proof.leaf, validator_slashed.hash_tree_root().unwrap());

    let new_block_header = process_block_header(
        slot,
        &latest_block_header,
        false, // validator_slashed, TODO: fix
        proposer_index,
        &block,
    )
    .unwrap();

    let output = ProcessBlockHeaderOutput {
        beacon_block_root: new_block_header.hash_tree_root().unwrap(),
        new_beacon_block_header_root: new_block_header.hash_tree_root().unwrap(),
        pre_state_root,
    };

    env::commit(&output);
}
