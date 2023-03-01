use super::types::*;
use fp_bindgen_support::{
    common::{abi::WasmAbi, mem::FatPtr},
    host::{
        errors::{InvocationError, RuntimeError},
        mem::{
            deserialize_from_slice, export_to_guest, export_to_guest_raw, import_from_guest,
            import_from_guest_raw, serialize_to_vec,
        },
        r#async::{create_future_value, future::ModuleRawFuture, resolve_async_value},
        runtime::RuntimeInstanceData,
    },
};
use std::sync::Arc;
use wasmer::{
    imports, AsStoreMut, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Module, Store,
};

pub struct Runtime {
    store: Store,
    instance: Instance,
    env: FunctionEnv<Arc<RuntimeInstanceData>>,
}

impl Runtime {
    pub fn new(wasm_module: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let mut store = Self::default_store();
        let module = Module::new(&store, wasm_module)?;
        let env = FunctionEnv::new(&mut store, Arc::new(RuntimeInstanceData::default()));
        let import_object = create_imports(&mut store, &env);
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();
        let env_from_instance = RuntimeInstanceData::from_instance(&mut store, &instance);
        Arc::get_mut(env.as_mut(&mut store))
            .unwrap()
            .copy_from(env_from_instance);
        Ok(Self {
            store,
            instance,
            env,
        })
    }

    fn default_store() -> wasmer::Store {
        Store::new(wasmer_compiler_singlepass::Singlepass::default())
    }

    fn function_env_mut(&mut self) -> FunctionEnvMut<Arc<RuntimeInstanceData>> {
        self.env.clone().into_mut(&mut self.store)
    }

    pub fn update(&mut self, data: Vec<u8>) -> Result<Vec<u8>, InvocationError> {
        let data = serialize_to_vec(&data);
        let result = self.update_raw(data);
        let result = result.map(|ref data| deserialize_from_slice(data));
        result
    }
    pub fn update_raw(&mut self, data: Vec<u8>) -> Result<Vec<u8>, InvocationError> {
        let data = export_to_guest_raw(&mut self.function_env_mut(), data);
        let function = self
            .instance
            .exports
            .get_typed_function::<FatPtr, FatPtr>(&mut self.store, "__fp_gen_update")
            .map_err(|_| InvocationError::FunctionNotExported("__fp_gen_update".to_owned()))?;
        let result = function.call(&mut self.store, data.to_abi())?;
        let result = import_from_guest_raw(&mut self.function_env_mut(), result);
        Ok(result)
    }
}

fn create_imports(store: &mut Store, env: &FunctionEnv<Arc<RuntimeInstanceData>>) -> Imports {
    imports! {
        "fp" => {
            "__fp_host_resolve_async_value" => Function::new_typed_with_env(store, env, resolve_async_value),

        }
    }
}
