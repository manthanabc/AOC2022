#[derive(Debug)]
struct position(i32, i32);

fn main() {
    let input = include_str!("../input.test");
    let mut head = position(0, 0);
    let mut tail = position(0, 0);
    input.lines().for_each(|line| {
        match line.split_once(" ").unwrap() {
            ("R", x) => head.0 += x.parse::<u32>().unwrap(),
            ("L", x) => head.0 -= x.parse::<u32>().unwrap(),
            ("U", x) => head.1 += x.parse::<u32>().unwrap(),
            ("D", x) => head.1 -= x.parse::<u32>().unwrap(),
             _       => panic!("PANKID")
        }
        println!("{line} {:?}", head);
    });
    println!("Hello, world!");
}

fn movee(head: &mut position, tail: &mut position, dir: &position, n: u32) {
    for i in 0..n {
        *head = position(head.0 + dir.0, head.0 + dir.0);
        move_tail(tail, head);
    }
}

fn move_tail(tail: &mut position, head: &position) {
    if (tail.0 - head.0) > 1 {
        tail.0 = if tail.0 > head.0 { head.0+1 } else { head.0-1 }
    } else if (tail.0 - head.0) > 1 {
        tail.0 = if tail.0 > head.0 { head.0+1 } else { head.0-1 }
    }
}
