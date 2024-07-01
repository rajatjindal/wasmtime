use crate::bindings::cli::environment;
use crate::{WasiImpl, WasiView};

impl<T> environment::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn get_environment(&mut self) -> anyhow::Result<Vec<(String, String)>> {
        Ok(self.ctx().env.clone())
    }
    fn get_arguments(&mut self) -> anyhow::Result<Vec<String>> {
        Ok(self.ctx().args.clone())
    }
    fn hello(&mut self) -> anyhow::Result<Result<String, crate::bindings::cli::environment::Error>> {
        Ok(Ok("hello from wasmtime".to_string()))
    }
    
    fn justhello(&mut self) -> anyhow::Result<String> {
        Ok("justhello from wasmtime".to_string())
    }

    fn initial_cwd(&mut self) -> anyhow::Result<Option<String>> {
        // FIXME: expose cwd in builder and save in ctx
        Ok(Some("whatever".to_string()))
    }
}
