use risc0_zkvm::guest::env;

use consensus::beacon_state;
use ream_consensus::deneb::beacon_block::BeaconBlock;
use ream_consensus::beacon_block_header::BeaconBlockHeader;

fn main() {
    // let count_start = env::cycle_count();

    let slot: u64 = env::read();
    let latest_block_header: BeaconBlockHeader = env::read();
    let proposer_index: u64 = env::read();
    let block: BeaconBlock = env::read();

    let new_block_header = beacon_state::process_block_header(
            slot,
            &latest_block_header,
            proposer_index,
            &block,
        )
        .unwrap();

    env::commit(&new_block_header);
}
