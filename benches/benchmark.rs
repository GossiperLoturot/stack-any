use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_main(c: &mut Criterion) {
    {
        let mut g = c.benchmark_group("New");
        g.bench_function("StackAny New", |b| {
            b.iter(|| {
                let stack = stack_any::stack_any!(u8, 127u8);
                black_box(stack);
            })
        });
        g.bench_function("Box New", |b| {
            b.iter(|| {
                let heap: Box<dyn std::any::Any> = Box::new(127u8);
                black_box(heap);
            })
        });
    }

    {
        let mut g = c.benchmark_group("Mut");
        g.bench_function("StackAny Mut", |b| {
            b.iter(|| {
                let mut stack = stack_any::stack_any!(u8, 127u8);
                *stack.downcast_mut().unwrap() = 63u8;
                black_box(stack);
            })
        });
        g.bench_function("Box Mut", |b| {
            b.iter(|| {
                let mut heap: Box<dyn std::any::Any> = Box::new(127u8);
                *heap.downcast_mut().unwrap() = 63u8;
                black_box(heap);
            })
        });
    }

    {
        let mut g = c.benchmark_group("Get");
        let stack = stack_any::stack_any!(u8, 127u8);
        g.bench_function("StackAny Get", |b| {
            b.iter(|| {
                let v = stack.downcast_ref::<u8>().unwrap();
                black_box(v);
            })
        });
        let heap: Box<dyn std::any::Any> = Box::new(127u8);
        g.bench_function("Box Get", |b| {
            b.iter(|| {
                let v = heap.downcast_ref::<u8>().unwrap();
                black_box(v);
            })
        });
    }
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
