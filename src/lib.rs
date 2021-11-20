use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("grusterylist")
        .override_help(
            "\n\
	     grusterylist 0.1.0\n\
	     Makes grocery lists in Rust\n\
	     (C) https://github.com/suchapalaver/\n\n\
	     Usage: cargo run -- <opts>\n\n\
	     OPTIONS:\n    \
	     -h, --help       Print help information\n    \
	     -V, --version    Print version information\n    \
	     \n\
	     SUBCOMMANDS:\n    \
	     g     Add groceries to groceries library\n    \
	     r     Add recipes to recipes library\n    \
	     l     Make a shopping list\n\
	     \n\
	     EXAMPLE:\n    \
	     cargo run -- l",
        )
        .arg(Arg::new("subcommands").required(true).max_values(1))
        .get_matches();

    match matches.value_of("subcommands").unwrap() {
        "l" => Ok(make_list()?),
        "g" => Ok(run_groceries()?),
        "r" => Ok(run_recipes()?),
        &_ => Err("Invalid command.\n\
		   For help, try:\n\
		   cargo run -- -h"
            .into()),
    }
}

use crate::data::*;

// used to serialize and deserialize a
// database of groceries we buy
// organized by kitchen storage section
pub mod data {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Groceries {
        pub sections: GroceriesSections,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GroceriesSections(pub Vec<GroceriesSection>);

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GroceriesSection {
        pub name: GroceriesSectionName,
        pub items: GroceriesItems,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GroceriesSectionName(String);

    impl fmt::Display for GroceriesSectionName {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct GroceriesItems(pub Vec<GroceriesItem>);

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct GroceriesItem(pub String);

    // to serialize and deserialize a database of recipes
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Recipes {
        pub library: RecipeLib,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RecipeLib(pub Vec<Recipe>);

    impl RecipeLib {
        pub fn new() -> Result<RecipeLib, Box<dyn Error>> {
            let library: Vec<Recipe> = Vec::new();
            Ok(RecipeLib(library))
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Recipe {
        pub name: RecipeName,
        pub items: RecipeItems,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct RecipeName(pub String);

    impl fmt::Display for RecipeName {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RecipeItems(pub Vec<GroceriesItem>);

    // used to serialize and deserialize the
    // most recently saved list or to create a
    // new grocery list that can be saved as JSON
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ShoppingList {
        pub recipes_msg: RecipesOnListMsg,
        pub recipes: RecipeNameList,
        pub checklist_msg: ChecklistMsg,
        pub checklist: ChecklistItems,
        pub items_msg: ItemsOnListMsg,
        pub items: ShoppingListItems,
    }

    // This is what we want to happen each time we create
    // a new shopping list
    impl ShoppingList {
        pub fn new() -> Result<ShoppingList, Box<dyn Error>> {
            Ok(ShoppingList {
                recipes_msg: RecipesOnListMsg::new()?,
                recipes: RecipeNameList::new()?,
                checklist_msg: ChecklistMsg::new()?,
                checklist: ChecklistItems::new()?,
                items_msg: ItemsOnListMsg::new()?,
                items: ShoppingListItems::new()?,
            })
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct RecipesOnListMsg(pub String);

    impl RecipesOnListMsg {
        fn new() -> Result<RecipesOnListMsg, Box<dyn Error>> {
            Ok(RecipesOnListMsg("\n\
				 We're making ...".to_string()))
        }
    }

    impl fmt::Display for RecipesOnListMsg {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct RecipeNameList(pub Vec<RecipeName>);

    impl RecipeNameList {
        fn new() -> Result<RecipeNameList, Box<dyn Error>> {
            Ok(RecipeNameList(Vec::new()))
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ChecklistMsg(pub String);

    impl ChecklistMsg {
        fn new() -> Result<ChecklistMsg, Box<dyn Error>> {
            Ok(ChecklistMsg("\n\
			     Check if we need ...".to_string()))
        }
    }

    impl fmt::Display for ChecklistMsg {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ChecklistItems(pub Vec<GroceriesItem>);

    impl ChecklistItems {
        fn new() -> Result<ChecklistItems, Box<dyn Error>> {
            Ok(ChecklistItems(Vec::new()))
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ItemsOnListMsg(pub String);

    impl ItemsOnListMsg {
        fn new() -> Result<ItemsOnListMsg, Box<dyn Error>> {
            Ok(ItemsOnListMsg("\n\
			       We need ...".to_string()))
        }
    }

    impl fmt::Display for ItemsOnListMsg {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ShoppingListItems(pub Vec<GroceriesItem>);

    impl ShoppingListItems {
        fn new() -> Result<ShoppingListItems, Box<dyn Error>> {
            Ok(ShoppingListItems(Vec::new()))
        }
    }
}

use crate::errors::*;

// Customized handling of
// file reading errors
pub mod errors {
    use super::*;

    #[derive(Debug)]
    pub enum ReadError {
        DeserializingError(serde_json::Error),
        PathError(Box<dyn Error>),
    }

    // Yup, you can't just return some string as an error message
    impl fmt::Display for ReadError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ReadError::DeserializingError(e) => write!(
                    f,
                    "Error deserializing from JSON file:\n\
                     '{}'!\n\
		     Something's wrong with the JSON file?\n\
		     See the example json files in the \
		     grusterylist repository to see \
		     how things should look.\n",
                    e
                ),
                ReadError::PathError(e) => write!(
                    f,
                    "Error: '{}'!\n\
		     Make sure file with that path \
		     can be accessed by the \
		     present working directory",
                    e
                ),
            }
        }
    }

    // This is to make compatibility with the chain of Box<dyn Error> messaging
    impl Error for ReadError {
        fn description(&self) -> &str {
            match *self {
                ReadError::DeserializingError(_) => "Error deserializing from JSON file!",
                ReadError::PathError(_) => "File does not exist!",
            }
        }
    }
}

use crate::helpers::*;
mod helpers {
    use super::*;

    pub fn read_groceries<P: AsRef<Path> + Copy>(path: P) -> Result<Groceries, Box<dyn Error>> {
        let reader = read(path)?;

        let groceries = serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

        Ok(groceries)
    }

    pub fn read_recipes<P: AsRef<Path> + Copy>(path: P) -> Result<Recipes, Box<dyn Error>> {
        let reader = read(path)?;

        let recipes = serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

        Ok(recipes)
    }

    // Gets user input when it's 'y' or anything else
    pub fn prompt_for_y() -> Result<bool, Box<dyn Error>> {
        Ok("y" == input()?)
    }

    // Function for getting user input
    pub fn input() -> Result<String, Box<dyn Error>> {
        let _ = Write::flush(&mut stdout())?;

        let mut input = String::new();

        stdin().read_line(&mut input)?;

        let output = input.trim().to_string();

        // I was using the below to debug
        // the input function's behavior,
        // so left it in as a reminder;
        // got this from Rust in Action
        /*
            if cfg!(debug_assertions) {
            eprintln!("debug:\n\
            UNTRIMMED:\n\
            {:?}\n\
            TRIMMED:\n\
            {:?}",
            input, output);
        }
             */

        Ok(output)
    }

    // Input a list and return it having added a list of user input strings
    pub fn list_input(
        mut items_list: Vec<GroceriesItem>,
    ) -> Result<Vec<GroceriesItem>, Box<dyn Error>> {
        eprintln!(
            "Enter the items, \
	     separated by commas"
        );

        let input = input()?;

        let input_list: Vec<_> = input
            .split(',')
            .map(|item| item.trim().to_lowercase())
            .collect();

        input_list.iter().for_each(|item| {
            if !items_list.contains(&GroceriesItem(item.to_string())) {
                items_list.push(GroceriesItem(item.to_string()));
            }
        });

        Ok(items_list)
    }

    // Reads from a path into a buffer-reader
    pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path).map_err(|err_msg| ReadError::PathError(Box::from(err_msg)))?;

        let reader = BufReader::new(file);

        Ok(reader)
    }

    // Writes a String to a path
    pub fn write<P: AsRef<Path>>(path: P, object: String) -> Result<(), Box<dyn Error>> {
        let _ = fs::write(path, &object)?;
        Ok(())
    }
}

use crate::groceries::*;
mod groceries {
    use super::*;

    pub fn run_groceries() -> Result<(), Box<dyn Error>> {
        let _ = update_groceries()?;
        Ok(())
    }

    fn update_groceries() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add groceries to our library?\n\
	     --y\n\
	     --any other key to exit"
        );

        while prompt_for_y()? {
            let path = "groceries.json";

            let groceries = read_groceries(path).map_err(|e| {
                format!(
                    "Failed to read groceries file '{}':\n\
		     '{}'\n",
                    path, e
                )
            })?;

            let updated_groceries_sections = update_groceries_sections(groceries)?;

            let groceries = updated_groceries_sections;

            let json = serde_json::to_string(&groceries)?;

            write(path, json)?;

            eprintln!(
                "Add more groceries to our library?\n\
		 --y\n\
		 --any other key to exit"
            );
        }
        Ok(())
    }

    fn update_groceries_sections(
        groceries: Groceries,
    ) -> Result<Vec<GroceriesSection>, Box<dyn Error>> {
        let mut updated_groceries_sections = Vec::new();

        let groceries_sections = groceries.sections;

        for groceries_section in groceries_sections.0 {
            eprintln!(
                "Add to our {} section?\n\
		 --y\n\
		 --any other key to continue",
                groceries_section.name
            );

            if prompt_for_y()? {
                let items = list_input(groceries_section.items.0)?;

                updated_groceries_sections.push(GroceriesSection {
                    name: groceries_section.name,
                    items: GroceriesItems(items),
                });
            } else {
                updated_groceries_sections.push(GroceriesSection {
                    name: groceries_section.name,
                    items: groceries_section.items,
                });
            }
        }
        Ok(updated_groceries_sections)
    }
}

use crate::recipes::*;
mod recipes {
    use super::*;

    pub fn run_recipes() -> Result<(), Box<dyn Error>> {
        let _ = view_recipes()?;

        let _ = new_recipes()?;

        Ok(())
    }

    fn view_recipes() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "View the recipes we have \
	     in our library?\n\
	     --y\n\
	     --any other key to continue"
        );

        if prompt_for_y()? {
            print_recipes()?;
        }
        Ok(())
    }

    fn new_recipes() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add recipes to our library?\n\
	     --y\n\
	     --any other key to continue"
        );

        while prompt_for_y()? {
            let path = "recipes.json";

            let recipes = read_recipes(path).map_err(|e| {
                format!(
                    "Failed to read recipes file '{}':\n\
		     '{}'",
                    path, e
                )
            })?;

            let RecipeLib(mut updated) = recipes.library;

            let new_recipe = get_new_recipe()?;

            updated.push(new_recipe);

            let recipes = Recipes {
                library: RecipeLib(updated),
            };

            save_recipes(recipes)?;

            eprintln!(
                "Add more recipes to our library?\n\
		 --y\n\
		 --any other key to exit"
            );
        }
        Ok(())
    }

    // Gets a new recipe from user
    // and returns it as a Recipe
    fn get_new_recipe() -> Result<Recipe, Box<dyn Error>> {
        eprintln!("What's the recipe?");

        let name = input()?;

        let mut items = Vec::new();

        items = list_input(items)?;

        Ok(Recipe {
            name: RecipeName(name),
            items: RecipeItems(items),
        })
    }

    fn save_recipes(recipes: Recipes) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&recipes)?;

        write("recipes.json", json)?;

        Ok(())
    }

    fn print_recipes() -> Result<(), Box<dyn Error>> {
        let path = "recipes.json";

        let recipes = read_recipes(path).map_err(|e| {
            format!(
                "Failed to read recipes file '{}':\n\
		 '{}'",
                path, e
            )
        })?;

        eprintln!("Here are our recipes:");

        for recipe in recipes.library.0 {
            eprintln!("- {}", recipe.name.to_string());
        }
        eprintln!();

        Ok(())
    }
}

use crate::list::*;
mod list {
    use super::*;

    // Like run() for the shopping-list-making function in grusterylist
    pub fn make_list() -> Result<(), Box<dyn Error>> {
        // Open a saved or new list
        let mut shopping_list = get_saved_or_new_list()?;

        // view list if using saved list
        if !shopping_list.items.0.is_empty() {
            print_list()?;
        }

        // add recipes
        shopping_list = add_recipes_to_list(shopping_list)?;

        // add individual groceries
        shopping_list = add_groceries_to_list(shopping_list)?;

        // overwrite saved list with current list
        save_list(shopping_list)?;

        // view list
        print_list()?;

        Ok(())
    }

    // Prompt user whether to use a saved or new list and return their choice
    fn get_saved_or_new_list() -> Result<ShoppingList, Box<dyn Error>> {
        let mut shopping_list = ShoppingList::new()?;

        eprintln!(
            "\n\
	     Use saved list?\n\
	     --y\n\
	     --any other key for new list"
        );

        if prompt_for_y()? {
            let path = "list.json";

            shopping_list = read_list(path).map_err(|e| {
                format!(
                    "Failed to read list file '{}':\n\
		     '{}'",
                    path, e
                )
            })?;
        }
        Ok(shopping_list)
    }

    // Prints list
    fn print_list() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "\n\
	     Print out shopping list?\n\
	     --y\n\
	     --any other key to continue"
        );

        if prompt_for_y()? {
            let path = "list.json";

            let shopping_list = read_list(path).map_err(|e| {
                format!(
                    "Failed to read list file '{}':\n\
		     {}",
                    path, e
                )
            })?;

            // Avoid printing empty lists
            if !shopping_list.checklist.0.is_empty()
                && !shopping_list.recipes.0.is_empty()
                && !shopping_list.items.0.is_empty()
            {
                println!("Here's what we have:\n");
            }
            if !shopping_list.checklist.0.is_empty() {
                println!("{}", shopping_list.checklist_msg);

                shopping_list.checklist.0.iter().for_each(|item| {
                    println!("\t{}", item.0.to_lowercase());
                });
            }
            if !shopping_list.recipes.0.is_empty() {
                println!("{}", shopping_list.recipes_msg);

                shopping_list.recipes.0.iter().for_each(|recipe| {
                    println!("\t{}", recipe);
                });
            }
            if !shopping_list.items.0.is_empty() {
                println!("{}", shopping_list.items_msg);

                shopping_list.items.0.iter().for_each(|item| {
                    println!("\t{}", item.0.to_lowercase());
                });
            }
            // Print a new line at end of output
            println!();
        }
        Ok(())
    }

    // Open and deserialize a shopping list JSON file from given path
    fn read_list<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
        let reader = read(path)?;

        let shopping_list =
            serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

        Ok(shopping_list)
    }

    // Adds recipe ingredients to a shopping list
    fn add_recipes_to_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add recipe ingredients to our list?\n\
	     --y\n\
	     --any other key to continue"
        );
        while prompt_for_y()? {
            let path = "recipes.json";

            let recipes = read_recipes(path).map_err(|e| {
                format!(
                    "Failed to read recipes file '{}':\n\
		     '{}'",
                    path, e
                )
            })?;

            for recipe in recipes.library.0 {
                eprintln!(
                    "Shall we add ...\n\
		     {}?\n\
		     --y\n\
		     --s to skip to end of recipes\n\
		     --any other key for next recipe",
                    recipe.name
                );

                match input()?.as_str() {
                    "y" => shopping_list = add_recipe_to_list(shopping_list, recipe)?,
                    "s" => break,
                    &_ => continue,
                }
            }
            eprintln!(
                "Add any more recipe ingredients to our list?\n\
		 --y\n\
		 --any other key to continue"
            );
        }
        Ok(shopping_list)
    }

    // Adds ingredients of an individual recipe to a shopping list
    fn add_recipe_to_list(
        mut shopping_list: ShoppingList,
        recipe: Recipe,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        shopping_list.recipes.0.push(recipe.name);

        eprintln!(
            "Do we need ... ?\n\
	     --y\n\
	     --c to remind to check\n\
	     --a to add this and all remaining ingredients\n\
	     --any other key for next ingredient"
        );

        let recipe_items = recipe.items;

        for ingredient in &recipe_items.0 {
            eprintln!("{}?", ingredient.0.to_lowercase());

            match input()?.as_str() {
                "y" => shopping_list = add_ingredient_to_list(shopping_list, ingredient)?,
                "c" => shopping_list = add_ingredient_to_checklist(shopping_list, ingredient)?,
                "a" => {
                    shopping_list = add_all_ingredients_to_list(shopping_list, recipe_items.0)?;
                    break;
                }
                &_ => continue,
            }
        }
        Ok(shopping_list)
    }

    // Adds individual ingredients to a shopping list
    fn add_ingredient_to_list(
        mut shopping_list: ShoppingList,
        ingredient: &GroceriesItem,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        if !shopping_list
            .items
            .0
            .contains(&GroceriesItem(ingredient.0.to_lowercase()))
        {
            shopping_list
                .items
                .0
                .push(GroceriesItem(ingredient.0.to_lowercase()));
        }
        Ok(shopping_list)
    }

    // Adds all ingredients in a single recipe to list
    fn add_all_ingredients_to_list(
        mut shopping_list: ShoppingList,
        recipe_items: Vec<GroceriesItem>,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        for ingredient in recipe_items {
            // Avoid adding repeat items to list
            if !shopping_list
                .items
                .0
                .contains(&GroceriesItem(ingredient.0.to_lowercase()))
            {
                shopping_list.items.0.push(ingredient);
            }
        }
        Ok(shopping_list)
    }

    // Adds ingredients to checklist on shopping list
    fn add_ingredient_to_checklist(
        mut shopping_list: ShoppingList,
        ingredient: &GroceriesItem,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        shopping_list
            .checklist
            .0
            .push(GroceriesItem(ingredient.0.to_lowercase()));

        Ok(shopping_list)
    }

    // Adds groceries to list
    fn add_groceries_to_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add groceries to shopping list?\n\
	     --y\n\
	     --any other key to skip"
        );

        while prompt_for_y()? {
            let path = "groceries.json";

            let groceries = read_groceries(path).map_err(|e| {
                format!(
                    "Failed to read groceries file '{}':\n\
		     '{}'",
                    path, e
                )
            })?;

            shopping_list = add_grocery_sections_to_list(shopping_list, groceries)?;

            eprintln!(
                "Add more groceries to shopping list?\n\
		 --y\n\
		 --any other key to skip"
            );
        }
        Ok(shopping_list)
    }

    fn add_grocery_sections_to_list(
        mut shopping_list: ShoppingList,
        groceries: Groceries,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        let groceries_sections = groceries.sections;

        for groceries_section in groceries_sections.0 {
            eprintln!(
                "Do we need {}?\n\
		 --y\n\
		 --s to skip remaining sections\n\
		 --any other key to continue",
                &groceries_section.name.to_string().to_lowercase()
            );

            match input()?.as_str() {
                "y" => {
                    shopping_list = add_grocery_section_to_list(shopping_list, groceries_section)?
                }
                "s" => break,
                &_ => continue,
            }
        }
        Ok(shopping_list)
    }

    fn add_grocery_section_to_list(
        mut shopping_list: ShoppingList,
        groceries_section: GroceriesSection,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Do we need ...?\n\
	     --y\n\
	     --c to check later\n\
	     --s to skip to next section\n\
	     --any other key to continue"
        );

        for item in groceries_section.items.0 {
            // https://stackoverflow.com/questions/45624813/how-can-i-unpack-a-tuple-struct-like-i-would-a-classic-tuple/45624862
            // the .0. is indexing the String wrapped in the tuple struct
            if !shopping_list
                .items
                .0
                .contains(&GroceriesItem(item.0.to_lowercase()))
            {
                eprintln!("{}?", item.0.to_lowercase());

                match input()?.as_str() {
                    // unpack the tuple, mutate the contents,
                    // rewrap the changes in the tuple struct
                    "y" => shopping_list
                        .items
                        .0
                        .push(GroceriesItem(item.0.to_lowercase())),
                    "c" => shopping_list
                        .checklist
                        .0
                        .push(GroceriesItem(item.0.to_lowercase())),
                    // skip remaining sections
                    "s" => break,
                    &_ => continue,
                }
            }
        }
        Ok(shopping_list)
    }

    // Saves shopping list
    fn save_list(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Save current list?\n\
	     --y\n\
	     --any other key to continue"
        );

        if prompt_for_y()? {
            let json = serde_json::to_string(&shopping_list)?;
            // Put trace here
            write("list.json", json)?;
        }
        Ok(())
    }
}
