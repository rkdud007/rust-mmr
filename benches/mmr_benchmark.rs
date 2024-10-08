use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mmr::{core::CoreMMR, hash::stark_poseidon::StarkPoseidonHasher, store::sqlite::SQLiteStore};

fn prepare_mmr(count: usize) -> CoreMMR<SQLiteStore, StarkPoseidonHasher> {
    let store = SQLiteStore::new(":memory:").unwrap();
    let hasher = StarkPoseidonHasher::new(Some(false));
    let _ = store.init();

    let mut mmr = CoreMMR::new(store, hasher.clone(), None);

    for i in 0..count {
        let _ = mmr.append(i.to_string()).unwrap();
    }

    mmr
}

fn bench(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("MMR insertion");
        let inputs = [10_000, 100_000, 100_0000];
        for input in inputs.iter() {
            group.bench_with_input(BenchmarkId::new("times", input), &input, |b, &&size| {
                b.iter(|| prepare_mmr(size));
            });
        }
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench
);
criterion_main!(benches);
