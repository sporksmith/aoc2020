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

fn id_safe_ingredients<'a>(foods: &'a [Food]) -> HashSet<Ingredient<'a>> {
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

    // Collect all ingredients
    let mut res = HashSet::<Ingredient>::new();
    for food in foods {
        for ing in &food.ingredients {
            res.insert(*ing);
        }
    }

    // Remove those still under suspicion
    for ingredients in allergens2ingredients.values() {
        for ing in ingredients {
            res.remove(ing);
        }
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

pub fn part1(input: &str) -> u64 {
    let foods = parse_foods(input);
    let safe_ingredients = id_safe_ingredients(&foods);
    count_ingredients(&foods, &safe_ingredients)
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

        let safe_ingredients = id_safe_ingredients(&foods);
        assert_eq!(
            safe_ingredients,
            ["kfcds", "nhms", "sbzzf", "trh"]
                .iter()
                .map(|s| Ingredient(s))
                .collect::<HashSet<_>>()
        );

        assert_eq!(count_ingredients(&foods, &safe_ingredients), 5);

        // e2e
        assert_eq!(part1(INPUT), 5);
    }
}
