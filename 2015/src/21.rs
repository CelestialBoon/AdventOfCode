use std::collections::HashMap;
use std::cmp;

fn main() {
    let (bosshp, bossdamage, bossarmor) = (104, 8, 1);

    let mut weapons:HashMap<isize, isize> = HashMap::new();
    weapons.insert(4, 8);
    weapons.insert(5, 10);
    weapons.insert(6, 25);
    weapons.insert(7, 40);
    weapons.insert(8, 74);

    let mut armor:HashMap<isize, isize> = HashMap::new();
    armor.insert(0, 0);
    armor.insert(1, 13);
    armor.insert(2, 31);
    armor.insert(3, 53);
    armor.insert(4, 75);
    armor.insert(5, 102);

    let mut damagerings:HashMap<isize, isize> = HashMap::new();
    damagerings.insert(0, 0);
    damagerings.insert(1, 25);
    damagerings.insert(2, 50);
    damagerings.insert(3, 75);
    damagerings.insert(3, 100);
    damagerings.insert(4, 125);
    damagerings.insert(5, 150);

    let mut armorrings:HashMap<isize, isize> = HashMap::new();
    armorrings.insert(0, 0);
    armorrings.insert(1, 20);
    armorrings.insert(2, 40);
    armorrings.insert(3, 60);
    armorrings.insert(3, 80);
    armorrings.insert(4, 100);
    armorrings.insert(5, 120);

    let mut minprice = 500;
    let mut maxprice = 0;
    for (wd, wp) in &weapons {
        for (ad, ap) in &armor {
            for (drd, drp) in &damagerings {
                for (ara, arp) in &armorrings {
                    let price = wp + ap + drp + arp;
                    let hitsgiven = {
                        let actualdamage = cmp::max(wd + drd - bossarmor, 1);
                        bosshp / actualdamage + if bosshp % actualdamage == 0 {0} else {1}
                    };
                    let hitstaken = {
                        let actualdamage = cmp::max(bossdamage - ad - ara, 1);
                        100 / actualdamage + if 100 % actualdamage == 0 {0} else {1}
                    };
                    if price < minprice && hitsgiven <= hitstaken {
                        minprice = price;
                        // println!("{} {} {} {}", wd, ad, drd, ara);
                    };
                    if price > maxprice && hitsgiven > hitstaken {
                        maxprice = price;
                        println!("{} {} {} {} {}", wd, ad, drd, ara, price);
                    };
                }
            }
        }
    }
    println!("{}, {}", minprice, maxprice);
    
}
