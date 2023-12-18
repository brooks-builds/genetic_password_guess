use eyre::{bail, Result};
use rand::{
    distributions::{Alphanumeric, DistString, Uniform},
    seq::SliceRandom,
    thread_rng, Rng,
};

pub fn create_chromosome(size: usize) -> String {
    let mut rng = thread_rng();
    Alphanumeric.sample_string(&mut rng, size)
}

pub fn get_score(chromosone: &str, answer: &str) -> Result<f32> {
    if chromosone.len() != answer.len() {
        bail!("chromose and answer must be the same length");
    }

    let correct = chromosone
        .chars()
        .zip(answer.chars())
        .filter(|(chromosone, answer)| chromosone == answer)
        .count() as f32;

    Ok(correct / answer.len() as f32)
}

pub fn selection(
    chromosomes: &mut [String],
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
    answer: &str,
) -> Vec<String> {
    chromosomes.sort_unstable_by(|a, b| {
        get_score(b, answer)
            .unwrap()
            .partial_cmp(&get_score(a, answer).unwrap())
            .unwrap()
    });
    let keep_count = (chromosomes.len() as f32 * graded_retain_percent) as usize;
    let keep_ungraded_count = (chromosomes.len() as f32 * nongraded_retain_percent) as usize;
    let selected = chromosomes[0..keep_count].to_vec();
    let mut rng = thread_rng();

    let nongraded_selected = chromosomes[keep_count..]
        .choose_multiple(&mut rng, keep_ungraded_count)
        .map(ToOwned::to_owned)
        .collect();

    let selected = [selected, nongraded_selected].concat();

    selected
}

pub fn mutate(chromosone: &str) -> String {
    let mut rng = thread_rng();
    let random_character = rng.sample(Alphanumeric) as char;
    let index = rng.gen_range(0..chromosone.len());
    let mut characters = chromosone.chars().collect::<Vec<char>>();

    characters[index] = random_character;
    characters.into_iter().collect()
}

pub fn crossover(chromosone_one: &str, chromosone_two: &str) -> String {
    let length = chromosone_one.len() / 2;

    format!(
        "{}{}",
        &chromosone_one[0..length],
        &chromosone_two[length..]
    )
}

pub fn create_population(size: usize, chromosone_size: usize) -> Vec<String> {
    let mut population = vec![];
    for _ in 0..size {
        population.push(create_chromosome(chromosone_size));
    }

    population
}

pub fn generation(
    mut population: Vec<String>,
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
    answer: &str,
) -> Vec<String> {
    let mut survivors = selection(
        &mut population,
        graded_retain_percent,
        nongraded_retain_percent,
        answer,
    );
    let mut children = vec![];
    let mut rng = thread_rng();

    while (children.len() + survivors.len()) < population.len() {
        let parent_1 = survivors.choose(&mut rng).unwrap();
        let parent_2 = survivors.choose(&mut rng).unwrap();
        let mut child = crossover(parent_1, &parent_2);
        let mutate_chance = rng.gen_range(0..1000);
        if mutate_chance <= 1 {
            child = mutate(&child);
        }
        children.push(child);
    }

    survivors.extend(children);

    survivors
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn should_create_chromosome() {
        let size = 8;
        let answer = create_chromosome(size);

        assert_eq!(answer.len(), size);
    }

    #[test]
    fn should_score_string() -> Result<()> {
        let chromosone = "ayfuwpts";
        let answer = "afuuwtss";

        let expected_result = 0.5;

        assert_eq!(get_score(chromosone, answer)?, expected_result);

        Ok(())
    }

    #[test]
    fn should_select_chromosomes() {
        let mut chromosomes = vec![
            "afuuwtss".to_owned(),
            "afuuwtsz".to_owned(),
            "afuuwtzz".to_owned(),
            "afuuwzzz".to_owned(),
            "afuuzzzz".to_owned(),
            "afuzzzzz".to_owned(),
            "afzzzzzz".to_owned(),
            "azzzzzzz".to_owned(),
            "zzzzzzzz".to_owned(),
            "zzzzzzza".to_owned(),
            "zzzzzzaa".to_owned(),
        ];
        let graded_retain_percent = 0.3;
        let nongraded_retain_percent = 0.2;
        let answer = "afuuwtss";
        let mut selected = selection(
            &mut chromosomes,
            graded_retain_percent,
            nongraded_retain_percent,
            answer,
        );
        let expected_selected = vec![
            "afuuwtss".to_owned(),
            "afuuwtsz".to_owned(),
            "afuuwtzz".to_owned(),
        ];
        let expected_length = 5;

        assert_eq!(selected.len(), expected_length);

        for expected in &expected_selected {
            assert!(selected.contains(expected));
        }

        selected.sort();
        selected.dedup();
        assert_eq!(selected.len(), expected_length);
    }

    #[test]
    fn should_crossover_two_chromosones() {
        let chromosone_one = "AAAAAAAA";
        let chromosone_two = "BBBBBBBB";
        let result = crossover(chromosone_one, chromosone_two);
        let expected_result = "AAAABBBB";

        assert_eq!(result, *expected_result);
    }

    #[test]
    fn should_mutate_chromosone() {
        let chromosone = "aaaaaa";
        let mutated = mutate(chromosone);
        let a_count = mutated.matches('a').count();
        let expected = 5;
        assert_ne!(chromosone, mutated);

        assert_eq!(a_count, expected);
    }

    #[test]
    fn should_create_a_population() {
        let population = create_population(10, 8);

        assert_eq!(population.len(), 10);
        assert_eq!(population[0].len(), 8);
    }
}
