// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

// This benchmark is here because criterion has a higher MSRV than camino -- camino-examples is only
// tested on stable, which is good enough.

use camino::Utf8PathBuf;
use criterion::*;

fn bench_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("Path");
    for i in [10, 100, 1000, 10000] {
        let p = "i".repeat(i);
        let buf = Utf8PathBuf::from(black_box(p));
        group.bench_with_input(BenchmarkId::new("Utf8PathBuf::as_str", i), &buf, |b, i| {
            b.iter(|| {
                let _ = black_box(&i).as_str();
            })
        });
    }
}

criterion_group!(benches, bench_path);
criterion_main!(benches);
