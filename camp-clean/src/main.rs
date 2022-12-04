fn main() {
    let input = include_str!("../input.prod").lines();
    let mut sum = input.filter(|f| { let (x1, x2) = f.split_once(',').unwrap(); contains(x1, x2) }).count();

    println!("{sum}");
}

fn contains(a: &str, b: &str) -> bool {
    let (a1, a2) = a.split_once('-').unwrap();
    let (b1, b2) = b.split_once('-').unwrap();
   
    let (a1, a2) = (a1.parse::<u16>().unwrap(), a2.parse::<u16>().unwrap());
    let (b1, b2) = (b1.parse::<u16>().unwrap(), b2.parse::<u16>().unwrap());


   // (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)   // part 1
    (a1 <= b1 && a2 >= b1) || (b1 <= a1 && b2 >= a1)     // part 2
}
