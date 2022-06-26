use crate::{GroceriesItem, GroceriesItemName, ReadError, Recipe};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    pub groceries: Vec<GroceriesItem>,
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self::new()
    }
}

impl ShoppingList {
    pub fn new() -> Self {
        Self::new_initialized()
    }

    fn new_initialized() -> Self {
        ShoppingList {
            checklist: vec![],
            recipes: vec![],
            groceries: vec![],
        }
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, ReadError> {
        let reader = crate::helpers::read(path)?;

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn print(&self) {
        if !self.checklist.is_empty() {
            println!("Check if we need:");

            self.checklist.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
        if !self.recipes.is_empty() {
            println!("recipes:");

            self.recipes.iter().for_each(|recipe| {
                println!("\t{}", recipe);
            });
        }
        if !self.groceries.is_empty() {
            println!("groceries:");

            self.groceries.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
    }

    pub fn add_groceries(&mut self, path: &str) -> Result<(), ReadError> {
        // move everything off list to temp list
        let list_items: Vec<GroceriesItem> = self.groceries.drain(..).collect();
        assert!(self.groceries.is_empty());
        let sections = vec!["fresh", "pantry", "dairy", "protein", "freezer"];
        let groceries = crate::Groceries::from_path(path)?;
        let groceries_by_section: Vec<Vec<GroceriesItem>> = {
            sections
                .into_iter()
                .map(|section| {
                    let mut a: Vec<GroceriesItem> = list_items
                        .iter()
                        .filter(|groceriesitem| groceriesitem.section.0 == section)
                        .cloned()
                        .collect();

                    let b: Vec<GroceriesItem> = groceries
                        .collection
                        .iter()
                        .filter(|groceriesitem| {
                            groceriesitem.section.0 == section && !a.contains(groceriesitem)
                        })
                        .cloned()
                        .collect();
                    a.extend(b);
                    a
                })
                .collect()
        };
        for section in groceries_by_section {
            if !section.is_empty() {
                for groceriesitem in &section {
                    if !self.groceries.contains(groceriesitem)
                        && groceriesitem
                            .recipes
                            .iter()
                            .any(|recipe| self.recipes.contains(&*recipe))
                    {
                        self.add_groceries_item(groceriesitem.clone());
                    }
                }
                for groceriesitem in section {
                    if !self.groceries.contains(&groceriesitem) {
                        eprintln!(
                            "Do we need {}?\n\
                              *y*\n\
                              *any other key* for next item\n\
                              *s* for next section",
                            groceriesitem.name.0.to_lowercase()
                        );

                        match crate::get_user_input()?.as_str() {
                            "y" => {
                                if !self.groceries.contains(&groceriesitem) {
                                    self.add_groceries_item(groceriesitem.clone());
                                }
                            }
                            "s" => break,
                            &_ => continue,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_groceries_item(&mut self, item: GroceriesItem) {
        self.groceries.push(item)
    }

    pub fn delete_groceries_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .groceries
            .iter()
            .position(|x| x.name == GroceriesItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.groceries.remove(i);
        }
        Ok(())
    }

    pub fn add_checklist_item(&mut self, item: GroceriesItem) {
        self.checklist.push(item)
    }

    pub fn delete_checklist_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .checklist
            .iter()
            .position(|x| x.name == GroceriesItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.checklist.remove(i);
        }
        Ok(())
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe)
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|x| x == &Recipe(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        Ok(())
    }

    pub fn to_json_string(&self) -> Result<String, ReadError> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn save(&self, path: &str) -> Result<(), ReadError> {
        let json = self.to_json_string()?;
        crate::helpers::write(path, json)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use crate::GroceriesItemSection;
    use assert_fs::prelude::*;

    // test suite helper function
    fn create_test_json_file() -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test.json")?;
        file.write_str(
            r#"
            {"checklist":[],"recipes":["tomato pasta"],"groceries":[{"name":"garlic","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"tomatoes","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"lemons","section":"fresh","is_recipe_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"pasta","section":"pantry","is_recipe_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"olive oil","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"short grain brown rice","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","flue flighter chicken stew"]},{"name":"parmigiana","section":"dairy","is_recipe_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"eggs","section":"dairy","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"sausages","section":"protein","is_recipe_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_recipe_ingredient":false,"recipes":[]}]}
            "#
        )?;
        Ok(file)
    }

    #[test]
    fn test_delete_groceries_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut sl = ShoppingList::from_path(file.path())?;
        let item = GroceriesItem {
            name: GroceriesItemName("kumquats".to_string()),
            section: GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: false,
            recipes: vec![],
        };
        sl.add_groceries_item(item);
        insta::assert_json_snapshot!(sl.groceries, @r###"
        [
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
            "name": "pasta",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
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
            "name": "short grain brown rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "flue flighter chicken stew"
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
            "name": "sausages",
            "section": "protein",
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
            "name": "kumquats",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        sl.delete_groceries_item("kumquats")?;
        insta::assert_json_snapshot!(sl.groceries, @r###"
        [
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
            "name": "pasta",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
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
            "name": "short grain brown rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "flue flighter chicken stew"
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
            "name": "sausages",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        Ok(())
    }

    #[test]
    fn test_delete_checklist_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut sl = ShoppingList::from_path(file.path())?;
        let item = GroceriesItem {
            name: GroceriesItemName("kumquats".to_string()),
            section: GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: false,
            recipes: vec![],
        };
        sl.add_checklist_item(item);
        insta::assert_json_snapshot!(sl.checklist, @r###"
        [
          {
            "name": "kumquats",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        sl.delete_checklist_item("kumquats")?;
        insta::assert_json_snapshot!(sl.checklist, @"[]");
        Ok(())
    }

    #[test]
    fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut sl = ShoppingList::from_path(file.path())?;
        insta::assert_json_snapshot!(sl.recipes, @r###"
        [
          "tomato pasta"
        ]
        "###);
        sl.delete_recipe("tomato pasta")?;
        insta::assert_json_snapshot!(sl.recipes, @"[]");
        Ok(())
    }

    #[test]
    fn json_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let list = ShoppingList::from_path(file.path())?;

        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta"
          ],
          "groceries": [
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
              "name": "pasta",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
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
              "name": "short grain brown rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
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
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            }
          ]
        }
        "###);
        Ok(())
    }

    #[test]
    fn test_add_groceries_item_and_add_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut list = ShoppingList::from_path(file.path())?;
        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta"
          ],
          "groceries": [
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
              "name": "pasta",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
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
              "name": "short grain brown rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
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
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            }
          ]
        }
        "###);

        let item = GroceriesItem {
            name: crate::GroceriesItemName("cumquats".to_string()),
            section: crate::GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: true,
            recipes: vec![Recipe("cumquat chutney".to_string())],
        };
        let recipe = Recipe("cumquat chutney".to_string());
        list.add_groceries_item(item);
        list.add_recipe(recipe);
        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta",
            "cumquat chutney"
          ],
          "groceries": [
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
              "name": "pasta",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
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
              "name": "short grain brown rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
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
              "name": "sausages",
              "section": "protein",
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
              "name": "cumquats",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "cumquat chutney"
              ]
            }
          ]
        }
        "###);

        Ok(())
    }
}
