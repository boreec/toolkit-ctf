extern crate toolkit_ctf;

use toolkit_ctf::primality::primality_test_algorithms::{
    NaiveTrialDivision, SixKOneDivision,
};
use toolkit_ctf::primality::PrimalityTest;

use std::time::Instant;

const N: u64 = 221657383;

fn run_naive_trial_division(ntd: &NaiveTrialDivision) {
    let start = Instant::now();
    let result = ntd.is_prime(N);
    let duration = start.elapsed();
    println!(
        "prime: {result}, i: {}, bound: {:?}, \t t: {:?}",
        ntd.increment, ntd.upper_bound, duration
    );
}

fn run_six_k_one_division(skod: &SixKOneDivision) {
    let start = Instant::now();
    let result = skod.is_prime(N);
    let duration = start.elapsed();
    println!(
        "prime: {result}, bound: {:?} \t t: {:?}",
        skod.upper_bound, duration
    );
}

fn main() {
    println!("N: {N}");
    println!("=== naive trial division ===");
    run_naive_trial_division(&NaiveTrialDivision { increment: 1, upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Whole });
    run_naive_trial_division(&NaiveTrialDivision { increment: 1, upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Half });
    run_naive_trial_division(&NaiveTrialDivision { increment: 1, upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Square });
    run_naive_trial_division(&NaiveTrialDivision { increment: 2, upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Whole });
    run_naive_trial_division(&NaiveTrialDivision { increment: 2, upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Half });
    run_naive_trial_division(&NaiveTrialDivision { increment: 2, upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Square });

    println!("");
    println!("=== 6k+i division ===");
    run_six_k_one_division(&SixKOneDivision { upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Whole });
    run_six_k_one_division(&SixKOneDivision { upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Half});
    run_six_k_one_division(&SixKOneDivision { upper_bound: toolkit_ctf::primality::primality_test_algorithms::DivisionUpperBound::Square });
}
