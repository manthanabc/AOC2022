fn main() {
    let input: Vec<&str> = include_str!("../input.prod").lines().collect();
    let lines = input.iter().enumerate();
    let mut total = 0;
    let mut best_score=0;
    for (i, inp) in lines {
        'm: for (j, ch) in inp.chars().enumerate() {
            
            let (mut right, mut left, mut top, mut downn) = (0, 0, 0, 0);
            // println!("trying {ch}");
            //visible from sides
            let (mut start, mut end) = inp.split_at(j);
            let end = &end[1..];
            // println!("{start} {end}");
            if start.len() > 0 {
            for i in 0..start.len() {
                if start.as_bytes()[start.len()-1-i] as char >= ch {
                    right+=1;
                    break;
                }
                right += 1;
            }} 
            if end.len() > 1 {
            for i in 0..end.len() {
                if end.as_bytes()[i] as char >= ch {
                    left+=1;
                    break;
                }
                left+=1;
            }}
            
            let mut side = false;
            let (mut start, mut end) = (start.chars(), end.chars());
            if !start.any(|cfh| cfh >= ch) || !end.any(|cfh| cfh >= ch) {
                // total += 1;
                // println!("visible");
                side = true;
            } 

            //visible from top
            let mut up = true;
            for x in 0..i {
               if input[i-1-x].as_bytes()[j] as char >= ch {
                    // println!(" t {:?} blocks", input[i-1-x].as_bytes()[j] as char);
                    top +=1;
                    up = false;
                    break;
                }
               top+=1;
            } 
            //visible from top
            let mut down = true;
            for x in i+1..input.len() {
               if input[x].as_bytes()[j] as char >= ch {
                    // println!("b {:?} blocks", input[x].as_bytes()[j] as char);
                    downn +=1;
                    down = false;
                    break;
                }
               downn+=1;
            }
            // println!("{top}*{downn}*{right}*{left}");
            if top*downn*right*left > best_score {
                best_score = top*downn*right*left;
            }
            if up || down || side{
                total += 1;
            }
        }
    }
    println!("{total} {best_score}");
}
