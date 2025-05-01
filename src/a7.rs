use std::str::FromStr;

pub struct Assignment7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Delete(char),
    Insert(char),
    Replace { from: char, to: char },
    Copy(char),
}

type Coord = (usize, usize);

impl Operation {
    pub fn from_raw(raw_ops: &[impl AsRef<str>]) -> Box<[Operation]> {
        raw_ops
            .iter()
            .map(|x| Operation::from_str(x.as_ref()).unwrap())
            .collect()
    }
    #[inline(always)]
    pub fn count_ops(ops: &[Operation]) -> u32 {
        ops.iter().map(Operation::cost).sum()
    }

    pub const fn cost(&self) -> u32 {
        match self {
            Operation::Copy(_) => 0,
            Operation::Delete(_) | Operation::Insert(_) | Operation::Replace { .. } => 1,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op = parts.next().ok_or(())?;
        let c = parts.next().ok_or(())?.chars().next().ok_or(())?;

        Ok(match op {
            "Delete" => Operation::Delete(c),
            "Insert" => Operation::Insert(c),
            "Replace" => {
                let to = parts.next().ok_or(())?.chars().next().ok_or(())?;
                Operation::Replace { from: c, to }
            }
            "Copy" => Operation::Copy(c),
            _ => return Err(()),
        })
    }
}

type Cost = u32;

type Matrix<T> = Box<[Box<[T]>]>;

impl Assignment7 {
    /// Given two strings `x` and `y`, compute the metamorphosis number.
    /// By definition, the metamorphosis number is the minimum number of operations required to
    /// transform string `x` into string `y`.
    ///
    /// The operations allowed are:
    /// 1. Insert a character   Cost: 1
    /// 2. Delete a character   Cost: 1
    /// 3. Replace a character  Cost: 1
    /// 4. Copy a character     Cost: 0
    ///
    /// # Arguments
    /// * `x` - The source string
    /// * `y` - The target string
    ///
    /// # Returns
    /// The metamorphosis number
    ///
    /// # Example
    /// ```rust
    /// let x = "Dalhousie";
    /// let y = "Delicious";
    /// let result = Assignment7::compute_metamorphosis_number(x, y);
    /// assert_eq!(result, 6);
    ///
    /// // Explanation: x => y
    /// // copy the D
    /// // replace the e with a
    /// // copy the l
    /// // replace the i with h
    /// // delete the c and i
    /// // copy the o, u and s
    /// // insert i and e.
    /// ```
    #[allow(dead_code)]
    pub fn compute_metamorphosis_number(from: &str, to: &str) -> u32 {
        Self::metamorphosis_core(from, to)[0][0]
    }

    fn metamorphosis_core(from: &str, to: &str) -> Matrix<Cost> {
        let mut cache_grid: Matrix<Cost> = Self::init_grid(from, to);

        for (i, row) in cache_grid.iter_mut().rev().enumerate() {
            *row.last_mut().expect("The last column should exist") = i as u32;
        }

        for (j, cell) in cache_grid
            .last_mut()
            .expect("The last row should exist")
            .iter_mut()
            .rev()
            .enumerate()
        {
            *cell = j as u32;
        }

        for i in (0..to.len()).rev() {
            for j in (0..from.len()).rev() {
                // If chars are equal,
                if from.chars().nth(j) == to.chars().nth(i) {
                    cache_grid[i][j] = cache_grid[i + 1][j + 1];
                } else {
                    let new_min = [
                        cache_grid[i][j + 1],     // Delete
                        cache_grid[i + 1][j + 1], // Replace
                        cache_grid[i + 1][j],     // Insert
                    ]
                    .into_iter()
                    .min()
                    .expect("There should be a minimum");

                    cache_grid[i][j] = 1 + new_min;
                };
            }
        }

        cache_grid
    }

    fn init_grid<T: Default + Clone>(from: &str, to: &str) -> Matrix<T> {
        vec![vec![T::default(); from.len() + 1].into_boxed_slice(); to.len() + 1].into_boxed_slice()
    }

    #[allow(dead_code)]
    pub fn render_metamorphosis_steps(from: &str, to: &str) -> Box<[Operation]> {
        let mut path = Vec::with_capacity(from.len() + to.len());
        let cache_grid = Self::metamorphosis_core(from, to);

        let destination_coord: Coord = (to.len(), from.len());

        let mut curr: Coord = (0, 0);
        while curr != destination_coord {
            let curr_cost = cache_grid[curr.0][curr.1];

            const DELETE: Coord = (0, 1);
            const INSERT: Coord = (1, 0);
            const REPLACE_OR_COPY: Coord = (1, 1);

            fn find_min_ops(delete: Cost, insert: Cost, replace_or_copy: Cost) -> (Coord, Cost) {
                // Replace/Copy because if it is a copy that has the minimum cost
                // Then it should have the priority over delete and insert
                // Note this relies on the implementation of [`std::cmp::min`] that it prioritizes
                // the first element in case of a tie.
                let array = [
                    (REPLACE_OR_COPY, replace_or_copy),
                    (DELETE, delete),
                    (INSERT, insert),
                ];
                *array.iter().min_by_key(|(_, cost)| cost).unwrap()
            }

            fn get_cost(grid: &Matrix<Cost>, coord: Coord) -> Cost {
                if coord.0 >= grid.len() || coord.1 >= grid[0].len() {
                    return u32::MAX;
                }
                grid[coord.0][coord.1]
            }

            let (next_offset, min_cost) = {
                let (i, j) = curr;
                find_min_ops(
                    get_cost(&cache_grid, (i, j + 1)),
                    get_cost(&cache_grid, (i + 1, j)),
                    get_cost(&cache_grid, (i + 1, j + 1)),
                )
            };

            path.push(match next_offset {
                DELETE => Operation::Delete(from.chars().nth(curr.1).unwrap()),
                INSERT => Operation::Insert(to.chars().nth(curr.0).unwrap()),
                REPLACE_OR_COPY => {
                    if curr_cost == min_cost {
                        Operation::Copy(from.chars().nth(curr.1).unwrap())
                    } else {
                        Operation::Replace {
                            from: from.chars().nth(curr.1).unwrap(),
                            to: to.chars().nth(curr.0).unwrap(),
                        }
                    }
                }
                _ => unreachable!("Invalid offset"),
            });

            curr = (next_offset.0 + curr.0, next_offset.1 + curr.1);
        }

        path.into_boxed_slice()
    }

    #[allow(dead_code)]
    fn print_grid(grid: &Matrix<Cost>, from: &str, to: &str) {
        print!("  ");
        from.chars().for_each(|c| print!("{c} "));
        println!();
        let mut to = to.chars();
        for row in grid {
            if let Some(c) = to.next() {
                print!("{c} ");
            } else {
                print!("  ");
            }
            for e in row {
                print!("{} ", e)
            }
            println!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_compute_metamorphosis_number {
        use super::*;

        #[test]
        fn test1() {
            let from = "Dalhousie";
            let to = "Delicious";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 6);
        }

        #[test]
        fn test2() {
            let from = "abd";
            let to = "acd";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 1);
        }

        #[test]
        fn test3() {
            let from = "acd";
            let to = "xadd";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 2);
        }

        #[test]
        fn test4() {
            let from = "a";
            let to = "abc";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 2);
        }

        /// /////
        #[test]
        fn test5() {
            let from = "kitten";
            let to = "sitting";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 3); // Replace 'k' with 's', replace 'e' with 'i', insert 'g'
        }

        #[test]
        fn test6() {
            let from = "intention";
            let to = "execution";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 5); // Series of replacements and insertions
        }

        #[test]
        fn test7() {
            let from = "abcdef";
            let to = "azced";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 3); // Replace 'b' with 'z', replace 'f' with 'd', delete 'e'
        }

        #[test]
        fn test8() {
            let from = "";
            let to = "abc";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 3); // Insert 'a', 'b', 'c'
        }

        #[test]
        fn test9() {
            let from = "hello";
            let to = "hallo";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 1);
        }

        #[test]
        fn test10() {
            let from = "clbmkkvh";
            let to = "bmkvh";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 3);
        }

        #[test]
        fn test11() {
            let from = "ibikwjep";
            let to = "ukwep";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 4);
        }

        #[test]
        fn test12() {
            let from = "ybpqvati";
            let to = "ybpvoti";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 2);
        }

        #[test]
        fn test13() {
            let from = "jafkvgyh";
            let to = "aogyu";
            let result = Assignment7::compute_metamorphosis_number(from, to);
            assert_eq!(result, 5);
        }
    }

    mod test_render_metamorphosis_steps {
        use super::*;

        #[test]
        fn test1() {
            let to = "Dalhousie";
            let from = "Delicious";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Copy D",
                    "Replace e with a",
                    "Copy l",
                    "Delete i",
                    "Delete c",
                    "Replace i with h",
                    "Copy o",
                    "Copy u",
                    "Copy s",
                    "Insert i",
                    "Insert e",
                ]))
            );
        }

        #[test]
        fn test2() {
            let from = "abd";
            let to = "acd";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Copy a",
                    "Replace b with c",
                    "Copy d",
                ]))
            );
        }

        #[test]
        fn test3() {
            let to = "acd";
            let from = "xadd";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Delete x",
                    "Copy a",
                    "Replace d with c",
                    "Copy d"
                ]))
            );
        }

        #[test]
        fn test4() {
            let from = "a";
            let to = "abc";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&["Copy a", "Insert b", "Insert c"]))
            );
        }

        #[test]
        fn test5() {
            let from = "kitten";
            let to = "sitting";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Replace k with s",
                    "Copy i",
                    "Copy t",
                    "Copy t",
                    "Replace e with i",
                    "Copy n",
                    "Insert g",
                ]))
            );
        }

        #[test]
        fn test6() {
            let from = "intention";
            let to = "execution";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Replace i with e",
                    "Replace n with x",
                    "Replace t with e",
                    "Replace e with c",
                    "Replace n with u",
                    "Copy t",
                    "Copy i",
                    "Copy o",
                    "Copy n"
                ]))
            );
        }

        #[test]
        fn test7() {
            let from = "abcdef";
            let to = "azced";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Copy a",
                    "Replace b with z",
                    "Copy c",
                    "Delete d",
                    "Copy e",
                    "Replace f with d"
                ]))
            );
        }

        #[test]
        fn test8() {
            let from = "";
            let to = "abc";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&["Insert a", "Insert b", "Insert c"]))
            );
        }

        #[test]
        fn test9() {
            let from = "hello";
            let to = "hallo";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Copy h",
                    "Replace e with a",
                    "Copy l",
                    "Copy l",
                    "Copy o"
                ]))
            );
        }

        #[test]
        fn test10() {
            let from = "clbmkkvh";
            let to = "bmkvh";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Delete c", "Delete l", "Copy b", "Copy m", "Copy k", "Delete k", "Copy v",
                    "Copy h",
                ]))
            );
        }

        #[test]
        fn test11() {
            let from = "ibikwjep";
            let to = "ukwep";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Delete i",
                    "Delete b",
                    "Replace i with u",
                    "Copy k",
                    "Copy w",
                    "Delete j",
                    "Copy e",
                    "Copy p"
                ]))
            );
        }

        #[test]
        fn test12() {
            let from = "ybpqvati";
            let to = "ybpvoti";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Copy y",
                    "Copy b",
                    "Copy p",
                    "Delete q",
                    "Copy v",
                    "Replace a with o",
                    "Copy t",
                    "Copy i",
                ]))
            );
        }

        #[test]
        fn test13() {
            let from = "jafkvgyh";
            let to = "aogyu";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                Operation::count_ops(&result),
                Operation::count_ops(&Operation::from_raw(&[
                    "Delete j",
                    "Copy a",
                    "Delete f",
                    "Delete k",
                    "Replace v with o",
                    "Copy g",
                    "Copy y",
                    "Replace h with u"
                ]))
            );
        }
    }
}
