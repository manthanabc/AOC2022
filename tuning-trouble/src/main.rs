fn main() {
    let mut input = include_str!("../input.prod").lines();
    let mut acc = vec![];
    // input.next();
    input.next().unwrap().chars().enumerate().fold(acc, |mut acc, (n, a)| {
        println!("start of {n} acc {:?}, a {a}", acc);
        if(acc.len() > 13) {
            acc.remove(0);
            if !acc.contains(&a) {dbg!(n); panic!();}
        }
        if acc.contains(&a) {
                let p = acc.iter().position(|g| *g == a).unwrap();
                acc=acc.split_off(p+1);
        }
        acc.push(a);
        if(acc.len() == 14) {
            dbg!(n+1); panic!();
        }
        println!("acc {:?}, a {a} \n\n", acc);

        acc
    });
    //(0..input.len()).for_each(|e|)
}
