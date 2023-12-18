use genetic_password_guess::{create_population, generation, get_score};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
};

const PASSWORD: &str = "Aoljfon oaeznFjlf";

fn main() {
    let answer = "KeyboardCat";
    let chromosone_size = answer.len();
    let population_size = 10;
    let mut population = create_population(population_size, chromosone_size);
    let mut answers: Vec<String> = vec![];
    let graded_retain_percent = 0.3;
    let nongraded_retain_percent = 0.2;
    let mut generations = 1;

    while answers.is_empty() {
        population = generation(
            population,
            graded_retain_percent,
            nongraded_retain_percent,
            answer,
        );

        dbg!(
            "score of first individual:",
            get_score(&population[0], answer).unwrap()
        );

        for individual in &population {
            if individual.as_str() == answer {
                answers.push(individual.to_owned());
            }
        }

        generations += 1;
    }

    println!("We found the answer {answers:?} in {generations} generations");
}
