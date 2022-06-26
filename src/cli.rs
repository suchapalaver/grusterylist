use clap::{Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new("grusterylist")
        .about("Makes grocery lists")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // recipes
        .subcommand(
            Command::new("recipes")
                .about("Manages recipes library")
                .subcommand_required(true)
                .subcommand(
                    // recipes add
                    Command::new("add")
                        .subcommand_required(false)
                        .about("Adds recipes to library")
                        .arg(
                            Arg::with_name("name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be added"),
                        )
                        .arg(
                            Arg::with_name("ingredients")
                                .short('i')
                                .long("ingredients")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be added"),
                        ),
                )
                .subcommand(
                    // recipes delete-recipe
                    Command::new("delete-recipe")
                        .about("Deletes recipe from library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be deleted"),
                        ),
                )
                .subcommand(
                    // recipes edit
                    Command::new("edit")
                        .about("Edits a recipe ingredient")
                        .arg(
                            Arg::with_name("ingredient")
                                .long("ingredient")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of ingredient to be edited"),
                        )
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be edited"),
                        )
                        .arg(
                            Arg::with_name("edit")
                                .long("edit")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides replacement for item to be edited"),
                        ),
                )
                .subcommand(
                    // recipes delete-ingredient
                    Command::new("delete-ingredient")
                        .about("Delete an ingredient from a recipe")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be edited"),
                        )
                        .arg(
                            Arg::with_name("ingredient")
                                .long("ingredient")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of ingredient to be deleted"),
                        ),
                )
                .subcommand(
                    // recipes add-ingredient
                    Command::new("add-ingredient")
                        .about("Adds an ingredient to a recipe")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be edited"),
                        )
                        .arg(
                            Arg::with_name("ingredient")
                                .long("ingredient")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of ingredient to be added"),
                        ),
                )
                .subcommand(
                    // recipes print
                    Command::new("print")
                        .about("Print recipes or recipe ingredients")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to view"),
                        ),
                )
                // --path groceries.json
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                ),
        )
        .subcommand(
            Command::new("groceries")
                .about("Manages groceries library")
                .subcommand(Command::new("add").about("Adds grocery items to library"))
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Makes shopping lists")
                .subcommand_required(true)
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("list.json")
                        .help("Provides path for shopping list"),
                )
                .arg(
                    Arg::with_name("library path")
                        .long("lib-path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                )
                .subcommand(Command::new("print").about("Print saved shopping list"))
                .subcommand(
                    Command::new("create")
                        .about("Create a shopping list")
                        .arg(
                            Arg::with_name("path")
                                .long("path")
                                .takes_value(true)
                                .default_value("list.json")
                                .help("Provides path for shopping list"),
                        )
                        .arg(
                            Arg::with_name("library path")
                                .long("lib-path")
                                .takes_value(true)
                                .default_value("groceries.json")
                                .help("Provides path for groceries library"),
                        )
                        .arg(
                            Arg::with_name("fresh")
                                .short('f')
                                .long("fresh")
                                .takes_value(false)
                                .help("Make a new list from scratch"),
                        ),
                ),
        )
}
