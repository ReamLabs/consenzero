use alloy_primitives::FixedBytes;

use ssz_rs::prelude::*;

use ream_consensus::deneb::beacon_state::BeaconState as ReamBeaconState;

use consensus_common::{
    beacon_block_header::BeaconBlockHeader, checkpoint::Checkpoint, eth_1_data::Eth1Data,
    execution_payload_header::ExecutionPayloadHeader, fork::Fork,
    historical_summary::HistoricalSummary, sync_committee::SyncCommittee, validator::Validator,
};

#[derive(Debug, SimpleSerialize)]
pub struct BeaconState {
    // Versioning
    pub genesis_time: u64,
    pub genesis_validators_root: FixedBytes<32>,
    pub slot: u64,
    pub fork: Fork,

    // History
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: Vector<FixedBytes<32>, 8192>,
    pub state_roots: Vector<FixedBytes<32>, 8192>,
    pub historical_roots: List<FixedBytes<32>, 16777216>,

    // Eth1
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: List<Eth1Data, 2048>,
    pub eth1_deposit_index: u64,

    // Registry
    pub validators: List<Validator, 1099511627776>,
    pub balances: List<u64, 1099511627776>,

    // Randomness
    pub randao_mixes: Vector<FixedBytes<32>, 65536>,

    // Slashings
    pub slashings: Vector<u64, 8192>,

    // Participation
    pub previous_epoch_participation: List<u8, 1099511627776>,
    pub current_epoch_participation: List<u8, 1099511627776>,

    // Finality
    pub justification_bits: Bitvector<4>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,

    // Inactivity
    pub inactivity_scores: List<u64, 1099511627776>,

    // Sync
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: SyncCommittee,

    // Execution
    pub latest_execution_payload_header: ExecutionPayloadHeader,

    // Withdrawals
    pub next_withdrawal_index: u64,
    pub next_withdrawal_validator_index: u64,
    // Deep history valid from Capella onwards.
    pub historical_summaries: List<HistoricalSummary, 16777216>,
}

impl From<ReamBeaconState> for BeaconState {
    fn from(state: ReamBeaconState) -> Self {
        let mut validators = List::<Validator, 1099511627776>::default();
        for v in state.validators.iter() {
            validators.push(v.clone().into());
        }

        let mut eth1_data_votes = List::<Eth1Data, 2048>::default();
        for v in state.eth1_data_votes.iter() {
            eth1_data_votes.push(v.clone().into());
        }

        let mut block_roots = Vector::<FixedBytes<32>, 8192>::default();
        for (i, v) in state.block_roots.iter().enumerate() {
            block_roots[i] = v.clone().as_slice().try_into().unwrap();
        }

        let mut state_roots = Vector::<FixedBytes<32>, 8192>::default();
        for (i, v) in state.state_roots.iter().enumerate() {
            state_roots[i] = v.clone().as_slice().try_into().unwrap();
        }

        let mut historical_roots = List::<FixedBytes<32>, 16777216>::default();
        for v in state.historical_roots.iter() {
            historical_roots.push(v.clone().as_slice().try_into().unwrap());
        }

        let mut balances = List::<u64, 1099511627776>::default();
        for v in state.balances.iter() {
            balances.push(*v);
        }

        let mut randao_mixes = Vector::<FixedBytes<32>, 65536>::default();
        for (i, v) in state.randao_mixes.iter().enumerate() {
            randao_mixes[i] = v.clone().as_slice().try_into().unwrap();
        }

        let mut slashings = Vector::<u64, 8192>::default();
        for (i, v) in state.slashings.iter().enumerate() {
            slashings[i] = *v;
        }

        let mut previous_epoch_participation = List::<u8, 1099511627776>::default();
        for v in state.previous_epoch_participation.iter() {
            previous_epoch_participation.push(*v);
        }

        let mut current_epoch_participation = List::<u8, 1099511627776>::default();
        for v in state.current_epoch_participation.iter() {
            current_epoch_participation.push(*v);
        }

        let justification_bits =
            Bitvector::<4>::try_from(state.justification_bits.as_slice()).unwrap();

        let mut historical_summaries = List::<HistoricalSummary, 16777216>::default();
        for v in state.historical_summaries.iter() {
            historical_summaries.push(v.clone().into());
        }

        BeaconState {
            // Versioning
            genesis_time: state.genesis_time,
            genesis_validators_root: state.genesis_validators_root.as_slice().try_into().unwrap(),
            slot: state.slot,
            fork: state.fork.into(),

            // History
            latest_block_header: state.latest_block_header.into(),
            block_roots,
            state_roots,
            historical_roots,
            // Frozen in Capella, replaced by historical_summaries
            // pub historical_roots: VariableList<B256, U16777216>,

            // Eth1
            eth1_data: state.eth1_data.into(),
            eth1_data_votes,
            eth1_deposit_index: state.eth1_deposit_index,

            // Registry
            validators,
            balances,

            // Randomness
            randao_mixes,

            // Slashings
            slashings,

            // Participation
            previous_epoch_participation,
            current_epoch_participation,

            // Finality
            justification_bits,
            previous_justified_checkpoint: state.previous_justified_checkpoint.into(),
            current_justified_checkpoint: state.current_justified_checkpoint.into(),
            finalized_checkpoint: state.finalized_checkpoint.into(),

            // Inactivity
            inactivity_scores: List::<u64, 1099511627776>::try_from(
                state.inactivity_scores.to_vec(),
            )
            .unwrap(),

            // Sync
            current_sync_committee: (*state.current_sync_committee.clone()).clone().into(),
            next_sync_committee: (*state.next_sync_committee.clone()).clone().into(),

            // Execution
            latest_execution_payload_header: state.latest_execution_payload_header.into(),

            // Withdrawals
            next_withdrawal_index: state.next_withdrawal_index,
            next_withdrawal_validator_index: state.next_withdrawal_validator_index,
            // Deep history valid from Capella onwards.
            historical_summaries,
        }
    }
}

impl BeaconState {
    // Mock function for consenzero PoC
    pub fn get_beacon_proposer_index(&self) -> anyhow::Result<u64> {
        Ok(174)
    }
}
