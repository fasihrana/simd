use simd::*;
use bencher::*;

const len: usize = 10000000;

fn normal_add_bench(b: &mut Bencher) {
    let mut arr = vec![1;len];
    b.iter(move ||{

        normal_add(&mut arr, 1); 
    });    
}

fn simd_add_bench(b:&mut Bencher) {
    unsafe {
        let mut arr = vec![1; len];
        let (mut prefix, mut arr, mut suffix) = arr.align_to_mut::<i32>();

        b.iter(move || {
            normal_add(prefix, 1);
            simd_add(&mut arr, 1);
            normal_add(suffix, 1);
        });
    }
}

benchmark_group!(benches,normal_add_bench, simd_add_bench);
benchmark_main!(benches);
