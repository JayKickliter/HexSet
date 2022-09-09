use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use geo_types::coord;
use h3_lorawan_regions::{
    compact::US915 as COMPACT_US915_INDICES, nocompact::US915 as PLAIN_US915_INDICES,
};
use hexmap::{compaction::EqCompactor, h3ron::H3Cell, HexMap, HexSet};
use std::convert::TryFrom;

fn hexset_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("US915 HexSet lookup");

    let us915_hexset: HexSet = PLAIN_US915_INDICES
        .iter()
        .map(|&idx| H3Cell::try_from(idx).unwrap())
        .collect();

    let tarpon_springs = coord! {x: -82.753822, y: 28.15215};
    let gulf_of_mexico = coord! {x: -83.101920, y: 28.128096};
    let paris = coord! {x: 2.340340, y: 48.868680};

    for resolution in [0, 4, 8, 12, 15] {
        let tarpon_springs = H3Cell::from_coordinate(tarpon_springs, resolution).unwrap();
        let gulf_of_mexico = H3Cell::from_coordinate(gulf_of_mexico, resolution).unwrap();
        let paris = H3Cell::from_coordinate(paris, resolution).unwrap();

        group.bench_with_input(
            BenchmarkId::new("Tarpon Spring", resolution),
            &tarpon_springs,
            |b, &cell| b.iter(|| us915_hexset.contains(&cell)),
        );

        group.bench_with_input(
            BenchmarkId::new("Gulf of Mexico", resolution),
            &gulf_of_mexico,
            |b, &cell| b.iter(|| us915_hexset.contains(&cell)),
        );

        group.bench_with_input(BenchmarkId::new("Paris", resolution), &paris, |b, &cell| {
            b.iter(|| us915_hexset.contains(&cell))
        });
    }
}

fn hexset_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("US915 HexSet construction");

    let precompacted_us915_cells: Vec<H3Cell> = COMPACT_US915_INDICES
        .iter()
        .map(|&idx| H3Cell::try_from(idx).unwrap())
        .collect();
    let plain_us915_cells: Vec<H3Cell> = PLAIN_US915_INDICES
        .iter()
        .map(|&idx| H3Cell::try_from(idx).unwrap())
        .collect();

    group.bench_function("pre-compacted", |b| {
        b.iter(|| (&precompacted_us915_cells).iter().collect::<HexSet>())
    });

    group.bench_function("plain", |b| {
        b.iter(|| (&plain_us915_cells).iter().collect::<HexSet>())
    });
}

fn hexmap_lookup(c: &mut Criterion) {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[allow(dead_code)]
    enum Region {
        EU868,
        US915,
    }

    let mut group = c.benchmark_group("US915 HexMap lookup");

    let us915_hexmap: HexMap<Region> = PLAIN_US915_INDICES
        .iter()
        .map(|&idx| H3Cell::try_from(idx).unwrap())
        .zip(std::iter::repeat(Region::US915))
        .collect();

    let tarpon_springs = coord! {x: -82.753822, y: 28.15215};
    let gulf_of_mexico = coord! {x: -83.101920, y: 28.128096};
    let paris = coord! {x: 2.340340, y: 48.868680};

    for resolution in [0, 4, 8, 12, 15] {
        let tarpon_springs = H3Cell::from_coordinate(tarpon_springs, resolution).unwrap();
        let gulf_of_mexico = H3Cell::from_coordinate(gulf_of_mexico, resolution).unwrap();
        let paris = H3Cell::from_coordinate(paris, resolution).unwrap();

        group.bench_with_input(
            BenchmarkId::new("Tarpon Spring", resolution),
            &tarpon_springs,
            |b, &cell| b.iter(|| us915_hexmap.get(&cell)),
        );

        group.bench_with_input(
            BenchmarkId::new("Gulf of Mexico", resolution),
            &gulf_of_mexico,
            |b, &cell| b.iter(|| us915_hexmap.get(&cell)),
        );

        group.bench_with_input(BenchmarkId::new("Paris", resolution), &paris, |b, &cell| {
            b.iter(|| us915_hexmap.get(&cell))
        });
    }
}

fn hexmap_construction(c: &mut Criterion) {
    // The value type for the map
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[allow(dead_code)]
    enum Region {
        EU868,
        US915,
    }

    let mut group = c.benchmark_group("US915 HexMap construction");

    let precompacted_us915_cells: Vec<H3Cell> = COMPACT_US915_INDICES
        .iter()
        .map(|&idx| H3Cell::try_from(idx).unwrap())
        .collect();
    let plain_us915_cells: Vec<H3Cell> = PLAIN_US915_INDICES
        .iter()
        .map(|&idx| H3Cell::try_from(idx).unwrap())
        .collect();

    group.bench_function("pre-compacted", |b| {
        b.iter(|| {
            (&precompacted_us915_cells)
                .iter()
                .zip(std::iter::repeat(&black_box(Region::US915)))
                .collect::<HexMap<Region>>()
        })
    });

    group.bench_function("plain", |b| {
        b.iter(|| {
            (&plain_us915_cells)
                .iter()
                .zip(std::iter::repeat(&black_box(Region::US915)))
                .zip(std::iter::repeat(&EqCompactor))
                .map(|((h, v), c)| (h, v, c))
                .collect::<HexMap<Region>>()
        })
    });
}

criterion_group!(
    benches,
    hexset_lookup,
    hexset_construction,
    hexmap_lookup,
    hexmap_construction
);
criterion_main!(benches);
