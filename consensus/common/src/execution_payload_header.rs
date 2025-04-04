use alloy_primitives::FixedBytes;
use ream_consensus::deneb::execution_payload_header::ExecutionPayloadHeader as ReamExecutionPayloadHeader;

use ssz_rs::prelude::*;

#[derive(Debug, SimpleSerialize)]
pub struct ExecutionPayloadHeader {
    pub parent_hash: FixedBytes<32>,
    pub fee_recipient: Vector<u8, 20>,
    pub state_root: FixedBytes<32>,
    pub receipts_root: FixedBytes<32>,
    pub logs_bloom: Vector<u8, 256>,
    pub prev_randao: FixedBytes<32>,
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: List<u8, 32>,
    pub base_fee_per_gas: u64,
    pub block_hash: FixedBytes<32>,
    pub transactions_root: FixedBytes<32>,
    pub withdrawals_root: FixedBytes<32>,
    pub blob_gas_used: u64,
    pub excess_blob_gas: u64,
}

impl From<ReamExecutionPayloadHeader> for ExecutionPayloadHeader {
    fn from(header: ReamExecutionPayloadHeader) -> Self {
        let mut logs_bloom = Vector::<u8, 256>::default();
        for (i, v) in header.logs_bloom.iter().enumerate() {
            logs_bloom[i] = *v;
        }

        let mut extra_data = List::<u8, 32>::default();
        for v in header.extra_data.iter() {
            extra_data.push(*v);
        }

        ExecutionPayloadHeader {
            parent_hash: header.parent_hash.into(),
                fee_recipient: header.fee_recipient.as_slice().try_into().unwrap(),
            state_root: header.state_root.into(),
            receipts_root: header.receipts_root.into(),
            logs_bloom,
            prev_randao: header.prev_randao.into(),
            block_number: header.block_number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data,
            base_fee_per_gas: header.base_fee_per_gas.try_into().unwrap(),
            block_hash: header.block_hash.into(),
            transactions_root: header.transactions_root.into(),
            withdrawals_root: header.withdrawals_root.into(),
            blob_gas_used: header.blob_gas_used,
            excess_blob_gas: header.excess_blob_gas,
        }
    }
}
