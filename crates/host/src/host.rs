use crate::bindings::acme::plugins::log;

use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

//
// Host
//

/// Plugins host.
pub struct Host {
    wasi: WasiCtx,
    resources: ResourceTable,
}

impl Host {
    /// Constructor.
    pub fn new() -> Self {
        let wasi = WasiCtxBuilder::new()
        .inherit_stdout() // 允许模块打印到宿主的标准输出
        .inherit_stderr() // 允许模块打印到宿主的标准错误
        // 不挂载任何目录，防止文件系统访问
        .build();
        Self { wasi, resources: ResourceTable::new() }
    }
}

// We need to implement WasiView for wasmtime_wasi::add_to_linker_sync
impl WasiView for Host {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl IoView for Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

// Our exposed Host functions
impl log::Host for Host {
    fn log(&mut self, message: String) {
        println!("log: {}", message);
    }
}



use crate::bindings::local::val::types::Val;