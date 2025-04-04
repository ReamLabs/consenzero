use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;
use crate::bls::{PubKey, BLSSignature};

#[derive(Debug, SimpleSerialize)]
pub struct DepositData {
    pub pubkey: PubKey,
    pub withdrawal_credentials: FixedBytes<32>,
    pub amount: u64,
    pub signature: BLSSignature,
}

impl From<ream_consensus::deposit_data::DepositData> for DepositData {
    fn from(data: ream_consensus::deposit_data::DepositData) -> Self {
        DepositData {
            pubkey: data.pubkey.into(),
            withdrawal_credentials: data.withdrawal_credentials.into(),
            amount: data.amount,
            signature: data.signature.into(),
        }
    }
}

#[derive(Debug, SimpleSerialize)]
pub struct Deposit {
    pub proof: Vector<Vector<u8, 32>, 33>,
    pub data: DepositData,
}

impl From<ream_consensus::deposit::Deposit> for Deposit {
    fn from(deposit: ream_consensus::deposit::Deposit) -> Self {
        let mut proof = Vector::<Vector<u8, 32>, 33>::default();
        for (i, v) in deposit.proof.iter().enumerate() {
            proof[i] = v.clone().as_slice().try_into().unwrap();
        }

        Deposit {
            proof,
            data: deposit.data.into(),
        }
    }
} 