use ream_consensus::validator::Validator;

pub struct MerklelizedValidator {
  validator: Validator,
  proof: u32,
}
