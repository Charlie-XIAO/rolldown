use bench::{DeriveOptions, derive_benchmark_items};
use criterion::{Criterion, criterion_group, criterion_main};

use rolldown_common::{BundlerOptions, ScanMode};
use rolldown_testing::bundler_options_presets::{rome_ts, threejs};

fn items() -> Vec<(&'static str, BundlerOptions)> {
  vec![
    ("threejs", threejs()),
    ("rome_ts", rome_ts()),
    #[cfg(not(feature = "codspeed"))]
    ("threejs10x", rolldown_testing::bundler_options_presets::threejs10x()),
  ]
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("scan");
  let derive_options = DeriveOptions { sourcemap: false, minify: false };
  items()
    .into_iter()
    .flat_map(|(name, options)| derive_benchmark_items(&derive_options, name, options.clone()))
    .for_each(|item| {
      group.bench_function(format!("scan@{}", item.name), move |b| {
        b.to_async(
          tokio::runtime::Builder::new_multi_thread()
            .worker_threads(8)
            .enable_all()
            .max_blocking_threads(4)
            .build()
            .unwrap(),
        )
        .iter(|| async {
          let mut rolldown_bundler =
            rolldown::Bundler::new(item.options.clone()).expect("Failed to create bundler");
          let _output =
            rolldown_bundler.scan(ScanMode::Full).await.expect("should not failed in scan");
        });
      });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
