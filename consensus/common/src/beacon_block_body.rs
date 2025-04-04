use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;

use crate::bls::BLSSignature;
use crate::eth_1_data::Eth1Data;
use crate::execution_payload::ExecutionPayload;
use crate::sync_aggregate::SyncAggregate;
use crate::attestation::Attestation;
use crate::attester_slashing::AttesterSlashing;
use crate::proposer_slashing::ProposerSlashing;
use crate::deposit::Deposit;
use crate::voluntary_exit::SignedVoluntaryExit;
use crate::bls_to_execution_change::SignedBLSToExecutionChange;
use crate::kzg::KZGCommitment;

#[derive(Debug, SimpleSerialize)]
pub struct BeaconBlockBody {
    pub randao_reveal: BLSSignature,
    pub eth1_data: Eth1Data,
    pub graffiti: FixedBytes<32>,
    pub proposer_slashings: List<ProposerSlashing, 16>,
    pub attester_slashings: List<AttesterSlashing, 2>,
    pub attestations: List<Attestation, 128>,
    pub deposits: List<Deposit, 16>,
    pub voluntary_exits: List<SignedVoluntaryExit, 16>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
    pub bls_to_execution_changes: List<SignedBLSToExecutionChange, 16>,
    pub blob_kzg_commitments: List<KZGCommitment, 4096>,
}

impl From<ream_consensus::deneb::beacon_block_body::BeaconBlockBody> for BeaconBlockBody {
    fn from(body: ream_consensus::deneb::beacon_block_body::BeaconBlockBody) -> Self {
        let mut proposer_slashings = List::<ProposerSlashing, 16>::default();
        for v in body.proposer_slashings.iter() {
            proposer_slashings.push(v.clone().into());
        }

        let mut attester_slashings = List::<AttesterSlashing, 2>::default();
        for v in body.attester_slashings.iter() {
            attester_slashings.push(v.clone().into());
        }

        let mut attestations = List::<Attestation, 128>::default();
        for v in body.attestations.iter() {
            attestations.push(v.clone().into());
        }

        let mut deposits = List::<Deposit, 16>::default();
        for v in body.deposits.iter() {
            deposits.push(v.clone().into());
        }

        let mut voluntary_exits = List::<SignedVoluntaryExit, 16>::default();
        for v in body.voluntary_exits.iter() {
            voluntary_exits.push(v.clone().into());
        }

        let mut bls_to_execution_changes = List::<SignedBLSToExecutionChange, 16>::default();
        for v in body.bls_to_execution_changes.iter() {
            bls_to_execution_changes.push(v.clone().into());
        }

        let mut blob_kzg_commitments = List::<KZGCommitment, 4096>::default();
        for v in body.blob_kzg_commitments.iter() {
            blob_kzg_commitments.push(v.clone().into());
        }

        BeaconBlockBody {
            randao_reveal: body.randao_reveal.into(),
            eth1_data: body.eth1_data.into(),
            graffiti: body.graffiti.as_slice().try_into().unwrap(),
            proposer_slashings,
            attester_slashings,
            attestations,
            deposits,
            voluntary_exits,
            sync_aggregate: body.sync_aggregate.into(),
            execution_payload: body.execution_payload.into(),
            bls_to_execution_changes,
            blob_kzg_commitments,
        }
    }
}
