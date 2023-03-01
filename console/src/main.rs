mod bindings;
mod types;

use anyhow::Result;
use bindings::*;
use types::*;

const WASM_BYTES: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/debug/cartridge1.wasm");

const WASM_BYTES_2: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/debug/cartridge2.wasm");

fn main() -> Result<()> {
    let bytes: Vec<u8> = vec![0, 5, 128];

    println!("running cartridge1");
    let mut rt = Runtime::new(WASM_BYTES).unwrap();
    let updated_bytes = rt.update(bytes.clone())?;
    println!("ran cartridge2");

    println!("running cartridge2");
    let mut rt = Runtime::new(WASM_BYTES_2).unwrap();
    let updated_bytes_2 = rt.update(bytes.clone())?;
    println!("ran cartridge2");

    println!("results:");
    println!("    bytes = {bytes:?}");
    println!("    post1 = {updated_bytes:?}");
    println!("    post2 = {updated_bytes_2:?}");

    Ok(())
}
