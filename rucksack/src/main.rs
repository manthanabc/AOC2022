fn main() {
    let input = include_str!("../input.test.txt").lines();
    let inputt:Vec<_> = input
        .map(|i| (i[..i.len()/2].to_owned(), i[i.len()/2..].to_owned()))
        .map(|(a, b)| { a.chars().find(|&c| b.contains(c)).unwrap()})
        .map(|c| if c.is_lowercase() { c as u32 - 96} else { c as u32 - 38}).collect();

    // part 2
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut sum = 0;
    for i in 0..input.len()/3 {
            
            let index = (i)*3+2;
            
            let s1 = input[index - 2];
            let s2 = input[index - 1];
            let s3 = input[index - 0];

            let f2: Vec<char> = s1.chars().collect();//.filter(|&c| s2.contains(c));
            let f3= f2.into_iter().find(|c| s2.contains(*c) && s3.contains(*c)).unwrap();
            sum += if f3.is_lowercase() { f3 as u32 - 96} else { f3 as u32 - 38};
            println!("{:?} \n{s1} \n{s2}", f3);
        }
    println!("{:?} {sum}", inputt.iter().sum::<u32>());
}
