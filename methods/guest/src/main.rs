use risc0_zkvm::guest::env;
use ream_consensus::deneb::beacon_state::BeaconState;
use ream_consensus::deneb::beacon_block::BeaconBlock;

fn main() {
    let count_start = env::cycle_count();

    // Read the pre-state
    let mut pre_state: BeaconState = env::read();
    let count_after_read_prestate = env::cycle_count();

    // Read the block
    let block: BeaconBlock = env::read();
    let count_after_read_block = env::cycle_count();

    // Transition the state
    let post_state = pre_state.process_block_header(&block).unwrap();
    let count_after_process_block_header = env::cycle_count();

    // Benchmark cycle counts
    eprintln!("read_pre_state: {}", count_after_read_prestate - count_start);
    eprintln!("read_block: {}", count_after_read_block - count_after_read_prestate);
    eprintln!("process_block_header: {}", count_after_process_block_header - count_after_read_block);

    // Write public output to the journal
    env::commit(&post_state);
}
