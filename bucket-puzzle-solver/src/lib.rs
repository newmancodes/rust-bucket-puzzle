mod problem;
mod solvers;

use problem::bucket::Bucket;
use problem::state::State;
use solvers::solver::Solver;
use solvers::solution::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_help_john_mcclane_and_zeus() {
        // Arrange
        let solver: Solver;
        let problem: State;

        // Act
        let solution: Option<Solution> = solver.solve(&problem);

        // Assert
        assert!(solution.is_some())
    }
}
