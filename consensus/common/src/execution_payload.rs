use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;

#[derive(Debug, SimpleSerialize)]
pub struct ExecutionPayload {
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
    pub transactions: List<List<u8, 1073741824>, 1048576>,
    pub withdrawals: List<Withdrawal, 16>,
    pub blob_gas_used: u64,
    pub excess_blob_gas: u64,
}

#[derive(Debug, SimpleSerialize)]
pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: Vector<u8, 20>,
    pub amount: u64,
}

impl From<ream_consensus::withdrawal::Withdrawal> for Withdrawal {
    fn from(withdrawal: ream_consensus::withdrawal::Withdrawal) -> Self {
        Withdrawal {
            index: withdrawal.index,
            validator_index: withdrawal.validator_index,
            address: withdrawal.address.as_slice().try_into().unwrap(),
            amount: withdrawal.amount,
        }
    }
}

impl From<ream_consensus::deneb::execution_payload::ExecutionPayload> for ExecutionPayload {
    fn from(payload: ream_consensus::deneb::execution_payload::ExecutionPayload) -> Self {
        let mut logs_bloom = Vector::<u8, 256>::default();
        for (i, v) in payload.logs_bloom.iter().enumerate() {
            logs_bloom[i] = *v;
        }

        let mut extra_data = List::<u8, 32>::default();
        for v in payload.extra_data.iter() {
            extra_data.push(*v);
        }

        let mut transactions = List::<List<u8, 1073741824>, 1048576>::default();
        for v in payload.transactions.iter() {
            let mut transaction = List::<u8, 1073741824>::default();
            for b in v.iter() {
                transaction.push(*b);
            }
            transactions.push(transaction);
        }

        let mut withdrawals = List::<Withdrawal, 16>::default();
        for v in payload.withdrawals.iter() {
            withdrawals.push(v.clone().into());
        }

        ExecutionPayload {
            parent_hash: payload.parent_hash.into(),
            fee_recipient: payload.fee_recipient.as_slice().try_into().unwrap(),
            state_root: payload.state_root.into(),
            receipts_root: payload.receipts_root.into(),
            logs_bloom,
            prev_randao: payload.prev_randao.into(),
            block_number: payload.block_number,
            gas_limit: payload.gas_limit,
            gas_used: payload.gas_used,
            timestamp: payload.timestamp,
            extra_data,
            base_fee_per_gas: payload.base_fee_per_gas.try_into().unwrap(),
            block_hash: payload.block_hash.into(),
            transactions,
            withdrawals,
            blob_gas_used: payload.blob_gas_used,
            excess_blob_gas: payload.excess_blob_gas,
        }
    }
} 