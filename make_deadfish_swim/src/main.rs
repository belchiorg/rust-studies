fn last_digit(list: &[u64]) -> u64 {
    let len = list.len();

    let mut x = list[0] % 10;

    for i in 1..len {
        if list[i] == 0 { return 1; }
        let m = list[i].to_string().chars().fold(0, |a,x| (a*10 + x.to_digit(10).unwrap()) % 4);
        let exp = if m == 0 { 4 } else { m };
        x = (x.pow(exp) % 10) as u64;
    }

    return x
}

fn main() {
    println!("{}",last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"));
}