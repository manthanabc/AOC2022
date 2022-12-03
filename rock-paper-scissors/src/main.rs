use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "/input.txt");

    let contents: String = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let mut score = 0;

    for games in contents.split('\n') {
        if games.is_empty() {
            continue;
        }

        println!("{games}");


        let mut games = games.split(' ');
        let p1 = games.next().unwrap().chars().next().unwrap();

        let p2 = games.next().unwrap().chars().next().unwrap();

        println!("{p1}, {p2}");

        let p2 = match p1 {
            'A' => { if p2 == 'X' {'Z'} else if p2 == 'Y' {'X'} else {'Y'} },
            'B' => { if p2 == 'X' {'X'} else if p2 == 'Y' {'Y'} else {'Z'} },
            'C' => { if p2 == 'X' {'Y'} else if p2 == 'Y' {'Z'} else {'X'} },
            _   => { panic!("fKDJFLKSD"); }
        };

        
         score += if p2 == 'X' {1} else if p2 == 'Y' {2} else {3};
         score += match (p1, p2) {                                   //// Part 1
            ('A', 'Y') => 6,
            ('A', 'Z') => 0,
            ('B', 'X') => 0,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            ('C', 'Y') => 0,
            _ => 3
        };

        println!("{score}");
    }

    println!("With text:\n {score}");
}
