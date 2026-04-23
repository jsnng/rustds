//! UTF-16LE decode throughput — every NChar / NVarChar / NText cell goes through
//! `chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]]))`.
//!
//! Two scenarios:
//!   - `scalar_sum`: raw byte pairs -> u16 -> sum. Measures pure decode speed;
//!     the reference for any SIMD replacement.
//!   - `to_string`: full `NVarCharSpan` Display path into a reused String
//!     (surrogate handling + writer). Closer to user-facing path.

use core::fmt::Write;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use bronotdsaurs::tds::types::prelude::NVarCharSpan;

const SIZES: &[usize] = &[8, 64, 4_096, 65_536];

fn make_ascii_utf16le(chars: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(chars * 2);
    for i in 0..chars {
        v.push(b'A' + (i as u8 % 26));
        v.push(0);
    }
    v
}

#[inline]
fn sum_u16s(buf: &[u8]) -> u32 {
    let mut s = 0u32;
    for c in buf.chunks_exact(2) {
        s = s.wrapping_add(u16::from_le_bytes([c[0], c[1]]) as u32);
    }
    s
}

fn bench_utf16(c: &mut Criterion) {
    let mut g = c.benchmark_group("utf16_decode");

    for &chars in SIZES {
        let buf = make_ascii_utf16le(chars);
        g.throughput(Throughput::Bytes(buf.len() as u64));
        g.bench_with_input(BenchmarkId::new("scalar_sum", chars), &buf, |b, buf| {
            b.iter(|| sum_u16s(black_box(buf.as_slice())));
        });
    }

    for &chars in SIZES {
        let buf = make_ascii_utf16le(chars);
        let mut sink = String::with_capacity(chars);
        g.throughput(Throughput::Bytes(buf.len() as u64));
        g.bench_with_input(BenchmarkId::new("to_string", chars), &buf, |b, buf| {
            b.iter(|| {
                sink.clear();
                let span = NVarCharSpan::new(black_box(buf.as_slice()));
                write!(&mut sink, "{}", span).unwrap();
                black_box(sink.len());
            });
        });
    }

    g.finish();
}

criterion_group!(benches, bench_utf16);
criterion_main!(benches);
