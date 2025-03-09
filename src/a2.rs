type IndexPair = (usize, usize);

#[allow(dead_code)]
fn n_log_n(arr: &[i32]) -> IndexPair {
    let mut prefix: Vec<(i32, isize)> = vec![(0, -1); arr.len() + 1]; // one extra element for the empty prefix

    // O(n)
    for (i, e) in arr.iter().enumerate() {
        prefix[i + 1] = (prefix[i].0 + e, i as isize);
    }

    let mut min_sub_range = (0, 0);
    let mut result_diff = i32::MAX;

    prefix.sort_unstable(); // O(n * log(n))

    // O(n)
    prefix.windows(2).into_iter().for_each(|window| {
        if let [(a, ai), (b, bi)] = window {
            let mut ai = *ai;
            let mut bi = *bi;

            let diff = a.abs_diff(*b);
            let min_diff = result_diff.min(diff as i32);

            if ai > bi {
                std::mem::swap(&mut ai, &mut bi);
            }

            ai += 1;

            if result_diff != min_diff {
                result_diff = min_diff;
                min_sub_range = (ai, bi);
            } else if result_diff == diff as i32 {
                // Update the min sub range if the current sub range is smaller
                if (bi as i32 - (ai as i32)) < (min_sub_range.1 as i32 - min_sub_range.0 as i32) {
                    min_sub_range = (ai, bi);
                }
            }
        } else {
            unreachable!("window size is not 2")
        }
    });

    (min_sub_range.0 as usize, min_sub_range.1 as usize)
}

#[allow(dead_code)]
fn n_square(arr: &[i32]) -> IndexPair {
    let mut min_sub_range = (usize::MAX, usize::MAX);
    let mut min_sum = u32::MAX;
    for i in 0..arr.len() {
        let mut sum = 0;
        for (j, e1) in arr[i..].iter().enumerate() {
            let j = i + j;
            sum += e1;
            let min = min_sum.min(sum.abs() as u32);
            if min_sum != min {
                min_sum = min;
                min_sub_range = (i, j);
            } else if min_sum == sum.abs() as u32 {
                // Update the min sub range if the current sub range is smaller
                if (j as i32 - i as i32).abs()
                    < (min_sub_range.1 as i32 - min_sub_range.0 as i32).abs()
                {
                    min_sub_range = (i, j);
                }
            }
        }
    }

    min_sub_range // O(1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarray_sum_algorithms() {
        let known = vec![
            vec![64, -47, -94, -13, 15, 16, -33],
            vec![-1, -40, -66, -74, 1, 29, -60],
            vec![-85, -28, 73, 3, 34],
            vec![-63, -22, 81, -85, -74, 28],
            vec![32, 32, 1, -5, -30, -30, -65, 130],
            vec![59, -12, -43, -45, -15, -92, -21],
            vec![23, -96, -100, 58, 16, 36, -100],
            vec![1, 2, 3, 4, 5],
            vec![28, -70, -23, 92, -2, 56, -33, -77],
            vec![5, 4, 0, 2, 4, 5],
            vec![5, 7, -2, 0, 10],
            vec![1, 2, 0, 5],
            vec![-8, -40, -10, -8, 55],
            vec![28, -70, -23, 92, 56, -33, -77],
        ];

        test(known.into_iter());
    }

    fn test(list: impl Iterator<Item = Vec<i32>>) {
        for arr in list {
            let res1 = n_square(&arr);
            let res2 = n_log_n(&arr);

            let sq = (
                &arr[res1.0..=res1.1],
                &arr[res1.0..=res1.1].iter().sum::<i32>(),
            );
            let nln = (
                &arr[res2.0..=res2.1],
                &arr[res2.0..=res2.1].iter().sum::<i32>(),
            );
            if sq.1.abs() == nln.1.abs() && sq.0.len() == nln.0.len() {
            } else {
                println!("Arr: {:?}\nResults are different", arr);
                assert_eq!(res1, res2, "{}", {
                    println!("arr: {:?}", arr);
                    format!("Results do not match sq: {:?} nln: {:?}", sq, nln)
                });
            }
        }
    }

    #[test]
    fn random_test_sort() {
        fn gen() -> Vec<i32> {
            use rand::Rng;
            let mut rng = rand::rng();
            let size = rng.random_range(5..=7);
            (0..size).map(|_| rng.random_range(-100..=100)).collect()
        }

        test((0..u16::MAX).map(|_| gen()));
    }
}
