use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Window, WorkerGlobalScope};

thread_local! {
    pub(crate) static GLOBAL: WindowOrWorker = WindowOrWorker::new();
}

pub(crate) enum WindowOrWorker {
    Window(Window),
    Worker(WorkerGlobalScope),
}

impl WindowOrWorker {
    pub(crate) fn new() -> Self {
        #[wasm_bindgen]
        extern "C" {
            type Global;

            #[wasm_bindgen(method, getter, js_name = Window)]
            fn window(this: &Global) -> JsValue;

            #[wasm_bindgen(method, getter, js_name = WorkerGlobalScope)]
            fn worker(this: &Global) -> JsValue;
        }

        let global: Global = js_sys::global().unchecked_into();

        if !global.window().is_undefined() {
            Self::Window(global.unchecked_into())
        } else if !global.worker().is_undefined() {
            Self::Worker(global.unchecked_into())
        } else {
            panic!("Only supported in a browser or web worker");
        }
    }
}

macro_rules! impl_window_or_worker {
    ($(fn $name:ident($($par_name:ident: $par_type:ty),*)$( -> $return:ty)?;)+) => {
        use crate::wasm::client::util::WindowOrWorker;
        impl WindowOrWorker {
            $(
                pub(crate) fn $name(&self, $($par_name: $par_type),*)$( -> $return)? {
                    match self {
                        Self::Window(window) => window.$name($($par_name),*),
                        Self::Worker(worker) => worker.$name($($par_name),*),
                    }
                }
            )+
        }
    };
}
