use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Allergen<'a>(&'a str);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Ingredient<'a>(&'a str);

#[derive(Debug, PartialEq, Eq)]
struct Food<'a> {
    ingredients: HashSet<Ingredient<'a>>,
    allergens: HashSet<Allergen<'a>>,
}

impl<'a> Food<'a> {
    fn new(input: &'a str) -> Food {
        let mut tokens_it = input.split_ascii_whitespace();
        let mut ingredients = HashSet::<Ingredient>::new();
        loop {
            let token = tokens_it.next().unwrap();
            if token.starts_with('(') {
                // skip "(contains" and move on to allergens
                break;
            }
            ingredients.insert(Ingredient(token));
        }

        let mut allergens = HashSet::<Allergen>::new();
        for mut token in tokens_it {
            token = token.strip_suffix(',').or(Some(token)).unwrap();
            token = token.strip_suffix(')').or(Some(token)).unwrap();
            allergens.insert(Allergen(token));
        }
        Food {
            ingredients,
            allergens,
        }
    }
}

fn parse_foods(input: &str) -> Vec<Food> {
    let mut foods = Vec::<Food>::new();
    for line in input.lines() {
        foods.push(Food::new(line));
    }
    foods
}

fn id_unsafe_ingredients<'a>(
    foods: &'a [Food],
) -> HashMap<Ingredient<'a>, Allergen<'a>> {
    let mut allergens2ingredients =
        HashMap::<Allergen, HashSet<Ingredient>>::new();
    for food in foods {
        for allergen in &food.allergens {
            if let Some(ingredients) = allergens2ingredients.get_mut(&allergen)
            {
                *ingredients = ingredients
                    .intersection(&food.ingredients)
                    .copied()
                    .collect();
            } else {
                allergens2ingredients
                    .insert(*allergen, food.ingredients.clone());
            }
        }
    }
    //println!("allergens2ing: {:?}", allergens2ingredients);
    let mut idd_allergens = HashSet::<Allergen>::new();
    loop {
        let mut new_idd_ingredients = HashSet::<Ingredient>::new();

        // Identify allergens with only 1 possible ingredient
        for (allergen, ings) in &allergens2ingredients {
            if ings.len() == 1 && !idd_allergens.contains(allergen) {
                let ing = ings.iter().next().unwrap();
                idd_allergens.insert(*allergen);
                new_idd_ingredients.insert(*ing);
            }
        }

        if new_idd_ingredients.is_empty() {
            break;
        }

        // Remove ID'd ingredients
        for (_, ings) in allergens2ingredients.iter_mut() {
            if ings.len() == 1 {
                // This is a known mapping.
                continue;
            }
            *ings = ings.difference(&new_idd_ingredients).copied().collect();
        }
    }

    let mut res = HashMap::new();
    for (allergen, mut ingredients) in allergens2ingredients {
        assert_eq!(ingredients.len(), 1);
        let ing = ingredients.drain().next().unwrap();
        res.insert(ing, allergen);
    }
    res
}

fn id_safe_ingredients<'a, It: Iterator<Item = &'a Ingredient<'a>>>(
    foods: &'a [Food],
    unsafe_ingredients: It,
) -> HashSet<Ingredient<'a>> {
    // Collect all ingredients
    let mut res = HashSet::<Ingredient>::new();
    for food in foods {
        for ing in &food.ingredients {
            res.insert(*ing);
        }
    }

    // Remove unsafe ingredients
    for ing in unsafe_ingredients {
        res.remove(ing);
    }
    res
}

fn count_ingredients(foods: &[Food], ingredients: &HashSet<Ingredient>) -> u64 {
    foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|i| ingredients.contains(i))
                .count() as u64
        })
        .sum()
}

fn encode_unsafe_ingredients<'a>(
    unsafe_ingredients: &'a HashMap<Ingredient<'a>, Allergen<'a>>,
) -> String {
    let mut pairs = unsafe_ingredients.iter().collect::<Vec<_>>();
    pairs.sort_by(|lhs, rhs| lhs.1 .0.cmp(rhs.1 .0));
    pairs
        .iter()
        .map(|(i, _)| i.0)
        .collect::<Vec<_>>()
        .as_slice()
        .join(",")
}

pub fn part1(input: &str) -> u64 {
    let foods = parse_foods(input);
    let unsafe_ingredients = id_unsafe_ingredients(&foods);
    let safe_ingredients =
        id_safe_ingredients(&foods, unsafe_ingredients.keys());
    count_ingredients(&foods, &safe_ingredients)
}

pub fn part2(input: &str) -> String {
    let foods = parse_foods(input);
    let unsafe_ingredients = id_unsafe_ingredients(&foods);
    encode_unsafe_ingredients(&unsafe_ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_example() {
        let foods = parse_foods(INPUT);
        assert_eq!(
            foods,
            [
                (
                    vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                    vec!["dairy", "fish"]
                ),
                (vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"], vec!["dairy"]),
                (vec!["sqjhc", "fvjkl"], vec!["soy"]),
                (vec!["sqjhc", "mxmxvkd", "sbzzf"], vec!["fish"])
            ]
            .iter()
            .map(|(ingredients, allergens)| Food {
                ingredients: ingredients
                    .iter()
                    .map(|s| Ingredient(s))
                    .collect(),
                allergens: allergens.iter().map(|s| Allergen(s)).collect()
            })
            .collect::<Vec<_>>()
        );

        let unsafe_ingredients = id_unsafe_ingredients(&foods);
        assert_eq!(
            unsafe_ingredients,
            [("mxmxvkd", "dairy"), ("sqjhc", "fish"), ("fvjkl", "soy")]
                .iter()
                .map(|(i, a)| (Ingredient(i), Allergen(a)))
                .collect::<HashMap<_, _>>()
        );

        let safe_ingredients =
            id_safe_ingredients(&foods, unsafe_ingredients.keys());
        assert_eq!(
            safe_ingredients,
            ["kfcds", "nhms", "sbzzf", "trh"]
                .iter()
                .map(|s| Ingredient(s))
                .collect::<HashSet<_>>()
        );

        assert_eq!(count_ingredients(&foods, &safe_ingredients), 5);

        // part 1 e2e
        assert_eq!(part1(INPUT), 5);

        assert_eq!(
            &encode_unsafe_ingredients(&unsafe_ingredients),
            "mxmxvkd,sqjhc,fvjkl"
        );

        // part 2 e2e
        assert_eq!(&part2(INPUT), "mxmxvkd,sqjhc,fvjkl");
    }
}
