fn main() {
    let max = 1500000;
    let mut sums = vec![0; max];
    for i in 1..max {
        // if i 
        for j in (i..=50*i).step_by(i) {
            if j < max {
                sums[j]+=i;
            }
        }
    }
    // println!("{:?}", sums);
    println!("{}", sums.iter().enumerate().find(|(_, n)| **n > 3090909).unwrap().0);
}
