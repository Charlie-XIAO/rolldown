use derive_more::Debug;
use rolldown_common::ClientHmrUpdate;
use std::sync::Arc;

pub type OnHmrUpdatesCallback = Arc<dyn Fn(Vec<ClientHmrUpdate>, Vec<String>) + Send + Sync>;

pub type SharedNormalizedDevOptions = Arc<NormalizedDevOptions>;

#[derive(Debug, Default)]
pub struct DevWatchOptions {
  /// If `true`, files are not written to disk.
  pub skip_write: Option<bool>,
  /// If `true`, use polling instead of native file system events for watching
  pub use_polling: Option<bool>,
  /// Poll interval in milliseconds (only used when use_polling is true)
  pub poll_interval: Option<u64>,
  /// If `true`, use debounced watcher. If `false`, use non-debounced watcher
  pub use_debounce: Option<bool>,
  /// Debounce duration in milliseconds (only used when use_debounce is true)
  pub debounce_duration: Option<u64>,
  /// Whether to compare file contents for poll-based watchers (only used when use_polling is true)
  pub compare_contents_for_polling: Option<bool>,
  /// Tick rate in milliseconds for debounced watchers (only used when use_debounce is true)
  pub debounce_tick_rate: Option<u64>,
}

#[derive(Debug, Default)]
pub struct DevOptions {
  #[debug(skip)]
  pub on_hmr_updates: Option<OnHmrUpdatesCallback>,
  /// If `true`, A rebuild will be always issued when a file changes.
  pub eager_rebuild: Option<bool>,
  pub watch: Option<DevWatchOptions>,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct NormalizedDevOptions {
  #[debug(skip)]
  pub on_hmr_updates: Option<OnHmrUpdatesCallback>,
  pub skip_write: bool,
  pub eager_rebuild: bool,
  pub use_polling: bool,
  pub poll_interval: u64,
  pub use_debounce: bool,
  pub debounce_duration: u64,
  pub compare_contents_for_polling: bool,
  pub debounce_tick_rate: Option<u64>,
}

pub fn normalize_dev_options(options: DevOptions) -> NormalizedDevOptions {
  let watch_options = options.watch.unwrap_or_default();
  NormalizedDevOptions {
    on_hmr_updates: options.on_hmr_updates,
    skip_write: watch_options.skip_write.unwrap_or_default(),
    eager_rebuild: options.eager_rebuild.unwrap_or_default(),
    use_polling: watch_options.use_polling.unwrap_or(false),
    poll_interval: watch_options.poll_interval.unwrap_or(100),
    use_debounce: watch_options.use_debounce.unwrap_or(true),
    debounce_duration: watch_options.debounce_duration.unwrap_or(10),
    compare_contents_for_polling: watch_options.compare_contents_for_polling.unwrap_or(false),
    debounce_tick_rate: watch_options.debounce_tick_rate,
  }
}
