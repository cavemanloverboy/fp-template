use std::collections::{BTreeMap, BTreeSet};

use egui::Frame;
use fp_bindgen::{prelude::*, types::CargoDependency};
use once_cell::sync::Lazy;

// Functions and types that we want our wasm plugin to define
pub struct FrameWrapper(Frame);

fp_export! {
    fn update(data: Vec<u8>) -> Vec<u8>;
}

fp_import! {}

// impl Serializable for FrameWrapper {
//     fn ident() -> TypeIdent {
//         TypeIdent::new("FrameWrapper", vec![])
//     }

//     fn ty() -> Type {
//         Type::Container("FrameWrapper".to_owned(), "Frame".into())
//     }
// }

// impl FrameWrapper {
//     pub fn unwrap(self) -> Frame {
//         self.0
//     }
// }

const VERSION: &str = "1.0.0";
const AUTHORS: &str = r#"["Cavey Cool <caveycool@gmail.com>"]"#;
const NAME: &str = "cartridge-bindings";

static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
    BTreeMap::from([
        (
            "fp-bindgen-support",
            CargoDependency {
                // path: Some("../../../../fp-bindgen-support"),
                version: Some("2.4.0"),
                features: BTreeSet::from(["async", "guest"]),
                ..CargoDependency::default()
            },
        ),
        (
            "egui",
            CargoDependency {
                version: Some("0.20.1"),
                features: BTreeSet::from(["persistence"]),
                ..CargoDependency::default()
            },
        ),
    ])
});

fn main() {
    for bindings_type in [
        BindingsType::RustPlugin(RustPluginConfig {
            name: NAME,
            authors: AUTHORS,
            version: VERSION,
            dependencies: PLUGIN_DEPENDENCIES.clone(),
        }),
        BindingsType::RustWasmerRuntime,
        BindingsType::RustWasmerWasiRuntime,
        // BindingsType::TsRuntimeWithExtendedConfig(
        //     TsExtendedRuntimeConfig::new()
        //         .with_msgpack_module(
        //             "https://unpkg.com/@msgpack/msgpack@2.7.2/mod.ts",
        //         )
        //         .with_raw_export_wrappers(),
        // ),
    ] {
        let output_path = format!("bindings/{}", bindings_type);

        fp_bindgen!(BindingConfig {
            bindings_type,
            path: &output_path,
        });
        println!("Generated bindings written to `{}/`.", output_path);
    }
}
