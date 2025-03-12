use alloy_primitives::FixedBytes;
use anyhow::ensure;
use ssz_rs::prelude::*;

use ream_consensus::deneb::beacon_state::BeaconState as ReamBeaconState;

use crate::fork::Fork;
use crate::beacon_block_header::BeaconBlockHeader;
use crate::beacon_block::BeaconBlock;
use crate::eth_1_data::Eth1Data;
use crate::validator::Validator;

#[derive(Debug, SimpleSerialize)]
pub struct BeaconState {
    // Versioning
    pub genesis_time: u64,
    pub genesis_validators_root: Vector<u8, 32>,
    pub slot: u64,
    pub fork: Fork,

    // // History
    pub latest_block_header: BeaconBlockHeader,
    // pub block_roots: FixedVector<B256, U8192>,
    // pub state_roots: FixedVector<B256, U8192>,
    // /// Frozen in Capella, replaced by historical_summaries
    // pub historical_roots: VariableList<B256, U16777216>,

    // // Eth1
    pub eth1_data: Eth1Data,
    // pub eth1_data_votes: VariableList<Eth1Data, U2048>,
    // pub eth1_deposit_index: u64,

    // // Registry
    pub validators: List<Validator, 10000000>,
    // pub balances: VariableList<u64, U1099511627776>,

    // // Randomness
    // pub randao_mixes: FixedVector<B256, U65536>,

    // // Slashings
    // pub slashings: FixedVector<u64, U8192>,

    // // Participation
    // pub previous_epoch_participation: VariableList<u8, U1099511627776>,
    // pub current_epoch_participation: VariableList<u8, U1099511627776>,

    // // Finality
    // pub justification_bits: BitVector<U4>,
    // pub previous_justified_checkpoint: Checkpoint,
    // pub current_justified_checkpoint: Checkpoint,
    // pub finalized_checkpoint: Checkpoint,

    // // Inactivity
    // pub inactivity_scores: VariableList<u64, U1099511627776>,

    // // Sync
    // pub current_sync_committee: Arc<SyncCommittee>,
    // pub next_sync_committee: Arc<SyncCommittee>,

    // // Execution
    // pub latest_execution_payload_header: ExecutionPayloadHeader,

    // // Withdrawals
    // pub next_withdrawal_index: u64,
    // pub next_withdrawal_validator_index: u64,

    // // Deep history valid from Capella onwards.
    // pub historical_summaries: VariableList<HistoricalSummary, U16777216>,
}

impl From<ReamBeaconState> for BeaconState {
    fn from(state: ReamBeaconState) -> Self {
        let mut validators = List::<Validator, 10000000>::default();

        for v in state.validators.iter() {
            validators.push(v.clone().into());
        }


        BeaconState {
            // Versioning
            genesis_time: state.genesis_time,
            genesis_validators_root: state.genesis_validators_root.as_slice().try_into().unwrap(),
            slot: state.slot,
            fork: state.fork.into(),

            // // History
            latest_block_header: state.latest_block_header.into(),
            // pub block_roots: FixedVector<B256, U8192>,
            // pub state_roots: FixedVector<B256, U8192>,
            // /// Frozen in Capella, replaced by historical_summaries
            // pub historical_roots: VariableList<B256, U16777216>,

            // // Eth1
            eth1_data: state.eth1_data.into(),
            // pub eth1_data_votes: VariableList<Eth1Data, U2048>,
            // eth1_deposit_index: state.eth1_deposit_index,

            // // Registry
            validators: validators,
            // pub balances: VariableList<u64, U1099511627776>,

            // // Randomness
            // pub randao_mixes: FixedVector<B256, U65536>,

            // // Slashings
            // pub slashings: FixedVector<u64, U8192>,

            // // Participation
            // pub previous_epoch_participation: VariableList<u8, U1099511627776>,
            // pub current_epoch_participation: VariableList<u8, U1099511627776>,

            // // Finality
            // pub justification_bits: BitVector<U4>,
            // pub previous_justified_checkpoint: Checkpoint,
            // pub current_justified_checkpoint: Checkpoint,
            // pub finalized_checkpoint: Checkpoint,

            // // Inactivity
            // pub inactivity_scores: VariableList<u64, U1099511627776>,

            // // Sync
            // pub current_sync_committee: Arc<SyncCommittee>,
            // pub next_sync_committee: Arc<SyncCommittee>,

            // // Execution
            // pub latest_execution_payload_header: ExecutionPayloadHeader,

            // // Withdrawals
            // next_withdrawal_index: state.next_withdrawal_index,
            // next_withdrawal_validator_index: state.next_withdrawal_validator_index,

            // // Deep history valid from Capella onwards.
            // pub historical_summaries: VariableList<HistoricalSummary, U16777216>,
        }
    }
}

pub fn process_block_header(slot: u64, latest_block_header: &BeaconBlockHeader, proposer_index: u64, block: &BeaconBlock) -> anyhow::Result<BeaconBlockHeader> {
    // Authenticates the arguments against the provided root


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

    // // Verify proposer is not slashed
    // let proposer_slashed = Self::not_slashed(block.proposer_index as usize, "validators_merkle_root", "merkle_path");
    // ensure!(!proposer_slashed, "Block proposer must not be slashed");

    Ok(new_latest_block_header)
}

// fn not_slashed(proposer_index: usize, block_state_root: &Vector<u8, 32>, merkle_path: &str) -> bool {
//     true
// }

impl BeaconState {
    // Mock function for consenzero PoC
    pub fn get_beacon_proposer_index(&self) -> anyhow::Result<u64> {
        Ok(42)
    }
}
