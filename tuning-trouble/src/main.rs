
use std::time::Instant;
fn main() {
    let mut input = include_str!("../input.prod").lines();
    let now = Instant::now();
    let mut acc = vec![];
    input.next().unwrap().chars().enumerate().fold(acc, |mut acc, (n, a)| {
//        println!("start of {n} acc {:?}, a {a}", acc);
        if(acc.len() > 13) {
            dbg!(n);        dbg!(now.elapsed());
 panic!();
        }
        if acc.contains(&a) {
                let p = acc.iter().position(|g| *g == a).unwrap();
                acc=acc.split_off(p+1);
        }
        acc.push(a);
//        println!("acc {:?}, a {a} \n\n", acc);
        acc
    });
}
