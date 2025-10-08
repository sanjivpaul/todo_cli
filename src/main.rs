mod storage;
mod todo;

use anyhow::Result;
use clap::{ Parser, Subcommand };
use crate::storage::Store;
use colored::*;


#[derive(Parser)]
#[command(author, version, about="Simple TODO CLI app")]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    /// Add a new todo. Provide the todo text (multiple words allowed).
    Add {
        #[arg(required = true)]
        text: Vec<String>,
    },

    /// List todos. By default only shows pending items.
    List {
        /// show all (including completed)
        #[arg(short, long)]
        all: bool,
    },
        /// Mark a todo as done by id.
    Done {
        id: u32,
    },

    /// Remove a todo by id.
    Remove {
        id: u32,
    },

    /// Edit a todo text by id.
    Edit {
        id: u32,
        #[arg(required = true)]
        text: Vec<String>,
    },

    /// Clear all todos (destructive).
    Clear,
    
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = Store::load()?;

    match cli.command {
       Commands::Add { text } => {
        let joined = text.join(" ");
        let t = todo::Todo::new(joined);
        store.add(t);
        store.save()?;
        println!("{}", "Added todo.".green().bold());
       }

       Commands::List { all } => {
            store.list(all);
        }
        Commands::Done { id } => {
            store.mark_done(id)?;
            store.save()?;
            println!("{}", "Marked 1 done.".green());
        }
        Commands::Remove { id } => {
            store.remove(id)?;
            store.save()?;
            println!("{}", "Removed 1.".red());
        }
        Commands::Edit { id, text } => {
            store.edit(id, &text.join(" "))?;
            store.save()?;
            println!("Edited {}.", id);
        }
        Commands::Clear => {
            store.clear();
            store.save()?;
            println!("{}", "Cleared all todos.".yellow());
        }
        
    }
    Ok(())
}
