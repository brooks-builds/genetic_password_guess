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
    let mut selected = chromosomes[0..keep_count].to_vec();
    let mut rng = thread_rng();

    let nongraded_selected = chromosomes[keep_count..]
        .choose_multiple(&mut rng, keep_ungraded_count)
        .map(ToOwned::to_owned)
        .collect();

    let selected = [selected, nongraded_selected].concat();

    selected
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
        let mut chromosome_size = 8;
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
}
