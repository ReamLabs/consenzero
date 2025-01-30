// // These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// // The ELF is used for proving and the ID is used for verification.
use methods::{CONSENSUS_STF_ELF, CONSENSUS_STF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use clap::Parser;
use ream_consensus::deneb::beacon_state::BeaconState;
use ssz::Decode;
use beam_types::BeamState;

mod beacon_client;
use beacon_client::BeaconNodeHttpClient;

mod cli;
use cli::ProviderArgs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Fetch the state and block from the RPC
    #[clap(long)]
    rpc: bool,

    /// Fetch the state and block from local files
    #[clap(long)]
    local: bool,

    #[clap(flatten)]
    provider: ProviderArgs,
}

#[tokio::main]
async fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    if args.rpc == args.local {
        eprintln!("Error: You must specify either --rpc or --local");
        std::process::exit(1);
    }

    let pre_state =
        if args.rpc {
            let beacon_client = BeaconNodeHttpClient::new(args.provider.rpc_url.unwrap());

            let head_slot = beacon_client.get_beacon_head_slot().await.unwrap();
        
            println!("head slot: {:?}", &head_slot);
        
            //
            // Fetch state by ssz
            //
        
            let pre_state_ssz = beacon_client.get_beacon_state_ssz(head_slot).await.unwrap();
            // println!("pre-state ssz: {:?}", &pre_state_ssz);
        
            let pre_state = BeaconState::from_ssz_bytes(&pre_state_ssz);
            println!("pre-state: {:#?}", &pre_state);
        
            //
            // Fetch state by json
            //
        
            // let pre_state = beacon_client.get_beacon_state(head_slot).await;
            // println!("pre-state: {:?}", &pre_state);

            // Deserialize the struct
            let value = std::fs::read_to_string(format!("host/src/data/BeaconState/value.yaml"))
                .expect("cannot find test asset");
            serde_yaml::from_str::<BeaconState>(&value).unwrap()

        } else {
            // Deserialize the struct
            let value = std::fs::read_to_string(format!("host/src/data/BeaconState/value.yaml"))
                .expect("cannot find test asset");
            serde_yaml::from_str::<BeaconState>(&value).unwrap()
        };
    
    // 
    // zkVM operations
    // 

    let zkvm_input = BeamState {
        // Versioning
        genesis_time: pre_state.genesis_time,
        genesis_validators_root: pre_state.genesis_validators_root,
        slot: pre_state.slot,
        fork: pre_state.fork,

        // History
        latest_block_header: pre_state.latest_block_header,
        block_roots: pre_state.block_roots,
        state_roots: pre_state.state_roots,
        // Frozen in Capella, replaced by historical_summaries
        historical_roots: pre_state.historical_roots,

        // Eth1
        eth1_data: pre_state.eth1_data,
        eth1_data_votes: pre_state.eth1_data_votes,
        eth1_deposit_index: pre_state.eth1_deposit_index,

        // Registry
        validators: pre_state.validators,
        balances: pre_state.balances,

        // Withdrawals
        next_withdrawal_index: pre_state.next_withdrawal_index,
        next_withdrawal_validator_index: pre_state.next_withdrawal_validator_index,
    };

    // Build the zkVM guest environment
    let env = ExecutorEnv::builder()
        .write(&zkvm_input)
        .unwrap()
        .build()
        .unwrap();

    // Run the state transition

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, CONSENSUS_STF_ELF).unwrap();

    // Extract the receipt.
    let receipt = prove_info.receipt;

    // TODO: Implement code for retrieving receipt journal here.
    // For example:
    let output: BeamState = receipt.journal.decode().unwrap();

    println!("Receipt journal: {:?}", output);

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(CONSENSUS_STF_ID).unwrap();
}
