use minesweeper_x::*;

const ARR_SIZE: usize = 3;
const C: usize = 2;

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_a(arr: &mut [bool; ARR_SIZE]) {
        let mut rng = rand::thread_rng();
        let mut c = C;
        for i in 0..arr.len() {
            if rng.gen_range(0..(arr.len() - i)) < c {
                arr[i] = true;
                c -= 1;
            }
        }
    }

    fn rand_b(arr: &mut [bool; ARR_SIZE]) {
        let mut rng = rand::thread_rng();
        let mut c = C;
        while c > 0 {
            let mut i = rng.gen_range(0..arr.len());
            while arr[i] {
                i = rng.gen_range(0..arr.len());
            }
            arr[i] = true;
            c -= 1;
        }
    }

    fn rand_c(arr: &mut [bool; ARR_SIZE]) {
        let mut rng = rand::thread_rng();
        let mut c = C;
        while c > 0 {
            let mut i = rng.gen_range(0..arr.len());
            while arr[i] {
                i = (i + 1) % arr.len();
            }
            arr[i] = true;
            c -= 1;
        }
    }

    fn sum_arr(arr: &[bool; ARR_SIZE]) -> usize {
        let mut sum = 0;
        for i in 0..arr.len() {
            if arr[i] {
                sum += 1 << i;
            }
        }
        sum
    }

    fn test_rand_alg(f: fn(&mut [bool; ARR_SIZE])) {
        let mut result = [0; 1 << ARR_SIZE];
        for _ in 0..1000000 {
            let mut arr = [false; ARR_SIZE];
            f(&mut arr);
            result[sum_arr(&arr)] += 1;
        }
        let mut len = 0;
        for i in 0..result.len() {
            if result[i] > 0 {
                len += 1;
            }
        }
        for i in 0..result.len() {
            if result[i] > 0 {
                println!(
                    "Possibility {:.5}%",
                    result[i] as f64 / 1000000.0 / len as f64 * 100.0
                );
            }
        }
    }

    #[test]
    fn test_rand_a() {
        test_rand_alg(rand_a);
    }

    #[test]
    fn test_rand_b() {
        test_rand_alg(rand_b);
    }

    #[test]
    fn test_rand_c() {
        test_rand_alg(rand_c);
    }
}
