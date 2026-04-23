//! NBC (Null-Bitmap Compression) ROW stride walk. Adds a null-bitmap scan over
//! `Row::steps`; nulls are skipped without advancing the cursor.
//!
//! Vary null density to see both the bitmap-read cost (high-null cases skip
//! most walks) and the mispredict cost (50% density is worst case).

use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use bronotdsaurs::tds::decoder::stream::{Drainable, NbcRow};

const COLS: usize = 64;
const CELL_DATA_LEN: usize = 16;

fn build_nbc_row(strides: &[u8], null_every: Option<usize>) -> Vec<u8> {
    let bitmap_len = strides.len().div_ceil(8);
    let mut buf = vec![0xd2u8]; // NBCROW token
    let mut bitmap = vec![0u8; bitmap_len];

    for i in 0..strides.len() {
        if let Some(k) = null_every
            && k > 0 && i % k == 0 {
                bitmap[i / 8] |= 1 << (i % 8);
            }
    }
    buf.extend_from_slice(&bitmap);

    for (i, &s) in strides.iter().enumerate() {
        let is_null = matches!(null_every, Some(k) if k > 0 && i % k == 0);
        if is_null || s == 0 {
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
        std::iter::repeat_n(0xbbu8, CELL_DATA_LEN);
    }
    buf
}

fn bench_nbc(c: &mut Criterion) {
    let mut g = c.benchmark_group("nbc_row_walk");
    let strides = vec![0x82u8; COLS]; // all NVarChar USHORTLEN

    // Approximate null densities via modulo. null_every=None -> 0%.
    for &(label, every) in &[
        ("nulls_0pct", None),
        ("nulls_25pct", Some(4usize)),
        ("nulls_50pct", Some(2usize)),
        ("nulls_99pct", Some(1usize)),
    ] {
        let buf = build_nbc_row(&strides, every);
        g.throughput(Throughput::Bytes(buf.len() as u64));
        g.bench_with_input(BenchmarkId::new(label, COLS), &buf, |b, buf| {
            b.iter(|| NbcRow::steps(black_box(buf.as_slice()), black_box(&strides)));
        });
    }

    g.finish();
}

criterion_group!(benches, bench_nbc);
criterion_main!(benches);
