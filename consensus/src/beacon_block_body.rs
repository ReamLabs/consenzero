use ssz_rs::prelude::*;

use ream_consensus::deneb::beacon_block_body::BeaconBlockBody as ReamBeaconBlockBody;

use crate::bls::BLSSignature;
use crate::eth_1_data::Eth1Data;

#[derive(Debug, SimpleSerialize, serde::Serialize, serde::Deserialize)]
pub struct BeaconBlockBody {
    pub randao_reveal: BLSSignature,
    pub eth1_data: Eth1Data,
    pub graffiti: Vector<u8, 32>,
    // pub proposer_slashings: VariableList<ProposerSlashing, U16>,
    // pub attester_slashings: VariableList<AttesterSlashing, U2>,
    // pub attestations: VariableList<Attestation, U128>,
    // pub deposits: VariableList<Deposit, U16>,
    // pub voluntary_exits: VariableList<SignedVoluntaryExit, U16>,
    // pub sync_aggregate: SyncAggregate,
    // pub execution_payload: ExecutionPayload,
    // pub bls_to_execution_changes: VariableList<SignedBLSToExecutionChange, U16>,
    // pub blob_kzg_commitments: VariableList<KZGCommitment, U4096>,
}

impl From<ReamBeaconBlockBody> for BeaconBlockBody {
    fn from(body: ReamBeaconBlockBody) -> Self {
        BeaconBlockBody {
            randao_reveal: body.randao_reveal.into(),
            eth1_data: body.eth1_data.into(),
            graffiti: body.graffiti.as_slice().try_into().unwrap(),
        //     pub proposer_slashings: VariableList<ProposerSlashing, U16>,
        //     pub attester_slashings: VariableList<AttesterSlashing, U2>,
        //     pub attestations: VariableList<Attestation, U128>,
        //     pub deposits: VariableList<Deposit, U16>,
        //     pub voluntary_exits: VariableList<SignedVoluntaryExit, U16>,
        //     pub sync_aggregate: SyncAggregate,
        //     pub execution_payload: ExecutionPayload,
        //     pub bls_to_execution_changes: VariableList<SignedBLSToExecutionChange, U16>,
        //     pub blob_kzg_commitments: VariableList<KZGCommitment, U4096>,
        }
    }
}
