use std::cmp::max;
use std::collections::HashMap;

pub struct Assignment6;

type Int = u32;

/// The work entry is a tuple of the work type and the num of jobs left
/// including the current job.
type WorkEntry = (Option<WorkType>, usize);

/// The work type is either easy or hard
#[derive(Hash, Eq, PartialEq)]
enum WorkType {
    Easy,
    Hard,
}

impl Assignment6 {
    #[allow(dead_code)]
    pub fn find_max_profit(e: &[Int], h: &[Int]) -> Int {
        assert_eq!(e.len(), h.len(), "Both arrays must be of equal length");
        let n = e.len();    
        assert!(n > 0);

        let mut res = vec![0; n];

        // Handle two base cases
        res[0] = e[0];
        if n == 1 {
            return res[0];
        }
        res[1] = max(e[0] + e[1], h[1]);
        if n == 2 {
            return res[1];
        }

        for i in 2..n {
            let temp = max(res[i - 2] + h[i], res[i - 1] + e[i]);
            res[i] = max(temp, res[i - 2] + e[i - 2] + e[i - 1]);
        }

        *res.last().expect("Empty array")
    }

    #[allow(dead_code)]
    pub fn scheduler(easy_job: &[Int], hard_job: &[Int]) -> Int {
        assert_eq!(
            easy_job.len(),
            hard_job.len(),
            "Jobs prices must be of equal length"
        );

        let mut cache = HashMap::with_capacity(easy_job.len() * hard_job.len());
        Self::scheduler_helper_recursive(easy_job, hard_job, true, &mut cache)
    }
    fn scheduler_helper_recursive(
        easy_job: &[Int],
        hard_job: &[Int],
        worked_last_week: bool,
        cache: &mut HashMap<WorkEntry, Int>,
    ) -> Int {
        let n = easy_job.len();
        if n == 0 {
            return 0;
        }

        let (&easy_pay, easy_job) = easy_job.split_first().unwrap();
        let (&hard_pay, hard_job) = hard_job.split_first().unwrap();

        let easy_res = if let Some(&result) = cache.get(&(Some(WorkType::Easy), n)) {
            result
        } else {
            let easy_check =
                easy_pay + Self::scheduler_helper_recursive(easy_job, hard_job, true, cache);
            cache.insert((Some(WorkType::Easy), n), easy_check);
            easy_check
        };

        let hard_res = {
            let cache_result = if worked_last_week {
                cache.get(&(None, n))
            } else {
                cache.get(&(Some(WorkType::Hard), n))
            };

            if let Some(&result) = cache_result {
                result
            } else {
                let hard_check = if worked_last_week { 0 } else { hard_pay }
                    + Self::scheduler_helper_recursive(
                        easy_job,
                        hard_job,
                        !worked_last_week,
                        cache,
                    );
                if worked_last_week {
                    cache.insert((None, n), hard_check);
                } else {
                    cache.insert((Some(WorkType::Hard), n), hard_check);
                }
                hard_check
            }
        };

        max(hard_res, easy_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_example1() {
        let easy_job = [10, 20, 30];
        let hard_job = [0, 50, 0];
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 80);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 80);
    }

    #[test]
    fn test_scheduler_example3() {
        let easy_job = [20, 30, 50, 25, 5];
        let hard_job = [0, 2, 700, 5, 500];
        // Optimal: 20 + 700 + 500 = 1220
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 20 + 700 + 500);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 1220);
    }

    #[test]
    fn test_scheduler_example2() {
        let easy_job = [10, 20, 30, 40];
        let hard_job = [0, 50, 0, 60];
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 50 + 30 + 40);
        assert_eq!(
            Assignment6::find_max_profit(&easy_job, &hard_job),
            50 + 30 + 40
        );
    }

    #[test]
    fn test_scheduler_one_week_easy() {
        let easy_job = [25];
        let hard_job = [0];
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 25);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 25);
    }

    #[test]
    fn test_scheduler_one_week_hard() {
        let easy_job = [0];
        let hard_job = [50]; // Cannot do hard job in the first week
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 0);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 0);
    }

    #[test]
    fn test_scheduler_two_weeks_easy() {
        let easy_job = [10, 20];
        let hard_job = [0, 0];
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 30);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 30);
    }

    #[test]
    fn test_scheduler_two_weeks_hard() {
        let easy_job = [10, 20];
        let hard_job = [99990, 50];
        // Optimal: 10 + 50 = 60
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 50);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 50);
    }

    #[test]
    fn test_scheduler_mixed_case() {
        let easy_job = [5, 15, 25, 800];
        let hard_job = [0, 40, 0, 8000];
        // Optimal: 5 + 40 + 25 + 800 = 870
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 40 + 8000);
        assert_eq!(
            Assignment6::find_max_profit(&easy_job, &hard_job),
            40 + 8000
        );
    }

    #[test]
    fn test_scheduler_all_easy_jobs() {
        let easy_job = [5, 10, 15, 20, 25];
        let hard_job = [0, 0, 0, 0, 0];
        // 5e + 10e + 15e + 20e + 25e = 75
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 75);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 75);
    }

    #[test]
    fn test_scheduler_all_hard_jobs() {
        let easy_job = [0, 0, 0, 0, 0];
        let hard_job = [0, 30, 0, 50, 0];
        // 0e + 30h + 0e + 50h + 0e = 80
        assert_eq!(Assignment6::scheduler(&easy_job, &hard_job), 80);
        assert_eq!(Assignment6::find_max_profit(&easy_job, &hard_job), 80);
    }

    // Test case where all easy jobs are best
    #[test]
    fn test_all_easy() {
        let easy = [10, 10, 10];
        let hard = [0, 0, 0];
        assert_eq!(Assignment6::scheduler(&easy, &hard), 30);
        assert_eq!(Assignment6::find_max_profit(&easy, &hard), 30);
    }

    // Test optimal hard job alternation
    #[test]
    fn test_hard_job_alternate() {
        let easy = [1, 3, 1, 3];
        let hard = [0, 5, 0, 5];
        // Optimal: 1 + 5 + 1 + 5 = 12
        assert_eq!(Assignment6::scheduler(&easy, &hard), 5 + 5);
        assert_eq!(Assignment6::find_max_profit(&easy, &hard), 5 + 5);
    }

    #[test]
    fn test_empty_input_skip_one_hard_day() {
        let easy = [20, 30, 40];
        let hard = [999999999, 50, 300];
        // Optimal: 20 + 30 + 300 = 350
        assert_eq!(Assignment6::scheduler(&easy, &hard), 20 + 300);
        assert_eq!(Assignment6::find_max_profit(&easy, &hard), 20 + 300);
    }
}
