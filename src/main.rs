use itertools::Itertools;

fn main() {
    eprintln!("Getting words");
    let words = include_str!("words.txt");
    let words = words.lines().map(ToString::to_string).filter(|x| x.len() <= 8).collect_vec();
    eprintln!("Read words");

    let mut input = String::new();
    eprintln!("What letters:");

    std::io::stdin().read_line(&mut input).expect("unable to read in letters");
    let input = input.trim();

    let to_check = input.clone().chars().permutations(5).chain(input.clone().chars().permutations(6)).chain(input.clone().chars().permutations(7)).chain(input.clone().chars().permutations(8));

    eprintln!("got candidates");

    for chars in to_check {
        let len = chars.len();

        let mut string = String::with_capacity(len);
        for ch in chars {
            string.push(ch);
        }


        if words.contains(&string) {
            println!("{string}");
        }
    }

}
