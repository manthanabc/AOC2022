use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct position(i32, i32);

fn main() {
    let input = include_str!("../input.prod");
    let mut nodes = vec![position(0, 0); 10];
    let mut map = HashSet::new();

    input.lines().for_each(|line| {
        let (mut dir, x) = match line.split_once(" ").unwrap() {
            ("R", x) => (position( 1, 0 ), x),
            ("L", x) => (position(-1, 0 ), x),
            ("U", x) => (position( 0, 1 ), x),
            ("D", x) => (position( 0, -1), x),
             _       => panic!("PANKID")
        };
        movee(&mut nodes, &mut dir, x.parse().unwrap(), &mut map);
    });
    println!("Answer {:?}", map.len());
}

fn movee(nodes: &mut Vec<position>, dir: &position, n: u32, map: &mut HashSet<position>) {
    for _ in 0..n {
        nodes[0] = position(nodes[0].0 + dir.0, nodes[0].1 + dir.1);
        for i in 1..nodes.len() {
            let nd = &nodes[i-1].clone();
            move_tail(&mut nodes[i], nd);
        }
        if !map.contains(&nodes[nodes.len()-1]) {
            map.insert(nodes[nodes.len()-1].clone());
        }
    }
}

fn move_tail(tail: &mut position, head: &position) {
    if (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1 {
        *tail = position(tail.0 + (head.0-tail.0).signum(), tail.1 + (head.1-tail.1).signum());
    }
}
