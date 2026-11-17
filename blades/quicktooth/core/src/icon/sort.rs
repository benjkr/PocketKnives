//! .NET's `List<T>.Sort(Comparison<T>)` and `Array.BinarySearch<double>`, step for step.
//!
//! The rasterizer's crossings are sorted by x, and .NET's introsort is unstable: the order it
//! leaves equal crossings in decides where a span splits, which changes the coverage sums in the
//! last bits. So the same algorithm runs here (ArraySortHelper<T> in dotnet/runtime 10.0).
use core::cmp::Ordering;

/// `double.CompareTo` (no NaNs reach the renderer, but they order as .NET's do: lowest).
pub fn compare_f64(a: f64, b: f64) -> i32 {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) => -1,
        Some(Ordering::Greater) => 1,
        Some(Ordering::Equal) => 0,
        None if a.is_nan() => {
            if b.is_nan() {
                0
            } else {
                -1
            }
        }
        None => 1,
    }
}

const INTROSORT_SIZE_THRESHOLD: usize = 16;

/// `ArraySortHelper<T>.IntrospectiveSort(keys, comparer)`.
pub fn introsort<T: Copy>(keys: &mut [T], cmp: &impl Fn(&T, &T) -> i32) {
    if keys.len() > 1 {
        let log2 = (usize::BITS - 1 - keys.len().leading_zeros()) as i32;
        intro_sort(keys, 2 * (log2 + 1), cmp);
    }
}

fn intro_sort<T: Copy>(keys: &mut [T], mut depth_limit: i32, cmp: &impl Fn(&T, &T) -> i32) {
    let mut partition_size = keys.len();
    while partition_size > 1 {
        if partition_size <= INTROSORT_SIZE_THRESHOLD {
            if partition_size == 2 {
                swap_if_greater(keys, cmp, 0, 1);
                return;
            }
            if partition_size == 3 {
                swap_if_greater(keys, cmp, 0, 1);
                swap_if_greater(keys, cmp, 0, 2);
                swap_if_greater(keys, cmp, 1, 2);
                return;
            }
            insertion_sort(&mut keys[..partition_size], cmp);
            return;
        }
        if depth_limit == 0 {
            heap_sort(&mut keys[..partition_size], cmp);
            return;
        }
        depth_limit -= 1;
        let p = pick_pivot_and_partition(&mut keys[..partition_size], cmp);
        intro_sort(&mut keys[p + 1..partition_size], depth_limit, cmp);
        partition_size = p;
    }
}

fn swap_if_greater<T: Copy>(keys: &mut [T], cmp: &impl Fn(&T, &T) -> i32, i: usize, j: usize) {
    if cmp(&keys[i], &keys[j]) > 0 {
        keys.swap(i, j);
    }
}

fn pick_pivot_and_partition<T: Copy>(keys: &mut [T], cmp: &impl Fn(&T, &T) -> i32) -> usize {
    let hi = keys.len() - 1;
    let middle = hi >> 1;
    swap_if_greater(keys, cmp, 0, middle);
    swap_if_greater(keys, cmp, 0, hi);
    swap_if_greater(keys, cmp, middle, hi);
    let pivot = keys[middle];
    keys.swap(middle, hi - 1);
    let (mut left, mut right) = (0usize, hi - 1);
    while left < right {
        loop {
            left += 1;
            if cmp(&keys[left], &pivot) >= 0 {
                break;
            }
        }
        loop {
            right -= 1;
            if cmp(&pivot, &keys[right]) >= 0 {
                break;
            }
        }
        if left >= right {
            break;
        }
        keys.swap(left, right);
    }
    if left != hi - 1 {
        keys.swap(left, hi - 1);
    }
    left
}

fn heap_sort<T: Copy>(keys: &mut [T], cmp: &impl Fn(&T, &T) -> i32) {
    let n = keys.len();
    let mut i = n >> 1;
    while i >= 1 {
        down_heap(keys, i, n, cmp);
        i -= 1;
    }
    let mut i = n;
    while i > 1 {
        keys.swap(0, i - 1);
        down_heap(keys, 1, i - 1, cmp);
        i -= 1;
    }
}

fn down_heap<T: Copy>(keys: &mut [T], mut i: usize, n: usize, cmp: &impl Fn(&T, &T) -> i32) {
    let d = keys[i - 1];
    while i <= n >> 1 {
        let mut child = 2 * i;
        if child < n && cmp(&keys[child - 1], &keys[child]) < 0 {
            child += 1;
        }
        if cmp(&d, &keys[child - 1]) >= 0 {
            break;
        }
        keys[i - 1] = keys[child - 1];
        i = child;
    }
    keys[i - 1] = d;
}

fn insertion_sort<T: Copy>(keys: &mut [T], cmp: &impl Fn(&T, &T) -> i32) {
    for i in 0..keys.len() - 1 {
        let t = keys[i + 1];
        let mut j = i as isize;
        while j >= 0 && cmp(&t, &keys[j as usize]) < 0 {
            keys[j as usize + 1] = keys[j as usize];
            j -= 1;
        }
        keys[(j + 1) as usize] = t;
    }
}

/// `Array.BinarySearch(double[], double)`: the index of a match (the first midpoint that hits,
/// not necessarily the leftmost), else the bitwise complement of the insertion point.
pub fn binary_search(array: &[f64], value: f64) -> i32 {
    let mut lo: i32 = 0;
    let mut hi: i32 = array.len() as i32 - 1;
    while lo <= hi {
        let i = lo + ((hi - lo) >> 1);
        let order = compare_f64(array[i as usize], value);
        if order == 0 {
            return i;
        }
        if order < 0 {
            lo = i + 1;
        } else {
            hi = i - 1;
        }
    }
    !lo
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn sorts_like_any_sort() {
        // A simple LCG, so the test needs no dependencies.
        let mut seed = 12345u64;
        for len in [0usize, 1, 2, 3, 5, 16, 17, 40, 200, 1000] {
            let mut v: Vec<(f64, i32)> = (0..len)
                .map(|k| {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    (((seed >> 33) % 50) as f64, k as i32)
                })
                .collect();
            introsort(&mut v, &|p: &(f64, i32), q: &(f64, i32)| compare_f64(p.0, q.0));
            assert!(v.windows(2).all(|w| w[0].0 <= w[1].0), "len {len}");
        }
    }

    #[test]
    fn binary_search_matches_dotnet() {
        let a = [0.0, 1.0, 2.0, 2.0, 2.0, 5.0];
        assert_eq!(binary_search(&a, 2.0), 2);
        assert_eq!(binary_search(&a, 3.0), !5);
        assert_eq!(binary_search(&a, -1.0), !0);
        assert_eq!(binary_search(&a, 9.0), !6);
    }
}
