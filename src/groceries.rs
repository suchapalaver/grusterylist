use crate::{ReadError, read};
use crate::{GroceriesItem, GroceriesItemName, GroceriesItemSection, Ingredients, Recipe};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Groceries {
    pub sections: Vec<GroceriesItemSection>,
    pub collection: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
}

impl Groceries {
    pub fn new_initialized() -> Result<Self, ReadError> {
        Ok(Groceries {
            sections: vec![],
            collection: vec![],
            recipes: vec![],
        })
    }

    pub fn get_item_matches(&self, name: &str) -> impl Iterator<Item = &GroceriesItem> {
        self.collection.iter().filter(|item| item.matches(name)).collect::<Vec<_>>().into_iter()
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Groceries, ReadError> {
        Ok(serde_json::from_reader(read(path)?)?)
    }

    pub fn add_item(&mut self, item: GroceriesItem) {
        self.collection.push(item);
    }

    pub fn delete_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .collection
            .iter()
            .position(|x| x.name == GroceriesItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.collection.remove(i);
        }
        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ReadError> {
        let s = serde_json::to_string(&self)?;
        Ok(fs::write(path, s)?)
    }

    pub fn items(&self) -> impl Iterator<Item = &GroceriesItem> {
        self.sections
            .iter()
            .flat_map(|sec| self
              .collection
              .iter()
              .filter(|x| x.section.0.contains(&sec.0)))
              .collect::<Vec<_>>()
              .into_iter()
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.iter()
    }

    // check if ingredients already in lib or add them if not
    pub fn check_recipe_ingredients(&mut self, ingredients: &str) -> Result<(), ReadError> {
        let ingredients = Ingredients::from_input_string(ingredients)?;
        // add new items to groceries
        for ingredient in ingredients.iter() {
            if self.collection.iter().all(|g| &g.name != ingredient) {
                let mut section_input_ok = false;
                let mut section_input = String::new();
                while !section_input_ok {
                    eprintln!(
                        "which section is {} in?\n\
                    *1* fresh
                    *2* pantry 
                    *3* protein 
                    *4* dairy 
                    *5* freezer",
                        ingredient
                    );

                    let input = crate::get_user_input()?;

                    section_input = match &input {
                        _ if input == "1" => {
                            section_input_ok = true;
                            "fresh".to_string()
                        }
                        _ if input == "2" => {
                            section_input_ok = true;
                            "pantry".to_string()
                        }
                        _ if input == "3" => {
                            section_input_ok = true;
                            "protein".to_string()
                        }
                        _ if input == "4" => {
                            section_input_ok = true;
                            "dairy".to_string()
                        }
                        _ if input == "5" => {
                            section_input_ok = true;
                            "freezer".to_string()
                        }
                        _ => {
                            eprintln!("re-enter section information");
                            continue;
                        }
                    };
                }
                let section = GroceriesItemSection(section_input);

                let item = GroceriesItem::new_initialized(ingredient.clone(), section);

                self.add_item(item);
            }
        }
        Ok(())
    }

    pub fn add_recipe(&mut self, name: &str, ingredients: &str) -> Result<(), ReadError> {
        let recipe = Recipe(name.to_string());

        let ingredients = Ingredients::from_input_string(ingredients)?;

        self.collection
            .iter_mut()
            .filter(|x| ingredients.contains(&x.name))
            .for_each(|mut x| {
                if !x.is_recipe_ingredient {
                    x.is_recipe_ingredient = true;
                }
                x.recipes.push(recipe.clone());
            });

        self.recipes.push(recipe);

        Ok(())
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|Recipe(x)| x.as_str() == name)
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        for item in self.collection.iter_mut() {
            if let Some(i) = item.recipes.iter().position(|Recipe(x)| x.as_str() == name) {
                item.recipes.remove(i);
                if item.recipes.is_empty() {
                    item.is_recipe_ingredient = false;
                }
            }
        }
        Ok(())
    }

    pub fn recipe_ingredients(&self, recipe: &str) -> impl Iterator<Item = &GroceriesItem> {
        self
            .collection
            .iter()
            .filter(|item| item.recipes.contains(&Recipe(recipe.to_string())))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use assert_fs::prelude::*;

    fn create_test_json_file() -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test.json")?;
        file.write_str(
            r#"
            {"sections":["fresh","pantry","protein","dairy","freezer"],"collection":[{"name":"eggs","section":"dairy","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"milk","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"lemons","section":"fresh","is_recipe_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"ginger","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli"]},{"name":"spinach","section":"fresh","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast","flue flighter chicken stew"]},{"name":"garlic","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"yellow onion","section":"fresh","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew"]},{"name":"fizzy water","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"kale","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"beer","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"parsley","section":"fresh","is_recipe_ingredient":true,"recipes":["turkey meatballs","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"kefir","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"kimchi","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"sour cream","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"potatoes","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"broccoli","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli"]},{"name":"asparagus","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"dill","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"red onion","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"unsalted butter","section":"dairy","is_recipe_ingredient":true,"recipes":["chicken breasts with lemon","oatmeal chocolate chip cookies","fried eggs for breakfast"]},{"name":"scallions","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"mozzarella","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"cucumbers","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"greek yogurt","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"cream cheese","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"sweet potato","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"sausages","section":"protein","is_recipe_ingredient":true,"recipes":[]},{"name":"tofu","section":"protein","is_recipe_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"short grain brown rice","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","flue flighter chicken stew"]},{"name":"tahini","section":"pantry","is_recipe_ingredient":true,"recipes":["hummus"]},{"name":"chicken stock","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"orzo","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"pasta","section":"pantry","is_recipe_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"bread","section":"pantry","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast","peanut butter and jelly on toast","turkey and cheese sandwiches"]},{"name":"coffee","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"cumin","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"coconut milk (unsweetened)","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas"]},{"name":"tortilla chips","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"Ritz crackers","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"black beans","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"mustard","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey and cheese sandwiches"]},{"name":"chips","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"popcorn","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"olive oil","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"honey","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"black pepper","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","sheet-pan chicken with jammy tomatoes"]},{"name":"apple cider vinegar","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"pickles","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"jasmine rice","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"rice vinegar","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"balsamic vinegar","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"vegetable oil","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"baking soda","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"mayonnaise","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey and cheese sandwiches"]},{"name":"cannellini beans","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"whole-wheat tortillas","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_recipe_ingredient":false,"recipes":[]},{"name":"edamame","section":"freezer","is_recipe_ingredient":false,"recipes":[]},{"name":"ice cream","section":"freezer","is_recipe_ingredient":false,"recipes":[]},{"name":"old fashioned rolled oats","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"chocolate chips","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"baking powder","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"baking soda","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"salt","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","oatmeal chocolate chip cookies","crispy sheet-pan noodles","sheet-pan chicken with jammy tomatoes"]},{"name":"white sugar","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"vanilla extract","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"whole-wheat flour","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"tomatoes","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"parmigiana","section":"dairy","is_recipe_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"1/2 & 1/2","section":"dairy","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast"]},{"name":"feta","section":"dairy","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast"]},{"name":"instant ramen noodles","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"sesame oil","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy sheet-pan noodles"]},{"name":"soy sauce","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"baby bok choy","section":"fresh","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"cilantro","section":"fresh","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"hoisin","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"maple syrup","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"sesame seeds","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy sheet-pan noodles"]},{"name":"ground turkey","section":"protein","is_recipe_ingredient":true,"recipes":["turkey meatballs"]},{"name":"panko bread crumbs","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey meatballs"]},{"name":"garlic powder","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey meatballs"]},{"name":"skinless boneless chicken thighs","section":"protein","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew","sheet-pan chicken with jammy tomatoes"]},{"name":"carrots","section":"fresh","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew"]},{"name":"red pepper flakes","section":"pantry","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew","crispy tofu with cashews and blistered snap peas"]},{"name":"chicken broth","section":"pantry","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew","chicken breasts with lemon"]},{"name":"string beans","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"peaches","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"whipped cream","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"kiwi fruit","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"marscapone cheese","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"swordfish","section":"protein","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"eggplant","section":"fresh","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"tomato puree","section":"pantry","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"pine nuts","section":"pantry","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"french bread","section":"pantry","is_recipe_ingredient":false,"recipes":[]},{"name":"cayenne pepper","section":"pantry","is_recipe_ingredient":false,"recipes":[]}],"recipes":["oatmeal chocolate chip cookies","tomato pasta","fried eggs for breakfast","crispy sheet-pan noodles","turkey meatballs","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","turkey and cheese sandwiches","peanut butter and jelly on toast","cheese and apple snack","hummus","chicken breasts with lemon","crispy tofu with cashews and blistered snap peas","swordfish pasta"]}"#)?;
        Ok(file)
    }

    #[test]
    fn test_groceries_new() -> Result<(), Box<dyn std::error::Error>> {
        let path = "test_groceries.json";
        let g = Groceries::new_initialized()?;
        g.save(path)?;
        let g = Groceries::from_path(path)?;
        insta::assert_json_snapshot!(g, @r###"
      {
        "sections": [],
        "collection": [],
        "recipes": []
      }
      "###);
        std::fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut g = Groceries::from_path(file.path())?;
        insta::assert_json_snapshot!(g.recipes, @r###"
        [
          "oatmeal chocolate chip cookies",
          "tomato pasta",
          "fried eggs for breakfast",
          "crispy sheet-pan noodles",
          "turkey meatballs",
          "flue flighter chicken stew",
          "sheet-pan chicken with jammy tomatoes",
          "turkey and cheese sandwiches",
          "peanut butter and jelly on toast",
          "cheese and apple snack",
          "hummus",
          "chicken breasts with lemon",
          "crispy tofu with cashews and blistered snap peas",
          "swordfish pasta"
        ]
        "###);
        g.delete_recipe("oatmeal chocolate chip cookies")?;
        insta::assert_json_snapshot!(g.recipes, @r###"
        [
          "tomato pasta",
          "fried eggs for breakfast",
          "crispy sheet-pan noodles",
          "turkey meatballs",
          "flue flighter chicken stew",
          "sheet-pan chicken with jammy tomatoes",
          "turkey and cheese sandwiches",
          "peanut butter and jelly on toast",
          "cheese and apple snack",
          "hummus",
          "chicken breasts with lemon",
          "crispy tofu with cashews and blistered snap peas",
          "swordfish pasta"
        ]
        "###);
        Ok(())
    }

    #[test]
    fn test_delete_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut g = Groceries::from_path(file.path())?;
        insta::assert_json_snapshot!(g.collection, @r###"
        [
          {
            "name": "eggs",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast",
              "turkey meatballs"
            ]
          },
          {
            "name": "milk",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "lemons",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "chicken breasts with lemon",
              "hummus",
              "sheet-pan chicken with jammy tomatoes",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "ginger",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "spinach",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "garlic",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "crispy sheet-pan noodles",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "yellow onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "fizzy water",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kale",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "beer",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "parsley",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "kefir",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kimchi",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sour cream",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "potatoes",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "broccoli",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "asparagus",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dill",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "red onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "chicken breasts with lemon",
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "scallions",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "mozzarella",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cucumbers",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "greek yogurt",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cream cheese",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "sweet potato",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sausages",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "tofu",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "short grain brown rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "tahini",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "hummus"
            ]
          },
          {
            "name": "chicken stock",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "orzo",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "pasta",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
          },
          {
            "name": "bread",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "peanut butter and jelly on toast",
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "coffee",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cumin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "coconut milk (unsweetened)",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "tortilla chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "Ritz crackers",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "black beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mustard",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "popcorn",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "olive oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "sheet-pan chicken with jammy tomatoes",
              "turkey meatballs",
              "swordfish pasta"
            ]
          },
          {
            "name": "honey",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "black pepper",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "apple cider vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "pickles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "jasmine rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "rice vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "balsamic vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "vegetable oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mayonnaise",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "cannellini beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "whole-wheat tortillas",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "edamame",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "ice cream",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "chocolate chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "salt",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "oatmeal chocolate chip cookies",
              "crispy sheet-pan noodles",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "white sugar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "vanilla extract",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "whole-wheat flour",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "tomatoes",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "basil",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "parmigiana",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "feta",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "instant ramen noodles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "soy sauce",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baby bok choy",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "cilantro",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "hoisin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "maple syrup",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame seeds",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "ground turkey",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "panko bread crumbs",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "garlic powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "skinless boneless chicken thighs",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "carrots",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "red pepper flakes",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "chicken broth",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "chicken breasts with lemon"
            ]
          },
          {
            "name": "string beans",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "peaches",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "whipped cream",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kiwi fruit",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "marscapone cheese",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "swordfish",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "eggplant",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "tomato puree",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "pine nuts",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "french bread",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "cayenne pepper",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        g.delete_item("eggs")?;
        insta::assert_json_snapshot!(g.collection, @r###"
        [
          {
            "name": "milk",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "lemons",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "chicken breasts with lemon",
              "hummus",
              "sheet-pan chicken with jammy tomatoes",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "ginger",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "spinach",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "garlic",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "crispy sheet-pan noodles",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "yellow onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "fizzy water",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kale",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "beer",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "parsley",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "kefir",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kimchi",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sour cream",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "potatoes",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "broccoli",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "asparagus",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dill",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "red onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "chicken breasts with lemon",
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "scallions",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "mozzarella",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cucumbers",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "greek yogurt",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cream cheese",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "sweet potato",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sausages",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "tofu",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "short grain brown rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "tahini",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "hummus"
            ]
          },
          {
            "name": "chicken stock",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "orzo",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "pasta",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
          },
          {
            "name": "bread",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "peanut butter and jelly on toast",
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "coffee",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cumin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "coconut milk (unsweetened)",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "tortilla chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "Ritz crackers",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "black beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mustard",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "popcorn",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "olive oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "sheet-pan chicken with jammy tomatoes",
              "turkey meatballs",
              "swordfish pasta"
            ]
          },
          {
            "name": "honey",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "black pepper",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "apple cider vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "pickles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "jasmine rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "rice vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "balsamic vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "vegetable oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mayonnaise",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "cannellini beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "whole-wheat tortillas",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "edamame",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "ice cream",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "chocolate chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "salt",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "oatmeal chocolate chip cookies",
              "crispy sheet-pan noodles",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "white sugar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "vanilla extract",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "whole-wheat flour",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "tomatoes",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "basil",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "parmigiana",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "feta",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "instant ramen noodles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "soy sauce",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baby bok choy",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "cilantro",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "hoisin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "maple syrup",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame seeds",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "ground turkey",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "panko bread crumbs",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "garlic powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "skinless boneless chicken thighs",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "carrots",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "red pepper flakes",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "chicken broth",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "chicken breasts with lemon"
            ]
          },
          {
            "name": "string beans",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "peaches",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "whipped cream",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kiwi fruit",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "marscapone cheese",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "swordfish",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "eggplant",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "tomato puree",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "pine nuts",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "french bread",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "cayenne pepper",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        Ok(())
    }

    #[test]
    fn test_groceries() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut g = Groceries::from_path(file.path())?;

        insta::assert_json_snapshot!(g, @r###"
        {
          "sections": [
            "fresh",
            "pantry",
            "protein",
            "dairy",
            "freezer"
          ],
          "collection": [
            {
              "name": "eggs",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "lemons",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "ginger",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli"
              ]
            },
            {
              "name": "spinach",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "garlic",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "yellow onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "fizzy water",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kale",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "beer",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "parsley",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "kefir",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kimchi",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sour cream",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "potatoes",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "broccoli",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli"
              ]
            },
            {
              "name": "asparagus",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dill",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "red onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "chicken breasts with lemon",
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "scallions",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "mozzarella",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cucumbers",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "greek yogurt",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cream cheese",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "sweet potato",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "tofu",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "tahini",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "hummus"
              ]
            },
            {
              "name": "chicken stock",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "orzo",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "pasta",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "bread",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "peanut butter and jelly on toast",
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "coffee",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cumin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "coconut milk (unsweetened)",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "tortilla chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "Ritz crackers",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "black beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mustard",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "popcorn",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "honey",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "black pepper",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "apple cider vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "pickles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "jasmine rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "rice vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "balsamic vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "vegetable oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mayonnaise",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "cannellini beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "whole-wheat tortillas",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "edamame",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "ice cream",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "old fashioned rolled oats",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "chocolate chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "salt",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "oatmeal chocolate chip cookies",
                "crispy sheet-pan noodles",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "white sugar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "vanilla extract",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "whole-wheat flour",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "1/2 & 1/2",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "feta",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "instant ramen noodles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "soy sauce",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baby bok choy",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "cilantro",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "hoisin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "maple syrup",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame seeds",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "ground turkey",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "panko bread crumbs",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "garlic powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "skinless boneless chicken thighs",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "carrots",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "red pepper flakes",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "chicken broth",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "chicken breasts with lemon"
              ]
            },
            {
              "name": "string beans",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "peaches",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "whipped cream",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kiwi fruit",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "marscapone cheese",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "swordfish",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "eggplant",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "tomato puree",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "pine nuts",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "french bread",
              "section": "pantry",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "cayenne pepper",
              "section": "pantry",
              "is_recipe_ingredient": false,
              "recipes": []
            }
          ],
          "recipes": [
            "oatmeal chocolate chip cookies",
            "tomato pasta",
            "fried eggs for breakfast",
            "crispy sheet-pan noodles",
            "turkey meatballs",
            "flue flighter chicken stew",
            "sheet-pan chicken with jammy tomatoes",
            "turkey and cheese sandwiches",
            "peanut butter and jelly on toast",
            "cheese and apple snack",
            "hummus",
            "chicken breasts with lemon",
            "crispy tofu with cashews and blistered snap peas",
            "swordfish pasta"
          ]
        }
        "###);

        let item = GroceriesItem {
            name: crate::GroceriesItemName("cumquats".to_string()),
            section: crate::GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: true,
            recipes: vec![Recipe("cumquat chutney".to_string())],
        };
        let recipe = "cumquat chutney";

        let ingredients = "kumquats, carrots, dried apricots, dried cranberries, chili, onion, garlic, cider vinegar, granulated sugar, honey, kosher salt, cardamom, cloves, coriander, ginger, black peppercorns";

        g.add_item(item);
        g.add_recipe(recipe, ingredients)?;

        insta::assert_json_snapshot!(g, @r###"
        {
          "sections": [
            "fresh",
            "pantry",
            "protein",
            "dairy",
            "freezer"
          ],
          "collection": [
            {
              "name": "eggs",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "lemons",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "ginger",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "cumquat chutney"
              ]
            },
            {
              "name": "spinach",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "garlic",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta",
                "cumquat chutney"
              ]
            },
            {
              "name": "yellow onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "fizzy water",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kale",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "beer",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "parsley",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "kefir",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kimchi",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sour cream",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "potatoes",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "broccoli",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli"
              ]
            },
            {
              "name": "asparagus",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dill",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "red onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "chicken breasts with lemon",
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "scallions",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "mozzarella",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cucumbers",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "greek yogurt",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cream cheese",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "sweet potato",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "tofu",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "tahini",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "hummus"
              ]
            },
            {
              "name": "chicken stock",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "orzo",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "pasta",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "bread",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "peanut butter and jelly on toast",
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "coffee",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cumin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "coconut milk (unsweetened)",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "tortilla chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "Ritz crackers",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "black beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mustard",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "popcorn",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "honey",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "cumquat chutney"
              ]
            },
            {
              "name": "black pepper",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "apple cider vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "pickles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "jasmine rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "rice vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "balsamic vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "vegetable oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mayonnaise",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "cannellini beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "whole-wheat tortillas",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "edamame",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "ice cream",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "old fashioned rolled oats",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "chocolate chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "salt",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "oatmeal chocolate chip cookies",
                "crispy sheet-pan noodles",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "white sugar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "vanilla extract",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "whole-wheat flour",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "1/2 & 1/2",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "feta",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "instant ramen noodles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "soy sauce",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baby bok choy",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "cilantro",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "hoisin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "maple syrup",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame seeds",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "ground turkey",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "panko bread crumbs",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "garlic powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "skinless boneless chicken thighs",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "carrots",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "cumquat chutney"
              ]
            },
            {
              "name": "red pepper flakes",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "chicken broth",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "chicken breasts with lemon"
              ]
            },
            {
              "name": "string beans",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "peaches",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "whipped cream",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kiwi fruit",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "marscapone cheese",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "swordfish",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "eggplant",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "tomato puree",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "pine nuts",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "french bread",
              "section": "pantry",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "cayenne pepper",
              "section": "pantry",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "cumquats",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "cumquat chutney"
              ]
            }
          ],
          "recipes": [
            "oatmeal chocolate chip cookies",
            "tomato pasta",
            "fried eggs for breakfast",
            "crispy sheet-pan noodles",
            "turkey meatballs",
            "flue flighter chicken stew",
            "sheet-pan chicken with jammy tomatoes",
            "turkey and cheese sandwiches",
            "peanut butter and jelly on toast",
            "cheese and apple snack",
            "hummus",
            "chicken breasts with lemon",
            "crispy tofu with cashews and blistered snap peas",
            "swordfish pasta",
            "cumquat chutney"
          ]
        }
        "###);

        Ok(())
    }
}
