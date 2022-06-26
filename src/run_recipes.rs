use crate::Groceries;
use clap::ArgMatches;

pub fn run(sync_matches: &ArgMatches) -> Result<(), crate::ReadError> {
    let path = sync_matches.get_one::<String>("path").unwrap();

    match sync_matches.subcommand() {
        Some(("add", s_matches)) => recipes_add(s_matches, path)?,
        Some(("add-ingredient", s_matches)) => recipes_add_ingredient(s_matches, path)?,
        Some(("delete-ingredient", s_matches)) => ingredient_delete(s_matches, path)?,
        Some(("delete-recipe", s_matches)) => recipes_delete(s_matches, path)?,
        Some(("edit", s_matches)) => recipe_edit(s_matches, path)?,
        Some(("print", s_matches)) => recipes_print(path, recipe(s_matches))?,
        _ => unreachable!(),
    }

    Ok(())
}

fn recipe(sync_matches: &ArgMatches) -> Option<String> {
    if sync_matches.contains_id("recipe") {
        let elems: Vec<_> = sync_matches
            .values_of("recipe")
            .expect("recipe input is required")
            .collect();
        let item = elems.join(" ");
        Some(item)
    } else {
        None
    }
}

fn recipes_add_ingredient(s_matches: &ArgMatches, _path: &str) -> Result<(), crate::ReadError> {
    let elems: Vec<_> = s_matches
        .values_of("ingredient")
        .expect("input is required")
        .collect();
    let item = elems.join(" ");

    let elems: Vec<_> = s_matches
        .values_of("recipe")
        .expect("input is required")
        .collect();
    let recipe = elems.join(" ");
    eprintln!("Recipe to be edited: {recipe}");
    eprintln!("Ingredient to be added: {item}");
    eprintln!("this feature is only implemented thus far");
    Ok(())
}

fn ingredient_delete(s_matches: &ArgMatches, _path: &str) -> Result<(), crate::ReadError> {
    let elems: Vec<_> = s_matches
        .values_of("ingredient")
        .expect("input is required")
        .collect();
    let item = elems.join(" ");

    let elems: Vec<_> = s_matches
        .values_of("recipe")
        .expect("input is required")
        .collect();
    let recipe = elems.join(" ");
    eprintln!("Recipe to be edited: {recipe}");
    eprintln!("Ingredient to be deleted: {item}");
    eprintln!("this feature is only implemented thus far");
    Ok(())
}

fn recipe_edit(s_matches: &ArgMatches, _path: &str) -> Result<(), crate::ReadError> {
    let elems: Vec<_> = s_matches
        .values_of("ingredient")
        .expect("input is required")
        .collect();
    let item = elems.join(" ");

    let elems: Vec<_> = s_matches
        .values_of("recipe")
        .expect("input is required")
        .collect();
    let recipe = elems.join(" ");

    let elems: Vec<_> = s_matches
        .values_of("edit")
        .expect("input is required")
        .collect();
    let edit = elems.join(" ");
    eprintln!("Recipe to be edited: {recipe}");
    eprintln!("Ingredient to be edited: {item}");
    eprintln!("Ingredient to be re-edited as '{edit}'");
    eprintln!("this feature is only implemented thus far");
    Ok(())
}

fn recipes_add(s_matches: &ArgMatches, path: &str) -> Result<(), crate::ReadError> {
    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
        .collect();
    let n = name_elems.join(" ");
    eprintln!("Recipe: {}", n);
    let ingredient_vec: Vec<_> = s_matches
        .values_of("ingredients")
        .expect("ingredients required")
        .collect();
    let i = ingredient_vec.join(", ");
    eprintln!("Ingredients: {}", i);
    let mut g = Groceries::from_path(path)?;
    eprintln!("before adding: {:?}", g.recipes);
    g.add_recipe(&n, &i)?;
    eprintln!("after adding: {:?}", g.recipes);
    g.save(path)?;
    Ok(())
}

fn recipes_delete(s_matches: &ArgMatches, path: &str) -> Result<(), crate::ReadError> {
    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
        .collect();
    let n = name_elems.join(" ");
    eprintln!("Recipe: {}", n);
    let mut g = Groceries::from_path(path)?;
    eprintln!("before deleting: {:?}", g.recipes);
    g.delete_recipe(&n)?;
    eprintln!("after: {:?}", g.recipes);
    g.save(path)?;
    Ok(())
}

fn recipes_print(path: &str, recipe: Option<String>) -> Result<(), crate::ReadError> {
    let groceries = Groceries::from_path(path)?;

    if recipe.is_some() {
        eprintln!();
        groceries.print_recipe(&recipe.unwrap())?;
        eprintln!();
    } else {
        eprintln!();
        eprintln!("Here are our recipes:");
        groceries.print_recipes();
        eprintln!();
    }
    Ok(())
}
