use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;

use ream_consensus::validator::Validator as ReamValidator;
use crate::bls::PubKey;

#[derive(Debug, SimpleSerialize)]
pub struct Validator {
    pub pubkey: PubKey,

    /// Commitment to pubkey for withdrawals
    pub withdrawal_credentials: FixedBytes<32>,

    /// Balance at stake
    pub effective_balance: u64,
    pub slashed: bool,

    /// When criteria for activation were met
    pub activation_eligibility_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: u64,

    /// When validator can withdraw funds
    pub withdrawable_epoch: u64,
}

impl From<ReamValidator> for Validator {
    fn from(validator: ReamValidator) -> Self {
        Validator {
            pubkey: validator.pubkey.into(),

            withdrawal_credentials: validator.withdrawal_credentials.into(),

            effective_balance: validator.effective_balance,
            slashed: validator.slashed,

            activation_eligibility_epoch: validator.activation_eligibility_epoch,
            activation_epoch: validator.activation_epoch,
            exit_epoch: validator.exit_epoch,

            withdrawable_epoch: validator.withdrawable_epoch,
        }
    }
}
