
pub fn normal_add(arr: &mut [i32], val: i32){
    for e in arr {
        *e += val;
    }
}

#[target_feature(enable = "avx,avx2")]
pub unsafe fn simd_add(arr:&mut [i32], val: i32) {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    let a = _mm256_set1_epi32(val);
    for chunk in arr.chunks_exact_mut(8) {

        let mut b =_mm256_set_epi32(
            *chunk.get_unchecked(7),
            *chunk.get_unchecked(6),
            *chunk.get_unchecked(5),
            *chunk.get_unchecked(4),
            *chunk.get_unchecked(3),
            *chunk.get_unchecked(2),
            *chunk.get_unchecked(1),
            *chunk.get_unchecked(0));
        b = _mm256_add_epi32(a, b);
        let b2 : *const i32 = &b as *const _ as *const _;
        std::ptr::copy_nonoverlapping(b2, chunk.get_unchecked_mut(0), 8);
        /*let b:(i32,i32,i32,i32,i32,i32,i32,i32) = std::mem::transmute(b);
        chunk[0] = b.0;
        chunk[1] = b.1;
        chunk[2] = b.2;
        chunk[3] = b.3;
        chunk[4] = b.4;
        chunk[5] = b.5;
        chunk[6] = b.6;
        chunk[7] = b.7;*/
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn normal_add_test() {
        let mut arr = vec![1,2,3,4,5,6,7,8];
        super::normal_add(&mut arr, 1);
        assert_eq!(arr[0], 2);
        assert_eq!(arr[1], 3);
        assert_eq!(arr[2], 4);
        assert_eq!(arr[3], 5);
        assert_eq!(arr[4], 6);
        assert_eq!(arr[5], 7);
        assert_eq!(arr[6], 8);
        assert_eq!(arr[7], 9);
    }

    #[test]
    fn simd_add_test() {
        let mut arr = vec![1,2,3,4,5,6,7,8];
        unsafe {super::simd_add(&mut arr, 1)};
        assert_eq!(arr[0], 2);
        assert_eq!(arr[1], 3);
        assert_eq!(arr[2], 4);
        assert_eq!(arr[3], 5);
        assert_eq!(arr[4], 6);
        assert_eq!(arr[5], 7);
        assert_eq!(arr[6], 8);
        assert_eq!(arr[7], 9);
    }
}
