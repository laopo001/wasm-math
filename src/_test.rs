extern crate test;
// Benchmark test
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            let mut arr: Vec<i32> = vec![];
            for _ in 0..10000 {
                arr.push(add_two(2));
            }
            add_two(2)
        });
    }
}