use itertools::Itertools;

fn main() {
    let input = include_str!("../input.prod").lines();
    let mut stacks: Vec<Vec<String>> = vec![Vec::new();9];

    let mut it = input.enumerate();
    while let Some((i, line)) = it.next() {

        let line = line.replace("    ", " ");
        if line.is_empty() { continue };

        if line.split(' ').any(|l| l.chars().any(|a| a.is_digit(10))) {
            if !line.contains("move") { continue };
            if let Some((_, mov, _, from, _, to)) = line.split(' ').collect_tuple(){
                let (mov, from, to) = (mov.parse::<usize>().unwrap(), from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap());
                (0..mov).for_each(|n| {
                         let v = stacks[from-1].remove(mov-n-1);  // .remove(0) for part 1
                         stacks[to-1].insert(0, v);       
                });
            };
            continue
        };
        
        line.split(' ').enumerate().for_each(|(j, n)| {
            if !n.is_empty() {
            stacks[j].push(n.to_string());
            };
            println!("{i} {:?}", n); 
        });
    }
    stacks.iter().for_each(|f| {println!("{}", f[0])});
}

