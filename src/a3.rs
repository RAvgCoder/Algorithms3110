#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hotel {
    distance: u32,
    price: u32,
}

struct DominatorStack(Vec<Hotel>);

impl DominatorStack {
    fn new(cap: usize) -> Result<Self, ()> {
        if cap == 0 {
            Err(())
        } else {
            Ok(Self(Vec::with_capacity(cap)))
        }
    }

    /// Safety Assumptions
    /// For any hotel Hi, the distance would always be larger than any hotel Hj passed previously
    /// where i > j and the dominator stack has enough capacity to hold all the hotels
    ///
    /// The invariance maintained by the dominator stack which includes:
    /// 1. The hotels are sorted by distance (Asc)
    /// 2. The hotels are sorted by price (Desc)
    /// 3. If two hotels have the same distance, the one with the lower price is kept
    fn push(&mut self, hotel: Hotel) -> Result<(), ()> {
        if self.0.capacity() > self.0.len() {
            if let Some(prev) = self.0.last_mut() {
                // Triggers only if there is at least one element
                if prev.distance != hotel.distance {
                    // Not the same distance
                    if prev.price > hotel.price {
                        // Price is lower so we can add this hotel
                        self.0.push(hotel);
                    }
                    // else: Price is higher Do nothing
                } else if prev.price > hotel.price {
                    // Same distance but the price is lower
                    // replace the previous hotel
                    *prev = hotel;
                }
            } else {
                // No elements in the vector
                self.0.push(hotel);
            }
            Ok(())
        } else {
            Err(())
        }
    }

    fn into_inner(self) -> Vec<Hotel> {
        self.0
    }
}

#[allow(dead_code)]
fn find_candidate_hotels(hotels: &[Hotel]) -> Vec<Hotel> {
    if hotels.len() == 0 {
        vec![]
    } else if hotels.len() == 1 {
        vec![hotels[0]]
    } else {
        let mid = hotels.len() / 2;
        let (left, right) = hotels.split_at(mid);

        let left = find_candidate_hotels(left);
        let right = find_candidate_hotels(right);

        let mut dominator_stack =
            DominatorStack::new(hotels.len()).expect("Hotel capacity is should be greater than 0");

        // Sort by distance
        let mut left_iter = left.into_iter().peekable();
        let mut right_iter = right.into_iter().peekable();

        while let (Some(hotel_l), Some(hotel_r)) = (left_iter.peek(), right_iter.peek()) {
            let l = hotel_l.distance;
            let r = hotel_r.distance;
            if l < r {
                dominator_stack.push(*hotel_l).unwrap();
                left_iter.next();
            } else {
                dominator_stack.push(*hotel_r).unwrap();
                right_iter.next();
            }
        }

        left_iter.for_each(|hotel| dominator_stack.push(hotel).unwrap());
        right_iter.for_each(|hotel| dominator_stack.push(hotel).unwrap());

        dominator_stack.into_inner()
    }
}

#[allow(dead_code)]
fn binary_exponentiation(base: i32, exp: i32) -> i32 {
    if exp == 1 {
        base
    } else if exp % 2 == 0 {
        let half = binary_exponentiation(base, exp / 2);
        half * half
    } else {
        base * binary_exponentiation(base, exp - 1)
    }
}

#[cfg(test)]
mod test {
    mod hotels {
        use crate::a3::{find_candidate_hotels, Hotel};

        #[test]
        fn test_find_candidate_hotels_fixed() {
            fn to_hotels(input: &[(u32, u32)]) -> Vec<Hotel> {
                input
                    .iter()
                    .map(|(distance, price)| Hotel {
                        distance: *distance,
                        price: *price,
                    })
                    .collect()
            }

            let input = [(20, 250), (20, 250)];
            let input_ans = [(20, 250)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));

            let input = [
                (20, 250),
                (40, 400),
                (10, 500),
                (30, 300),
                (50, 0),
                (35, 250),
            ];
            let input_ans = [(10, 500), (20, 250), (50, 0)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));

            let input = [(10, 50), (20, 40), (30, 30), (40, 0), (50, 10)];
            let input_ans = [(10, 50), (20, 40), (30, 30), (40, 0)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));

            let input = [(3, 20), (4, 1)];
            let input_ans = [(3, 20), (4, 1)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));

            let input = [(10, 50), (20, 40), (30, 30), (40, 20), (50, 10)];
            let input_ans = [(10, 50), (20, 40), (30, 30), (40, 20), (50, 10)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));

            let input = [(1, 100), (2, 150), (3, 200), (4, 250)];
            let input_ans = [(1, 100)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));

            let input = [(3, 300), (3, 250), (4, 250), (5, 200), (5, 150)];
            let input_ans = [(3, 250), (5, 150)];
            let output = find_candidate_hotels(&to_hotels(&input));
            assert_eq!(output, to_hotels(&input_ans));
        }
    }
    mod bin {
        use crate::a3::binary_exponentiation;

        #[test]
        fn test_binary_exponentiation_fixed() {
            assert_eq!(binary_exponentiation(2, 3), 8);
            assert_eq!(binary_exponentiation(3, 3), 27);
            assert_eq!(binary_exponentiation(2, 4), 16);
            assert_eq!(binary_exponentiation(3, 4), 81);
        }

        #[test]
        fn test_binary_exponentiation_random() {
            use rand::Rng;
            let mut rng = rand::rng();

            for _ in 0..u8::MAX {
                let base = rng.random_range(1..=5);
                let exp = rng.random_range(1..=12);
                assert_eq!(binary_exponentiation(base, exp), base.pow(exp as u32));
            }
        }
    }
}
