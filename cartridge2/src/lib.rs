#[rustfmt::skip]
mod export;
#[rustfmt::skip]
mod import;
#[rustfmt::skip]
mod types;

pub use export::*;
pub use import::*;
pub use types::*;

pub use fp_bindgen_support::*;

#[fp_export_impl(cartridge_bindings)]
// this impl is an add!
fn update(data: Vec<u8>) -> Vec<u8> {
    let mut data = data;
    for byte in data.iter_mut() {
        *byte = byte.wrapping_mul(222);
    }
    data
}
