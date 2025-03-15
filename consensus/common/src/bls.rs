use ssz_rs::prelude::*;

use ream_bls::BLSSignature as ReamBLSSignature;
use ream_bls::PubKey as ReamPubKey;

#[derive(Debug, SimpleSerialize)]
pub struct BLSSignature {
    pub inner: [u8; 96],
}

impl From<ReamBLSSignature> for BLSSignature {
    fn from(sig: ReamBLSSignature) -> Self {
        let inner_vec: Vec<u8> = sig.inner.into();

        BLSSignature {
            inner: inner_vec.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, SimpleSerialize)]
pub struct PubKey {
    pub inner: [u8; 48],
}

impl Default for PubKey {
    fn default() -> Self {
        Self { inner: [0; 48] }
    }
}

impl From<ReamPubKey> for PubKey {
    fn from(pubkey: ReamPubKey) -> Self {
        let inner_vec: Vec<u8> = pubkey.inner.into();

        PubKey {
            inner: inner_vec.try_into().unwrap(),
        }
    }
}
