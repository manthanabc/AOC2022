use std::cell::RefCell;
use num::integer::lcm;

#[derive(Debug, Clone)]
struct Mankie {
    index: u32,
    items: RefCell<Vec<u128>>,
    thrown_items: RefCell<Vec<u32>>,
    opration: String,
    test: u32,
    throwto: (usize, usize), // true, false
    count: u128,
}

impl Mankie {
    fn new() -> Self {
        Mankie {
            index: 59,
            items: RefCell::new(Vec::new()),
            thrown_items: RefCell::new(Vec::new()),
            opration: String::new(),
            test: 2,
            throwto: (0, 0),
            count: 0,
        }
    }
}

fn doo(item: u128, op: &str, item2: u128) -> u128 {
    match op.trim() {
        "*" => item * item2,
        "+" => item + item2,
        _ => panic!("bite me"),
    }
}

fn main() {
    let inputs = include_str!("../input.prod");

    let mut monkies: RefCell<Vec<Mankie>> = RefCell::new(Vec::new());

    // parse da monkies
    inputs.split("\n\n").for_each(|f| {
        let mut monkie = Mankie::new();
        f.lines().for_each(|l| {
            match l.trim().split(" ").collect::<Vec<&str>>()[..] {
                ["Monkey", x] => monkie.index = x[..x.len() - 1].parse().unwrap(),
                ["Starting", "items:", ..]       => l[17..].split(", ").for_each(|t| { monkie.items.borrow_mut().push(t.trim().parse().unwrap());}),
                ["Operation:", "new", _] => todo!)_ 
                _ => {
                    if l.contains("Operation") {
                        let op = l.split_once("Operation: new =").unwrap().1.trim();
                        monkie.opration = op.to_string();

                    } else if l.contains("Test") {
                        monkie.test = l.split_once("divisible by ").unwrap().1.parse().unwrap();
                    } else if l.contains("If") {
                        if l.contains("true") {
                            println!("{:?}", l.rsplit_once(" ").unwrap());
                            monkie.throwto.0 =
                                l.rsplit_once(" ").unwrap().1.trim().parse().unwrap();
                        }
                        if l.contains("false") {
                            monkie.throwto.1 =
                                l.rsplit_once(" ").unwrap().1.trim().parse().unwrap();
                        }
                    }
                }
            }
        });
        monkies.borrow_mut().push(monkie);
        println!("{}", f);
    });
    let factor = monkies.borrow().iter().map(|m| m.test).fold(1, lcm);
    for i in 0..10000 {
        let lenth = monkies.borrow().len() ;
        for j in 0..lenth{
            {
                let mut monkie = monkies.borrow()[j].clone();
                while monkie.items.borrow().len() > 0 {
                    let mut item = monkie.items.borrow_mut().remove(0);
                    item = match monkie.opration.trim().split(" ").collect::<Vec<_>>()[..] {
                        ["old", o, "old"] => doo(item, o, item),
                        ["old", o, x] => doo(item, o, x.parse().unwrap()),
                        [x, o, "old"] => doo(x.parse().unwrap(), o, item),
                        _ => panic!("FKSDJFKSJD"),
                    };
                    monkie.count += 1;
                    item = item / 1% factor as u128;
            if item  %monkie.test as u128 == 0 {
                monkies.borrow_mut()[monkie.throwto.0]
                    .items
                    .borrow_mut()
                    .push(item);
            } else {
                // item = item% monkie.test as u128; 
                monkies.borrow_mut()[monkie.throwto.1]
                    .items
                    .borrow_mut()
                    .push(item);
            }
                }
                monkies.borrow_mut()[j] = monkie;
            }
        }
    }
    let mut ans = monkies.borrow().iter().map(|g| g.count).collect::<Vec<u128>>();
    ans.sort();
    println!("{:?}", ans[ans.len()-1]*ans[ans.len()-2]);
}
