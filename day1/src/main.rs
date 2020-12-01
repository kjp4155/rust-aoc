use std::io;

fn solve(v: &[i32], i: usize, j: usize, count: u32, target_sum: i32) -> i32 {
    if count == 0 {
        if target_sum == 0 { 1 } else { 0 }
    }
    else {
        for idx in i..j {
            if v[idx] <= target_sum {
                let partial_mul = solve(&v, idx + 1, j, count - 1, target_sum - v[idx]);
                if partial_mul != 0 {
                    return v[idx] * partial_mul;
                }
            }
        }
        0
    }
}

fn main() {

    let mut v = Vec::new();

    loop {
        let mut x = String::new();
        
        io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

        match x.trim().parse() {
            Ok(num) => v.push(num),
            Err(_) => break,
        };
    }

    println!("Phase 1 answer: {}", solve(v.as_slice(), 0, v.len(), 2, 2020));
    println!("Phase 2 answer: {}", solve(v.as_slice(), 0, v.len(), 3, 2020));
}
