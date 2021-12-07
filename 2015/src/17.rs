fn main() {
    let containers = vec![11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3];
    let mut nmatches = 0;
    let mut matches = vec![0isize; 21];

    for i in 1..2isize.pow(containers.len() as u32) {
        let ters: Vec<isize> = format!("{:020b}", i).chars().enumerate()
            .filter_map(|(i, c)| if c == '1' {Some(containers[i])} else {None}).collect();
        let sum = ters.iter().sum::<isize>();
        if sum == 150 { matches[ters.len()]+=1; nmatches+=1; }
    }
    println!("{}", nmatches);
    println!("{}", matches.iter().filter_map(|n| if *n == 0 {None} else {Some(n)}).next().unwrap());
}