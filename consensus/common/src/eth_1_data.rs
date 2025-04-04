use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;

use ream_consensus::eth_1_data::Eth1Data as ReamEth1Data;

#[derive(Debug, SimpleSerialize, serde::Serialize, serde::Deserialize)]
pub struct Eth1Data {
    pub deposit_root: FixedBytes<32>,
    pub deposit_count: u64,
    pub block_hash: FixedBytes<32>,
}

impl From<ReamEth1Data> for Eth1Data {
    fn from(data: ReamEth1Data) -> Self {
        Eth1Data {
            deposit_root: data.deposit_root.as_slice().try_into().unwrap(),
            deposit_count: data.deposit_count,
            block_hash: data.block_hash.as_slice().try_into().unwrap(),
        }
    }
}
