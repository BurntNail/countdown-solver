use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn main() {
    let mut input = String::new();
    eprintln!("What letters:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("unable to read in letters");

    let input = input.trim();
    let words = include_str!("words.txt")
        .lines()
        .filter(|x| x.len() <= 9)
        .collect::<Vec<_>>();

    const NO_RUNS: u32 = 1000;
    let mut total_times_taken = Duration::default();
    for _ in 0..NO_RUNS {
        let start = Instant::now();
        let worked = get_all_matches(&words, input);
        black_box(worked);

        total_times_taken += start.elapsed();
    }
    let time_took = total_times_taken / NO_RUNS;

    eprintln!("Took {time_took:?}");

    let mut sorted_by_len: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in get_all_matches(&words, input) {
        sorted_by_len.entry(word.len()).or_default().push(word);
    }

    for i in (1..=9).rev() {
        println!("Length {i}");

        for word in sorted_by_len.remove(&i).unwrap_or_default() {
            println!("\t{word}");
        }
    }
}

fn get_all_matches<'a>(words: &[&'a str], input: &str) -> Vec<&'a str> {
    let to_check = word_to_bitmap(&input);

    let mut counted = HashMap::new();
    for ch in input.chars() {
        *counted.entry(ch).or_insert(0) += 1;
    }

    words
        .into_iter()
        .filter(|x| (!to_check & word_to_bitmap(x)) == 0)
        .filter(|p| {
            let mut test_counted = counted.clone();

            p.chars().all(|ch| {
                let relvant_entry = test_counted.entry(ch).or_insert(0);

                if *relvant_entry == 0 {
                    false
                } else {
                    *relvant_entry -= 1;
                    true
                }
            })
        })
        .map(|p| *p)
        .collect::<Vec<_>>()
}

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

fn word_to_bitmap(w: &str) -> u32 {
    let mut out = 0;

    for ch in w.chars() {
        out |= 1 << (ch as u8 - b'a');
    }

    out
}
