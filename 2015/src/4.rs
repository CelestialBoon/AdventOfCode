use regex::Regex;

fn main() {
    let seed = "yzbqklnj";
    let rx = Regex::new(r"^000000").unwrap();
    let mut num = 0;
    loop {
        num+=1;
        let hash = format!("{:x}", md5::compute(seed.to_string() + &num.to_string()));
        // println!("{}", seed);
        if rx.is_match(&hash) {
            println!("{}", num);
            break;
        }
    }
}