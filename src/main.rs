use std::collections::HashMap;

fn main() {
    eprintln!("NB: ignores frequency so you'll have to double check");

    let mut input = String::new();
    eprintln!("What letters:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("unable to read in letters");
    let input = input.trim();

    let mut counted = HashMap::new();
    for ch in input.chars() {
        *counted.entry(ch).or_insert(0) += 1;
    }

    eprintln!("{counted:?}");

    // let to_check = word_to_bitmap(&input);

    let possibilities = include_str!("words.txt")
        .lines()
        .filter(|x| x.len() <= input.len())
        .map(|w| w.to_string());
        // .filter(|x| word_to_bitmap(&x) == to_check);


    let worked = possibilities.filter(|p| {
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
    });

    let (longest_len, longest_items) = worked.fold((0, vec![]), |(mut longest_len, mut longest), p| {
        eprintln!("{p}");

        let len = p.len();

        if len == longest_len {
            longest.push(p);
        } else if len > longest_len {
            longest_len = len;
            longest.clear();
            longest.push(p);
        }

        (longest_len, longest)
    });

    println!("Length: {longest_len}");
    for p in longest_items {
        println!("{p}");
    }
}

// fn word_to_bitmap(w: &str) -> u32 {
//     let mut out = 0;

//     for ch in w.chars() {
//         out |= 1 << (ch as u8 - b'a');
//     }

//     out
// }
