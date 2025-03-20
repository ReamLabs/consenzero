use alloy_primitives::FixedBytes;
use ssz_rs::{GeneralizedIndex, MerkleizationError, Node};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Proof {
    pub leaf: Node,
    pub branch: Vec<Node>,
    pub index: GeneralizedIndex,
    pub witness: FixedBytes<32>,
}

impl Proof {
    pub fn new(proof: ssz_rs::proofs::Proof, witness: FixedBytes<32>) -> Self {
        Self {
            leaf: proof.leaf,
            branch: proof.branch,
            index: proof.index,
            witness,
        }
    }

    pub fn verify(&self) -> Result<(), MerkleizationError> {
        let proof = ssz_rs::proofs::Proof {
            leaf: self.leaf,
            branch: self.branch.clone(),
            index: self.index,
        };

        proof.verify(self.witness)
    }
}
