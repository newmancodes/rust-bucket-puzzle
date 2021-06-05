mod problem;
mod solvers;

use problem::builder::Builder;
use problem::rules::Rules;
use solvers::iterative_deepening_solver::IterativeDeepeningSolver;
use solvers::solution::Solution;
use solvers::solver::Solver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_help_john_mcclane_and_zeus() {
        // Arrange
        let solver = IterativeDeepeningSolver::new(10);
        let problem = Builder::new()
            .add_empty_bucket(5)
            .add_empty_bucket(3)
            .build();
        let rules = Rules::new(true, true);
        let target: u8 = 4;

        // Act
        let solution = solver.solve(&problem, &rules, target);

        // Assert
        assert!(solution.is_some())
    }
}
