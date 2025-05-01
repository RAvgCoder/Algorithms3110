use crate::a4::register_bank::{Pos, Register, RegisterBank};

type Int = i32;
mod register_bank {
    use crate::a4::Int;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone)]
    pub struct Pos(pub usize, pub usize); // (row, col)

    #[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone)]
    pub struct Register(pub Int, pub Pos);

    pub struct RegisterBank {
        bank: BinaryHeap<Reverse<Register>>,
        k: usize,
    }

    pub struct BankHandle<'r_bank> {
        register_bank: &'r_bank mut RegisterBank,
        min: Register,
    }

    impl RegisterBank {
        /// O(n) => registers
        #[allow(dead_code)]
        pub fn new(capacity: usize) -> RegisterBank {
            RegisterBank {
                bank: BinaryHeap::with_capacity(capacity),
                k: capacity,
            }
        }

        /// O(1) assumed
        pub fn remove_min(&mut self) -> Option<BankHandle> {
            self.bank.pop().map(|x| BankHandle {
                register_bank: self,
                min: x.0,
            })
        }

        pub fn re_innit(&mut self, registers: Vec<Register>) {
            self.bank.clear();
            assert!(
                registers.len() <= self.k,
                "RegisterBank has capacity: {}, but was given registers with size: {}",
                self.k,
                registers.len()
            );
            assert!(self.bank.capacity() >= registers.len());
            self.bank.extend(registers.into_iter().map(Reverse))
        }
    }

    impl BankHandle<'_> {
        pub fn get(&self) -> Register {
            self.min
        }

        pub fn insert(self, register: Register) {
            self.register_bank.bank.push(Reverse(register));
            assert!(self.register_bank.bank.len() <= self.register_bank.k);
        }
    }
}

#[allow(dead_code)]
fn sort(numbers: &[Int], k: usize, register_bank: &mut RegisterBank) -> Vec<Int> {
    let n = numbers.len();

    // Base Case
    if n == 1 {
        return vec![numbers[0]];
    } else if n == 0 {
        return vec![];
    }

    let num_sub_arrays = n.div_ceil(k);

    let sub_array_list: Vec<Vec<Int>> = numbers
        .chunks(num_sub_arrays)
        .map(|arr| sort(arr, k, register_bank))
        .collect();

    let init_reg = sub_array_list
        .iter()
        .enumerate()
        .map(|(i, vec)| Register(vec[0], Pos(i, 0)))
        .collect();

    register_bank.re_innit(init_reg);

    let mut sorted_array = Vec::with_capacity(sub_array_list.len() * sub_array_list[0].len());

    while let Some(bank_handle) = register_bank.remove_min() {
        let Register(val, Pos(i, j)) = bank_handle.get();

        // Adds the number popped into the sorted array
        assert!(sorted_array.capacity() > sorted_array.len());
        sorted_array.push(val);

        // Add the next elem if available
        if let Some(next) = sub_array_list[i].get(j + 1) {
            bank_handle.insert(Register(*next, Pos(i, j + 1)))
        }
    }

    sorted_array
}

#[cfg(test)]
mod test {
    use crate::a4::register_bank::RegisterBank;
    use crate::a4::{sort, Int};
    use rand::Rng;

    #[test]
    fn random_test_sort() {
        let mut rng = rand::rng();

        for _ in 0..u8::MAX {
            let list_len = rng.random_range(2..=500);
            let k = rng.random_range(2..=70);

            let mut list = (0..list_len)
                .map(|_| rng.random_range(Int::MIN..=Int::MAX))
                .collect::<Vec<_>>();

            let mut register_bank = RegisterBank::new(k);
            let result = sort(&list, k, &mut register_bank);

            list.sort_unstable();

            assert_eq!(result, list);
        }
    }
}
