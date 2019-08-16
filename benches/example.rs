use simd::*;
use bencher::*;

const len: usize = 10000000;

fn normal_add_bench(b: &mut Bencher) {
    b.iter(||{
        let mut arr = vec![1;len];
        normal_add(&mut arr, 1); 
    });    
}

fn simd_add_bench(b:&mut Bencher) {
    b.iter(||{
        let mut arr = vec![1;len];
        unsafe{simd_add(&mut arr, 1)};
    });    
}

benchmark_group!(benches,normal_add_bench, simd_add_bench);
benchmark_main!(benches);
