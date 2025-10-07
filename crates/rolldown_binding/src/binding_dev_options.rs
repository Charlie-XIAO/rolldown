use crate::types::binding_client_hmr_update::BindingClientHmrUpdate;
use crate::types::binding_rebuild_strategy::BindingRebuildStrategy;
use crate::types::js_callback::JsCallback;
use napi::bindgen_prelude::FnArgs;
use napi_derive::napi;

#[napi(object, object_to_js = false)]
pub struct BindingDevWatchOptions {
  pub skip_write: Option<bool>,
  pub use_polling: Option<bool>,
  pub poll_interval: Option<u32>,
  pub use_debounce: Option<bool>,
  pub debounce_duration: Option<u32>,
  pub compare_contents_for_polling: Option<bool>,
  pub debounce_tick_rate: Option<u32>,
}

#[napi(object, object_to_js = false)]
pub struct BindingDevOptions {
  #[napi(
    ts_type = "undefined | ((updates: BindingClientHmrUpdate[], changedFiles: string[]) => void | Promise<void>)"
  )]
  pub on_hmr_updates: Option<JsCallback<FnArgs<(Vec<BindingClientHmrUpdate>, Vec<String>)>, ()>>,
  pub rebuild_strategy: Option<BindingRebuildStrategy>,
  pub watch: Option<BindingDevWatchOptions>,
}
