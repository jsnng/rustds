//! Per-row stride walk — the hottest per-cell function in the decoder.
//!
//! Stride encoding (see `tds::types::tokens::col_metadata::walk`):
//!   0x00          -> zero-length cell (no bytes)
//!   0x01..=0x7F   -> fixed cell of N bytes
//!   0x80 | N      -> variable cell with N-byte length prefix (N in {1,2,4})
//!
//! Fixtures cover:
//!   - all-fixed (Int4)            -> branch always takes the "fixed" arm
//!   - all-variable (NVarChar)     -> branch always takes the "variable" arm
//!   - mixed                       -> worst case for branch prediction

use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use bronotdsaurs::tds::decoder::stream::{Drainable, Row};

const CELL_DATA_LEN: usize = 16; // bytes of payload inside each variable cell

fn build_row(strides: &[u8]) -> Vec<u8> {
    let mut buf = vec![0xd1u8]; // ROW token
    for &s in strides {
        if s == 0 {
            continue;
        }
        if s & 0x80 == 0 {
            buf.extend(std::iter::repeat_n(0xaau8, s as usize));
        } else {
            let prefix = (s & 0x7f) as usize;
            match prefix {
                1 => buf.push(CELL_DATA_LEN as u8),
                2 => buf.extend_from_slice(&(CELL_DATA_LEN as u16).to_le_bytes()),
                _ => buf.extend_from_slice(&(CELL_DATA_LEN as u32).to_le_bytes()),
            }
            buf.extend(std::iter::repeat_n(0xbbu8, CELL_DATA_LEN));
        }
    }
    buf
}

fn bench_row_walk(c: &mut Criterion) {
    let mut g = c.benchmark_group("row_walk");

    for &n in &[4usize, 16, 64] {
        let strides = vec![4u8; n];
        let buf = build_row(&strides);
        g.throughput(Throughput::Bytes(buf.len() as u64));
        g.bench_with_input(BenchmarkId::new("fixed_i4", n), &n, |b, _| {
            b.iter(|| Row::steps(black_box(&buf), black_box(&strides)));
        });
    }

    for &n in &[4usize, 16, 64] {
        let strides = vec![0x82u8; n]; // NVarChar USHORTLEN
        let buf = build_row(&strides);
        g.throughput(Throughput::Bytes(buf.len() as u64));
        g.bench_with_input(BenchmarkId::new("nvarchar_us", n), &n, |b, _| {
            b.iter(|| Row::steps(black_box(&buf), black_box(&strides)));
        });
    }

    for &reps in &[1usize, 4, 16] {
        let pattern = [4u8, 8, 0x82, 0x84]; // Int4, Int8, NVarChar, Image
        let strides: Vec<u8> = pattern
            .iter()
            .copied()
            .cycle()
            .take(reps * pattern.len())
            .collect();
        let buf = build_row(&strides);
        g.throughput(Throughput::Bytes(buf.len() as u64));
        g.bench_with_input(
            BenchmarkId::new("mixed", strides.len()),
            &strides.len(),
            |b, _| {
                b.iter(|| Row::steps(black_box(&buf), black_box(&strides)));
            },
        );
    }

    g.finish();
}

criterion_group!(benches, bench_row_walk);
criterion_main!(benches);
