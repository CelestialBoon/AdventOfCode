use std::collections::HashSet;
use std::cmp::max;

fn main() {
    //p1 solution is just triangular sum of -yrange.0 -1 (-100 -> 4950)
    let xrange = (144, 178);
    let yrange = (-100, -76);
    
    let mut possibletraj = 0;
    let step = |x, y, xv, yv| {
        (x+xv, y+yv, max(0, xv-1), yv-1)
    };
    for ixv in 0..=xrange.1+1 {
       for iyv in yrange.0..300 {
            let (mut x, mut y, mut xv, mut yv) = (0,0,ixv,iyv);
            while y > yrange.0 {
                let news = step(x, y, xv, yv);
                x = news.0; y = news.1; xv = news.2; yv = news.3;
                if clamp(xrange.0, x, xrange.1) && clamp(yrange.0, y, yrange.1) {
                    possibletraj +=1;
                    break;
                }
            }
        }
    }
    println!("{}", possibletraj);
}

fn clamp(x1:isize, x:isize, x2:isize) -> bool {
    x >= x1 && x <= x2
}