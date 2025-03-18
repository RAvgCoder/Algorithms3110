use std::ops::RangeInclusive;

type Int = u32;

type Intervals = RangeInclusive<Int>;

enum ContainmentType {
    ContainsOther,  // `self_` fully contains `other`
    ContainsSelf,   // `other` fully contains `self_`
    PartialOverlap, // `self_` overlaps but does not fully contain `other`
}

pub struct Assignment5;

impl Assignment5 {
    #[allow(dead_code)]
    pub fn find_optimal_ranges_for_numbers<'i>(
        intervals: &'i [Intervals],
        numbers: &[Int],
    ) -> Vec<&'i Intervals> {
        // Sort by start
        let intervals = {
            let mut intervals = intervals.iter().collect::<Vec<_>>();
            intervals.sort_unstable_by_key(|r| *r.start());
            intervals
        };

        let numbers = {
            let mut numbers = numbers.to_vec();
            numbers.sort_unstable();
            numbers
        };

        let overlap = Self::filter_non_enclosed_ranges(&intervals);

        Self::filter_ranges_in_numbers(&overlap, &numbers)
    }

    fn check_overlap_status(self_: &Intervals, other: &Intervals) -> ContainmentType {
        assert!(
            other.start() >= self_.start(),
            "Other range has to start before self_"
        );

        if self_.start() <= other.start() && self_.end() >= other.end() {
            ContainmentType::ContainsOther
        } else if self_.start() == other.start() && self_.end() < other.end() {
            ContainmentType::ContainsSelf
        } else if self_.start() < other.start() && self_.end() < other.end() {
            ContainmentType::PartialOverlap
        } else {
            panic!(
                "Invalid range containment \nSelf: {:?}\nOther: {:?}",
                self_, other
            );
        }
    }

    fn filter_non_enclosed_ranges<'i>(intervals: &[&'i Intervals]) -> Vec<&'i Intervals> {
        let mut stich_stack = vec![];

        for &interval in intervals {
            // SAFETY: We have ensured that the vector is large enough to fit at maximum intervals.len()
            if stich_stack.is_empty() {
                stich_stack.push(interval);
            } else {
                let last = stich_stack.last_mut().unwrap();

                match Self::check_overlap_status(last, interval) {
                    ContainmentType::ContainsSelf => {
                        *last = interval;
                    }
                    ContainmentType::PartialOverlap => {
                        stich_stack.push(interval);
                    }
                    ContainmentType::ContainsOther => {
                        // Do nothing
                    }
                }
            }
        }

        stich_stack
    }

    pub fn find_farthest_interval<'l, 'i>(
        intervals: &'l [&'i Intervals],
        n: Int,
    ) -> (&'i Intervals, &'l [&'i Intervals]) {
        for (idx, interval) in intervals.iter().enumerate() {
            if !interval.contains(&n) && *interval.start() > n {
                return (intervals[idx - 1], &intervals[idx..]);
            }
        }

        let end_idx = intervals.len() - 1;
        (intervals[end_idx], &intervals[end_idx + 1..])
    }

    fn filter_ranges_in_numbers<'i>(
        mut interval_list: &[&'i Intervals],
        nums: &[Int],
    ) -> Vec<&'i Intervals> {
        let mut candidates = vec![];

        let mut curr_candidate: Option<&Intervals> = None;

        for n in nums {
            if let Some(candidate) = &mut curr_candidate {
                if candidate.contains(n) {
                    // If the current candidate contains the number
                    continue;
                }
            }

            let (farthest_range, interval) = Self::find_farthest_interval(interval_list, *n);
            interval_list = interval;

            candidates.push(farthest_range);

            curr_candidate = Some(farthest_range);
        }

        candidates
    }
}

#[cfg(test)]
mod test {
    macro_rules! compare_slices {
        ($expected:expr, $result:expr) => {{
            let is_eq = $expected.len() == $result.len()
                && $expected.iter().zip($result.iter()).all(|(a, b)| a == *b);

            if !is_eq {
                panic!(
                    "Slices are not equal\nExpected: {:?}\nGot: {:?}",
                    $expected, $result
                );
            }
        }};
    }
    #[test]
    fn test_lilo_and_stich1() {
        use crate::a5::Assignment5;
        let intervals = [
            5..=6,
            4..=5,
            2..=2,
            1..=1,
            1..=2,
            6..=6,
            2..=5,
            3..=4,
            3..=6,
        ];

        let numbers = (1..=6).collect::<Vec<_>>();
        let expected = [1..=2, 3..=6];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }

    #[test]
    fn test_lilo_and_stich2() {
        use crate::a5::Assignment5;
        let intervals = [
            1..=1,
            3..=6,
            7..=20,
            25..=50,
            50..=100,
            50..=80,
            20..=50,
            16..=70,
            80..=100,
            99..=100,
        ];

        let numbers = [1, 3, 35, 41, 55, 99, 100];
        let expected = [1..=1, 3..=6, 16..=70, 50..=100];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }

    #[test]
    fn test_lilo_and_stich3() {
        use crate::a5::Assignment5;
        let intervals = [0..=7, 6..=10, 20..=50];

        let numbers = [7, 20, 49];
        let expected = [6..=10, 20..=50];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }

    #[test]
    fn test_lilo_and_stich4() {
        use crate::a5::Assignment5;
        let intervals = [2..=8, 9..=11, 10..=12];

        let numbers = [7, 8, 9, 10, 11];
        let expected = [2..=8, 9..=11];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }

    #[test]
    fn test_lilo_and_stich5() {
        use crate::a5::Assignment5;
        let intervals = [1..=3, 2..=6, 4..=9];

        let numbers = [2, 5, 7];
        let expected = [2..=6, 4..=9];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }

    #[test]
    fn test_lilo_and_stich6() {
        use crate::a5::Assignment5;
        let intervals = [1..=4, 2..=5, 3..=6, 4..=7, 5..=8, 6..=9, 7..=10];

        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = [1..=4, 5..=8, 7..=10];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_lilo_and_stich7() {
        use crate::a5::Assignment5;
        let intervals = [1..=4, 2..=5, 3..=6, 4..=7, 5..=8, 6..=9, 7..=10];

        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let expected = [1..=4, 5..=8, 7..=10];

        let result = Assignment5::find_optimal_ranges_for_numbers(&intervals, &numbers);
        compare_slices!(expected, result);
    }
}
