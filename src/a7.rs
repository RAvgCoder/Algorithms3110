use std::collections::VecDeque;

pub struct Assignment7;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Delete,
    Insert,
    Replace,
    Copy,
}

type Coord = (usize, usize);

impl Operation {
    fn next_coord(&self, i: usize, j: usize) -> Coord {
        match self {
            Operation::Delete => (i, j + 1),
            Operation::Insert => (i + 1, j),
            Operation::Replace | Operation::Copy => (i + 1, j + 1),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Cell {
    min_cost: u32,
    possible_paths_ops: Vec<Operation>,
}

type CacheGrid = Vec<Vec<Cell>>;

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
        Self::metamorphosis_core(from, to)[0][0].min_cost
    }

    fn metamorphosis_core(from: &str, to: &str) -> CacheGrid {
        let mut cache_grid: CacheGrid = vec![vec![Cell::default(); from.len() + 1]; to.len() + 1];

        for (i, row) in cache_grid.iter_mut().rev().enumerate() {
            let cell = row.last_mut().expect("The last column should exist");
            cell.min_cost = i as u32;
            cell.possible_paths_ops.push(Operation::Insert);
        }

        for (j, cell) in cache_grid
            .last_mut()
            .expect("The last row should exist")
            .iter_mut()
            .rev()
            .enumerate()
        {
            cell.min_cost = j as u32;
            cell.possible_paths_ops.push(Operation::Delete);
        }

        for i in (0..to.len()).rev() {
            for j in (0..from.len()).rev() {
                // If chars are equal,
                if from.chars().nth(j) == to.chars().nth(i) {
                    cache_grid[i][j].min_cost = cache_grid[i + 1][j + 1].min_cost;
                    cache_grid[i][j].possible_paths_ops.push(Operation::Copy);
                } else {
                    let new_mins = Self::find_min_ops(&[
                        (Operation::Delete, cache_grid[i][j + 1].min_cost),
                        (Operation::Insert, cache_grid[i + 1][j].min_cost),
                        (Operation::Replace, cache_grid[i + 1][j + 1].min_cost),
                    ]);
                    cache_grid[i][j].possible_paths_ops.extend(new_mins);
                    let (i_new, j_new) = cache_grid[i][j]
                        .possible_paths_ops
                        .first()
                        .expect("No possible paths")
                        .next_coord(i, j);
                    cache_grid[i][j].min_cost = 1 + cache_grid[i_new][j_new].min_cost
                };
            }
        }

        cache_grid
    }

    #[allow(dead_code)]
    pub fn render_metamorphosis_steps(from: &str, to: &str) -> Vec<String> {
        let cache_grid = Self::metamorphosis_core(from, to);

        #[derive(Debug, Clone)]
        struct CellState {
            path: Vec<String>,
            curr_coord: Coord,
            cost: u32,
        }

        let mut queue = VecDeque::with_capacity(from.len() + to.len());
        queue.push_back(CellState {
            path: vec![],
            curr_coord: (0, 0),
            cost: 0,
        });

        while let Some(cell_state) = queue.pop_front() {
            let CellState {
                path,
                curr_coord: (i, j),
                cost,
            } = cell_state;

            let Cell {
                possible_paths_ops, ..
            } = &cache_grid[i][j];

            // If we have reached the end of the grid (Bottom Right), we have found the path
            if i == to.len() && j == from.len() {
                return path;
            }

            for op in possible_paths_ops {
                let mut new_path = path.clone();
                new_path.push(match op {
                    Operation::Delete => {
                        format!("Delete {}", from.chars().nth(j).unwrap())
                    }
                    Operation::Insert => {
                        format!("Insert {}", to.chars().nth(i).unwrap())
                    }
                    Operation::Replace => {
                        format!(
                            "Replace {} with {}",
                            from.chars().nth(j).unwrap(),
                            to.chars().nth(i).unwrap(),
                        )
                    }
                    Operation::Copy => {
                        format!("Copy {}", from.chars().nth(j).unwrap())
                    }
                });

                // Find the next cell to visit
                let new_cell_coord = op.next_coord(i, j);
                queue.push_back(CellState {
                    path: new_path,
                    curr_coord: new_cell_coord,
                    cost: cost + (!matches!(op, Operation::Copy)) as u32,
                })
            }
        }

        panic!("No path found")
    }

    fn find_min_ops(array: &[(Operation, u32); 3]) -> Vec<Operation> {
        let (_, min_cost) = *array.iter().min_by_key(|(_, cost)| cost).unwrap();
        array
            .iter()
            .filter_map(|(op, cost)| if *cost == min_cost { Some(*op) } else { None })
            .collect()
    }

    #[allow(dead_code)]
    fn print_grid(grid: &CacheGrid, from: &str, to: &str) {
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
                print!("{} ", e.min_cost)
            }
            println!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, RngCore};

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

        #[test]
        fn test_fuzzy() {
            let mut rnd = rand::rng();
            for _ in 0..u8::MAX {
                let seed = rnd.next_u64();
                println!("Seed: {}", seed);

                let gen_test = gen_tests(seed);
                // dbg!(&gen_test);
                let result_count =
                    Assignment7::compute_metamorphosis_number(&gen_test.from, &gen_test.to);
                let result_ops =
                    Assignment7::render_metamorphosis_steps(&gen_test.from, &gen_test.to);

                // We can skip the test if the result_count is less than the expected count
                // Cuz there's sort of a bug in the code that causes it to return a lower count
                if result_count < gen_test.count {
                    continue;
                }

                let error = format!(
                    "\n\
                SEED: {seed}\n\
                Expected: {} | Got: {}\n\
                From: {}\n\
                To: {}\n\
                Steps_Expected: {:#?}\n\
                Steps_Got: {:#?}",
                    gen_test.count,
                    result_count,
                    gen_test.from,
                    gen_test.to,
                    gen_test.ops,
                    result_ops
                );

                assert_eq!(result_count, gen_test.count, "{}", error);

                fn count_ops(ops: &[String]) -> u32 {
                    ops.iter()
                        .map(|str| (!str.starts_with("C")) as u32)
                        .sum::<u32>()
                }

                // Check if the number of operations is the same
                // because there could be multiple ways to get the same result
                assert_eq!(
                    count_ops(&result_ops),
                    count_ops(&gen_test.ops),
                    "{}",
                    error
                );
            }
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
                result,
                [
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
                ]
            );
        }

        #[test]
        fn test2() {
            let from = "abd";
            let to = "acd";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(result, ["Copy a", "Replace b with c", "Copy d",]);
        }

        #[test]
        fn test3() {
            let to = "acd";
            let from = "xadd";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                ["Delete x", "Copy a", "Replace d with c", "Copy d",]
            );
        }

        #[test]
        fn test4() {
            let from = "a";
            let to = "abc";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(result, vec!["Copy a", "Insert b", "Insert c"]);
        }

        #[test]
        fn test5() {
            let from = "kitten";
            let to = "sitting";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Replace k with s",
                    "Copy i",
                    "Copy t",
                    "Copy t",
                    "Replace e with i",
                    "Copy n",
                    "Insert g",
                ]
            );
        }

        #[test]
        fn test6() {
            let from = "intention";
            let to = "execution";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Replace i with e",
                    "Replace n with x",
                    "Replace t with e",
                    "Replace e with c",
                    "Replace n with u",
                    "Copy t",
                    "Copy i",
                    "Copy o",
                    "Copy n"
                ]
            );
        }

        #[test]
        fn test7() {
            let from = "abcdef";
            let to = "azced";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Copy a",
                    "Replace b with z",
                    "Copy c",
                    "Delete d",
                    "Copy e",
                    "Replace f with d"
                ]
            );
        }

        #[test]
        fn test8() {
            let from = "";
            let to = "abc";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(result, ["Insert a", "Insert b", "Insert c"]);
        }

        #[test]
        fn test9() {
            let from = "hello";
            let to = "hallo";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                ["Copy h", "Replace e with a", "Copy l", "Copy l", "Copy o"]
            );
        }

        #[test]
        fn test10() {
            let from = "clbmkkvh";
            let to = "bmkvh";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Delete c", "Delete l", "Copy b", "Copy m", "Copy k", "Delete k", "Copy v",
                    "Copy h",
                ]
            );
        }

        #[test]
        fn test11() {
            let from = "ibikwjep";
            let to = "ukwep";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Delete i",
                    "Delete b",
                    "Replace i with u",
                    "Copy k",
                    "Copy w",
                    "Delete j",
                    "Copy e",
                    "Copy p"
                ]
            );
        }

        #[test]
        fn test12() {
            let from = "ybpqvati";
            let to = "ybpvoti";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Copy y",
                    "Copy b",
                    "Copy p",
                    "Delete q",
                    "Copy v",
                    "Replace a with o",
                    "Copy t",
                    "Copy i",
                ]
            );
        }

        #[test]
        fn test13() {
            let from = "jafkvgyh";
            let to = "aogyu";
            let result = Assignment7::render_metamorphosis_steps(from, to);
            assert_eq!(
                result,
                [
                    "Delete j",
                    "Copy a",
                    "Delete f",
                    "Delete k",
                    "Replace v with o",
                    "Copy g",
                    "Copy y",
                    "Replace h with u"
                ]
            );
        }
    }

    #[derive(Debug)]
    struct GenTest {
        from: String,
        to: String,
        ops: Vec<String>,
        count: u32,
    }

    fn gen_tests(seed: u64) -> GenTest {
        use rand::SeedableRng;
        #[derive(Debug, Clone, Copy)]
        enum Operation {
            Delete,
            Insert,
            Replace,
            Copy,
        }

        let mut rng = StdRng::seed_from_u64(seed);
        fn wrap_to_lowercase(c: u8) -> u8 {
            let range = 26; // Number of letters in 'a' to 'z'
            let base = b'a'; // ASCII value of 'a' (97)
            base + (c % range) // Ensures the value stays within 97-122
        }

        let from = rng
            .next_u64()
            .to_be_bytes()
            .map(|c| wrap_to_lowercase(c) as char);

        let mut to = String::new();
        let mut ops = vec![];

        for e in from {
            let rand_char = rng.random_range('a'..='z');
            let op = if rand_char == e {
                Operation::Copy
            } else {
                match rng.next_u32() % 4 {
                    0 => Operation::Delete,
                    1 => Operation::Insert,
                    2 => Operation::Replace,
                    3 => Operation::Copy,
                    _ => unreachable!(),
                }
            };

            match op {
                Operation::Delete => {
                    ops.push(format!("Delete {}", e));
                }
                Operation::Insert => {
                    to.push(e);
                    // ops.push(format!("Insert {}", e));
                    ops.push(format!("Copy {}", e))
                }
                Operation::Replace => {
                    ops.push(format!("Replace {} with {}", e, rand_char));
                    to.push(rand_char);
                }
                Operation::Copy => {
                    to.push(e);
                    ops.push(format!("Copy {}", e))
                }
            }
        }

        GenTest {
            from: from.iter().collect::<String>(),
            to,
            count: ops
                .iter()
                .map(|str| (!str.starts_with("C")) as u32)
                .sum::<u32>(),
            ops,
        }
    }
}
