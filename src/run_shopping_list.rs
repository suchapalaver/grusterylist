use crate::ReadError;
use crate::ShoppingList;
use clap::ArgMatches;

pub fn run(sync_matches: &ArgMatches) -> Result<(), ReadError> {
    let path = sync_matches.get_one::<String>("path").unwrap();
    let lib_path = sync_matches.get_one::<String>("lib-path").unwrap();

    match sync_matches.subcommand() {
        Some(("print", _)) => {
            let list = ShoppingList::from_path(path)?;
            list.print();
        }
        _ => match sync_matches.subcommand() {
            Some(("create", sync_matches)) => {
                let mut sl = ShoppingList::new();
                if !sync_matches.contains_id("fresh") {
                    sl = ShoppingList::from_path(path)?;
                }
                sl.prompt_add_recipes(path)?;

                sl.prompt_add_groceries(lib_path)?;

                sl.prompt_save_list(path)?;
            }
            _ => unreachable!(),
        },
    }
    Ok(())
}

impl ShoppingList {
    // pub(crate) fn prompt_view_list(&self) -> Result<(), ReadError> {
    //     if !self.groceries.is_empty() {
    //         eprintln!(
    //             "\n\
    //     Print shopping list?\n\
    //     *y*\n\
    //     *any other key* to continue"
    //         );
    //
    //         if crate::prompt_for_y()? {
    //             self.print();
    //             println!();
    //         }
    //     }
    //     Ok(())
    // }

    pub(crate) fn prompt_add_recipes(&mut self, path: &str) -> Result<(), ReadError> {
        eprintln!(
            "Add recipe ingredients to our list?\n\
                *y*\n\
                *any other key* to continue"
        );

        while crate::prompt_for_y()? {
            let groceries = crate::Groceries::from_path(path)?;

            for recipe in groceries.recipes.into_iter() {
                eprintln!(
                    "Shall we add ...\n\
                            {}?\n\
                            *y*\n\
                            *s* to skip to end of recipes\n\
                            *any other key* for next recipe",
                    recipe
                );

                match crate::get_user_input()?.as_str() {
                    "y" => {
                        if !self.recipes.contains(&recipe) {
                            self.add_recipe(recipe);
                        }
                    }
                    "s" => break,
                    &_ => continue,
                }
            }
            eprintln!(
                "Add any more recipe ingredients to our list?\n\
                    *y*\n\
                    *any other key* to continue"
            );
        }
        Ok(())
    }

    pub(crate) fn prompt_add_groceries(&mut self, path: &str) -> Result<(), ReadError> {
        eprintln!(
            "Add groceries to shopping list?\n\
            *y*\n\
            *any other key* to skip"
        );

        while crate::prompt_for_y()? {
            self.add_groceries(path)?;
            eprintln!(
                "Add more groceries to shopping list?\n\
            *y*\n\
            *any other key* to skip"
            );
        }
        Ok(())
    }

    pub(crate) fn prompt_save_list(&mut self, path: &str) -> Result<(), ReadError> {
        // don't save list if empty
        if !self.checklist.is_empty() && !self.groceries.is_empty() && !self.recipes.is_empty() {
            eprintln!(
                "Save current list?\n\
                *y*\n\
                *any other key* to continue"
            );

            if crate::prompt_for_y()? {
                self.save(path)?;
            }

            self.print();
        }
        Ok(())
    }
}
