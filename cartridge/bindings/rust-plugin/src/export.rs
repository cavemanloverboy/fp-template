use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn update(data: Vec<u8>) -> Vec<u8>;
