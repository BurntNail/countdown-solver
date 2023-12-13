use std::{collections::HashMap, time::Instant};

fn main() {
    eprintln!("NB: ignores frequency so you'll have to double check");

    let mut input = String::new();
    eprintln!("What letters:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("unable to read in letters");

    let input = input.trim();
    let words = include_str!("words.txt").lines().filter(|x| x.len() <= 9).collect::<Vec<_>>();

    let start = Instant::now();
    let worked = get_all_matches(words, input);
    let time_took = start.elapsed();

    eprintln!("Took {time_took:?}");

    let mut sorted_by_len: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in worked {
        sorted_by_len.entry(word.len()).or_default().push(word);
    }

    for i in (1..=9).rev() {
        println!("Length {i}");

        for word in sorted_by_len.remove(&i).unwrap_or_default() {
            println!("\t{word}");
        }
    }
}

fn get_all_matches<'a> (words: Vec<&'a str>, input: &str) -> Vec<&'a str> {
    let to_check = word_to_bitmap(&input);

    let mut counted = HashMap::new();
    for ch in input.chars() {
        *counted.entry(ch).or_insert(0) += 1;
    }        

    words.into_iter()
        .filter(|x| {
            let bitmap = word_to_bitmap(x);
            let mut works = true;

            for i in 0..32 {
                let bitmap = bitmap & (1 << i);
                let to_check = to_check & (1 << i);

                if bitmap > to_check {
                    works = false;
                    break;
                }
            }

            works
        })
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
        .collect::<Vec<_>>()
}

fn word_to_bitmap(w: &str) -> u32 {
    let mut out = 0;

    for ch in w.chars() {
        out |= 1 << (ch as u8 - b'a');
    }

    out
}
