use std::collections::BTreeSet;

use crate::lib::*;

/// Input:
/// 
/// 3
/// 
/// 4 2 1
/// 
/// Output:
/// 
/// 1 2 1 2 1 3 1
pub fn main() {
    let varieties: Vec<u32> = parse_two_line_input();
    if let Some(solution) = solve(varieties) {
        for variety in solution.iter() {
            print!("{} ", *variety + 1);
        }
        println!();
    } else {
        println!("-");
    }
}

pub fn solve(varieties: Vec<u32>) -> Option<Vec<usize>> {
    let mut step = SolutionStep::new(varieties);
    step.solve()
}

#[derive(Clone, Debug)]
struct SolutionStep {
    varieties_freqs: Vec<u32>,
    varieties_used: Vec<usize>,
    available_varieties: BTreeSet<usize>,
}
impl SolutionStep {
    pub fn new(varieties_freqs: Vec<u32>) -> SolutionStep {
        let available_varieties = (0..varieties_freqs.len()).collect();
        SolutionStep {
            varieties_freqs,
            varieties_used: Vec::new(),
            available_varieties,
        }
    }

    pub fn solve(&mut self) -> Option<Vec<usize>> {
        if self.is_solution() {
            let result = std::mem::replace(&mut self.varieties_used, Vec::new());
            return Some(result);
        }
        for candidate_variety in self.feasible_varieties().iter() {
            if let Some(used) = self.with_variety_used(*candidate_variety, SolutionStep::solve) {
                return Some(used);
            }
        }

        None
    }

    pub fn is_solution(&self) -> bool {
        self.varieties_freqs.iter().all(|v| *v == 0)
    }

    fn use_variety(&mut self, variety: usize) {
        if let Some(count) = self.varieties_freqs.get_mut(variety) {
            *count -= 1;
            if *count == 0 {
                self.available_varieties.remove(&variety);
            }
            self.varieties_used.push(variety);
        }
    }

    fn backtrack(&mut self) {
        if let Some(variety) = self.varieties_used.pop() {
            let count = self.varieties_freqs.get_mut(variety).unwrap();
            *count += 1;
            if *count == 1 {
                self.available_varieties.insert(variety);
            }
        }
    }

    fn feasible_varieties(&self) -> Vec<usize> {
        let last = self.varieties_used.last().cloned();
        self
            .available_varieties
            .iter()
            .cloned()
            .filter(move |variety| last.filter(|l| *l == *variety).is_none())
            .collect()
    }

    fn with_variety_used<R, F: FnMut(&mut Self) -> R>(&mut self, variety: usize, mut f: F) -> R {
        self.use_variety(variety);
        let result = f(self);
        self.backtrack();
        result
    }

}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test_input_421() {
        assert_eq!(solve(vec![4, 2, 1]), Some(vec![0, 1, 0, 1, 0, 2, 0]));
    }

}
