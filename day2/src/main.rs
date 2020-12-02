use std::io;

fn count_char (s: &str, c: char) -> u32 {
    let mut cnt: u32 = 0;
    for sc in s.chars() {
        if sc == c {
            cnt += 1;
        }
    }
    return cnt;
}

fn solve_line () -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Readline failed");
    let split = buffer.split(" ");
    let vec: Vec<&str> = split.collect();

    let vec2: Vec<&str> = vec[0].split("-").collect();

    let idx: usize = 0;

    let target_char = vec[1].chars().nth(0).unwrap();
    let passwd: &str = vec[2];
    let low: u32 = vec2[0].parse::<u32>().unwrap();
    let high: u32 = vec2[1].parse::<u32>().unwrap();

    // let cnt = count_char(passwd, target_char);
    // if low <= cnt && cnt <= high {1} else {0} 

    let lowc = passwd.chars().nth((low-1) as usize).unwrap();
    let highc = passwd.chars().nth((high-1) as usize).unwrap();

    let mut cnt: u32 = 0;
    if lowc == target_char {cnt += 1;}
    if highc == target_char {cnt += 1;}

    println!("{} {} {}", lowc, highc, cnt);
    if cnt == 1 {1} else {0}
}

fn main() {
    let mut answer: u32 = 0;
    loop {
        answer += solve_line();
        println!("{}", answer);
    }
}
