use std::path::PathBuf;
use crate::todo::Todo;
use std::fs;
use serde_json;
use directories::ProjectDirs;
use anyhow::{Result, Context, anyhow};
use colored::*;

pub struct Store{
    todos: Vec<Todo>,
    path: PathBuf,
}

impl Store{
        /// Load store from disk (or create a new empty store).
    pub fn load() ->Result<Self>{
        let path = get_db_path();
        if path.exists(){
            let s = fs::read_to_string(&path).context("reading todo file")?;
            let todos: Vec<Todo> = serde_json::from_str(&s).unwrap_or_default();
            Ok(Self{todos, path})
        }else {
            Ok(Self{todos: Vec::new(), path})
        }

    }

    // save current todos to disk
    pub fn save(&self)-> Result<()>{
        let s = serde_json::to_string_pretty(&self.todos)?;
        fs::write(&self.path, s).context("writing todo file")?;
        Ok(())
    }

    fn next_id(&self) -> u32 {
        self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn add(&mut self, mut todo: Todo) {
        todo.id = self.next_id();
        self.todos.push(todo);
    }

        /// Print todos; if `all` == false, show only not-done items.
    pub fn list(&self, all:bool){
        if self.todos.is_empty(){
            println!("{}","No todos yet. Add one with `todo-cli add <task>`".yellow());
            return;
        }

        for t in &self.todos{
            if !all && t.done{
                continue;
            }

            let status = if t.done {
                "[x]".green()
            } else {
                "[ ]".red()
            };

            let id_str = format!("{:>3}", t.id).cyan();

            let text_colored = if t.done {
                t.text.clone().dimmed()
            } else {
                t.text.clone().bold()
            };

            println!("{} {} - {}", status, id_str, text_colored);
            }
    }

    pub fn mark_done(&mut self, id: u32) -> Result<()> {
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(t) => {
                t.done = true;
                Ok(())
            }
            None => Err(anyhow!("No todo with id {}", id)),
        }
    }

    pub fn remove(&mut self, id: u32) -> Result<()> {
        let before = self.todos.len();
        self.todos.retain(|t| t.id != id);
        if self.todos.len() == before {
            Err(anyhow!("No todo with id {}", id))
        } else {
            Ok(())
        }
    }

    pub fn edit(&mut self, id: u32, new_text: &str) -> Result<()> {
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(t) => {
                t.text = new_text.to_string();
                Ok(())
            }
            None => Err(anyhow!("No todo with id {}", id)),
        }
    }

    pub fn clear(&mut self) {
        self.todos.clear();
    }

}

/// Choose a platform-appropriate data-file location, otherwise `todos.json` in current dir.
fn get_db_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "example", "todo-cli") {
        let data_dir = proj_dirs.data_dir();
        if let Err(e) = std::fs::create_dir_all(data_dir) {
            eprintln!("Warning: couldn't create data dir {:?}: {}", data_dir, e);
        }
        data_dir.join("todos.json")
    } else {
        PathBuf::from("todos.json")
    }
}