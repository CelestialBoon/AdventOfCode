use std::cmp;

fn main() {
    
    struct Ingredient {
        name:String,
        capacity:isize,
        durability:isize,
        flavor:isize,
        texture:isize,
        calories:isize
    }

    impl Ingredient {
        pub fn new(name:String, capacity:isize, durability:isize, flavor:isize, texture:isize, calories:isize) -> Self {
            Self { name:name,
                capacity: capacity,
                durability: durability,
                flavor: flavor,
                texture: texture,
                calories: calories
            }
        }
    }

    let ingredients = vec![
        Ingredient::new("Sprinkles".to_string(),    2, 0, -2, 0, 3),
        Ingredient::new("Butterscotch".to_string(), 0, 5, -3, 0, 3),
        Ingredient::new("Chocolate".to_string(),    0, 0, 5, -1, 8),
        Ingredient::new("Candy".to_string(),        0, -1, 0, 5, 8),
        ];

    let mut maxscore = 0;    

    for a in 1..=97 {
        for b in 1..=98-a {
            for c in 1..=99-a-b {
                let d = 100-a-b-c;
                let ratio = vec![a, b, c, d];
                let capacity = ratio.iter().zip(ingredients.iter()).map(|(r, ingr)| ingr.capacity * r).sum::<isize>();
                let durability = ratio.iter().zip(ingredients.iter()).map(|(r, ingr)| ingr.durability * r).sum::<isize>();
                let flavor = ratio.iter().zip(ingredients.iter()).map(|(r, ingr)| ingr.flavor * r).sum::<isize>();
                let texture = ratio.iter().zip(ingredients.iter()).map(|(r, ingr)| ingr.texture * r).sum::<isize>();
                let calories = ratio.iter().zip(ingredients.iter()).map(|(r, ingr)| ingr.calories * r).sum::<isize>();

                if calories == 500 && capacity > 0 && durability > 0 && flavor > 0 && texture > 0 {
                    maxscore = cmp::max(maxscore, capacity * durability * flavor * texture);
                }
            }
        }
    }

    println!("{}", maxscore);
}