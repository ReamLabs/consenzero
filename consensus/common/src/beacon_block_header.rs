use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;

use ream_consensus::beacon_block_header::BeaconBlockHeader as ReamBeaconBlockHeader;

#[derive(Debug, SimpleSerialize)]
pub struct BeaconBlockHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: FixedBytes<32>,
    pub state_root: FixedBytes<32>,
    pub body_root: FixedBytes<32>,
}

impl From<ReamBeaconBlockHeader> for BeaconBlockHeader {
    fn from(header: ReamBeaconBlockHeader) -> Self {
        BeaconBlockHeader {
            slot: header.slot,
            proposer_index: header.proposer_index,
            parent_root: header.parent_root.as_slice().try_into().unwrap(),
            state_root: header.state_root.as_slice().try_into().unwrap(),
            body_root: header.body_root.as_slice().try_into().unwrap(),
        }
    }
}
