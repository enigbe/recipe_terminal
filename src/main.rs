use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Recipe {
    title: String,
    description: String,
    author: String,
}

pub struct Recipes {
    inner: HashMap<String, Recipe>,
}

impl Recipes {
    /// create new Recipes struct
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    /// add Recipe to Recipes
    fn add(&mut self, recipe: Recipe) {
        self.inner.insert(recipe.title.to_string(), recipe);
    }
    /// get all recipes
    fn get_all(&self) -> Vec<&Recipe> {
        self.inner.values().collect()
    }

    /// deletes a recipe from the map
    fn remove(&mut self, title: &str) -> bool{
        self.inner.remove(title).is_some()
    }

    /// updates an existing recipe
    fn update(&mut self, title: &str, desc: &str, author: &str) -> bool {
        match self.inner.get_mut(title) {
            Some(recipes_map) => {
                recipes_map.description = desc.to_owned();
                recipes_map.author = author.to_owned();
                true
            },
            None => false,
        }
    }

}
enum RecipeMenu {
    AddRecipe,
    ViewRecipe,
    UpdateRecipe,
    DeleteRecipe,
    ViewAllRecipes
}

impl RecipeMenu {

    fn from_str(input: &str) -> Option<RecipeMenu> {
        match input {
            "1" => Some(Self::AddRecipe),
            "2" => Some(Self::ViewRecipe),
            "3" => Some(Self::UpdateRecipe),
            "4" => Some(Self::DeleteRecipe),
            "5" => Some(Self::ViewAllRecipes),
            _ => None,
        }
    }

    /// show recipe menu
    fn show() {
        println!("-- Recipe Manager --");
        println!("1. Add Recipe");
        println!("2. View Recipe");
        println!("3. Update Recipe");
        println!("4. Delete Recipe");
        println!("5. View All Recipes");
        println!("");
        println!("Enter selection number (e.g. 1)");
    }
}

/// get_input: function to collect user input
fn get_input() -> Option<String> {
    // buffer to hold user input
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

mod menu {
    use crate::{Recipe, Recipes, get_input};

    /// add a new recipe to the recipes collection
    pub fn add_recipe(recipes: &mut Recipes) {
        println!("Recipe title:");
        let title = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Recipe description:");
        let description = match get_input() {
            Some(description) => description,
            None => return,
        };
        println!("Recipe author:");
        let author = match get_input() {
            Some(author) => author,
            None => return,
        };

        let new_recipe = Recipe {
            title,
            description,
            author,
        };

        recipes.add(new_recipe);
        println!("new recipe added");
    }

    /// view all the recipes in the collection
    pub fn view_recipes(recipes_map: &Recipes) {
        // loop through the recipes vector and print each recipe
        for recipe in recipes_map.get_all() {
            println!("{:?}", recipe);
        }
    }

    /// remove recipe
    pub fn  remove_recipe(recipes: &mut Recipes) {
        for recipe in recipes.get_all() {
            println!("{:?}", recipe);
        }
        println!("Enter recipe title to remove");

        let title = match get_input() {
            Some(title) => title,
            None => return,
        };

        if recipes.remove(&title) {
            println!("recipe remove");
        } else {
            println!("recipe not found");
        }
    }

    /// update a recipe
    pub fn update_recipe(recipes: &mut Recipes) {
        for recipe in recipes.get_all() {
            println!("{:?}", recipe);
        }
        println!("enter recipe title to update");
        let title = match get_input() {
            Some(title) => title,
            None => return,
        };
        println!("enter recipe description to update");
        let description = match get_input() {
            Some(description) => description,
            None => return,
        };
        println!("enter recipe author to update");
        let author = match get_input() {
            Some(author) => author,
            None => return,
        };

        if recipes.update(&title, &description, &author) {
            println!("updated")
        } else {
            println!("recipe not found")
        }
    }
}

fn run() -> Option<()> {
    loop {
        let mut recipes_map = Recipes::new();
        // 1. Display recipe menu
        RecipeMenu::show();
        let input = get_input()?;
        // 2. Make a choice on the recipe menu based on user input
        match RecipeMenu::from_str(input.as_str()) {
            Some(RecipeMenu::AddRecipe) => menu::add_recipe(&mut recipes_map),
            Some(RecipeMenu::ViewRecipe) => (),
            Some(RecipeMenu::UpdateRecipe) => (menu::update_recipe(&mut recipes_map)),
            Some(RecipeMenu::DeleteRecipe) => (menu::remove_recipe(&mut recipes_map)),
            Some(RecipeMenu::ViewAllRecipes) => menu::view_recipes(&recipes_map),
            None => break,
        }
    }
    None
}
fn main() {
    run();
}
