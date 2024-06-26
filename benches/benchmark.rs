use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_main(c: &mut Criterion) {
    {
        let mut g = c.benchmark_group("New");
        g.bench_function("StackAny New", |b| {
            b.iter(|| {
                let stack = stack_any::stack_any!(i32, 10);
                black_box(stack);
            })
        });
        g.bench_function("Box New", |b| {
            b.iter(|| {
                let heap: Box<dyn std::any::Any> = Box::new(10);
                black_box(heap);
            })
        });
    }

    {
        let mut g = c.benchmark_group("Mut");
        g.bench_function("StackAny Mut", |b| {
            b.iter(|| {
                let mut stack = stack_any::stack_any!(i32, 10);
                *stack.downcast_mut::<i32>() = 13;
                black_box(stack);
            })
        });
        g.bench_function("Box Mut", |b| {
            b.iter(|| {
                let mut heap: Box<dyn std::any::Any> = Box::new(10);
                *heap.downcast_mut::<i32>().unwrap() = 13;
                black_box(heap);
            })
        });
    }

    {
        let mut g = c.benchmark_group("Get");
        let stack = stack_any::stack_any!(i32, 10);
        g.bench_function("StackAny Get", |b| {
            b.iter(|| {
                let v = *stack.downcast_ref::<i32>();
                black_box(v);
            })
        });
        let heap: Box<dyn std::any::Any> = Box::new(10);
        g.bench_function("Box Get", |b| {
            b.iter(|| {
                let v = *heap.downcast_ref::<i32>().unwrap();
                black_box(v);
            })
        });
    }
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
