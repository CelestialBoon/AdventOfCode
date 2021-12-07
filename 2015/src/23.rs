fn main() {
    let mut a:i64 = 60975;
    let mut b = 0;
    loop {
        if a == 1 {
            println!("{}", b);
            break;
        } else if a % 2 == 0 {
            a = a/2; 
        } else {
            a = a*3 + 1;
        }
        b+=1;
    } 
 }