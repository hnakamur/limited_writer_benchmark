use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use limited_writer_benchmark::{LimitedWriter, LimitedWriter2};
use std::fmt::Write;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("LimitedWriter");
    struct Input<'a> {
        limit: usize,
        s: &'a str,
    }
    const INPUTS: [Input; 8] = [
        Input { limit: 0, s: "" },
        Input {
            limit: 1,
            s: "\u{0024}",
        },
        Input {
            limit: 2,
            s: "\u{0024}",
        },
        Input {
            limit: 3,
            s: "\u{0024}\u{00A2}",
        },
        Input {
            limit: 5,
            s: "\u{0024}\u{00A2}",
        },
        Input {
            limit: 6,
            s: "\u{0024}\u{00A2}\u{20AC}",
        },
        Input {
            limit: 9,
            s: "\u{0024}\u{00A2}\u{20AC}",
        },
        Input {
            limit: 10,
            s: "\u{0024}\u{00A2}\u{20AC}\u{10348}",
        },
    ];
    for (id, input) in INPUTS.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("len_utf8_at", id), &input, |b, i| {
            b.iter(|| {
                let mut buf = String::new();
                let mut w = LimitedWriter::new(&mut buf, i.limit);
                write!(&mut w, "{}", i.s).unwrap();
            })
        });
        group.bench_with_input(BenchmarkId::new("char_indices", id), &input, |b, i| {
            b.iter(|| {
                let mut buf = String::new();
                let mut w = LimitedWriter2::new(&mut buf, i.limit);
                write!(&mut w, "{}", i.s).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
