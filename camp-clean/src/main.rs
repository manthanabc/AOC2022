fn main() {
    let input = include_str!("../input.prod").lines();
    let mut sum = 0;
    input.for_each(|f| { let (x1, x2) = f.split_once(',').unwrap(); if contains(x1, x2) { sum += 1 };});

    println!("{sum}");
}

fn contains(a: &str, b: &str) -> bool {
    let (a1, a2) = a.split_once('-').unwrap();
    let (b1, b2) = b.split_once('-').unwrap();
   
    let (a1, a2) = (a1.parse::<u16>().unwrap(), a2.parse::<u16>().unwrap());
    let (b1, b2) = (b1.parse::<u16>().unwrap(), b2.parse::<u16>().unwrap());


    //let s = (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2);   // part 1
    let s = (a1 <= b1 && a2 >= b1) || (b1 <= a1 && b2 >= a1);     // part 2

    println!("{a1}-{a2} {b1}-{b2}  {s}");
    s
}
