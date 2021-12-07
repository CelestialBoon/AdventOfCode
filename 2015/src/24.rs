use itertools::Itertools;

fn main () {    
    let packages = vec![1, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
    let third = packages.iter().sum::<isize>() / 3;
    let fourth = packages.iter().sum::<isize>() / 4;
    
    println!("{}", (1..29).filter_map(|i| packages.iter().combinations(i)
        .filter(|b| b.iter().map(|n| **n).sum::<isize>() == third)
        .map(|b| b.iter().map(|n| **n).product::<isize>())
        .min()).next().unwrap());

    println!("{}", (1..29).filter_map(|i| packages.iter().combinations(i)
        .filter(|b| b.iter().map(|n| **n).sum::<isize>() == fourth)
        .map(|b| b.iter().map(|n| **n).product::<isize>())
        .min()).next().unwrap()); 
}