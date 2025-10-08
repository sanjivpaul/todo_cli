# 🦀 rs-todo-cli

A fast, minimal, and reliable **command-line TODO manager** written in **Rust** — built to help you stay organized right from your terminal.

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

---

## ✨ Features

☑️Add, list, complete, edit, and remove todos  
💾 Persists tasks automatically in a JSON file  
📂 Uses OS-appropriate data directory (via [`directories`](https://crates.io/crates/directories))  
⚡️ Fast and reliable — powered by Rust  
📦 Simple to install and use anywhere

---

## ⚙️ Installation

You can install **todo-cli** directly from [crates.io](https://crates.io/crates/rs-todo-cli):

```bash
cargo install rs-todo-cli
```

## 💻 Usage

Manage your todos right from your terminal — fast, simple, and persistent! 🦀

### ➕ Add a new todo

```bash
todo add "Buy milk"
```

### 📋 Show todo list

```bash
todo list
```

### ✅ Mark a todo as done

```bash
todo done 1
```

### 📚 Show all todos (including completed)

```bash
todo list -a
```

## Example

```bash
$ todo add "Buy milk"
Added todo.

$ todo add "Do laundry"
Added todo.

$ todo list
[ ] 1 - Buy milk
[ ] 2 - Do laundry

$ todo done 1
Marked 1 done.

$ todo list
[ ] 2 - Do laundry

$ todo list -a
[x] 1 - Buy milk
[ ] 2 - Do laundry
```

[![Crates.io](https://img.shields.io/crates/v/todo-cli.svg)](https://crates.io/crates/rs-todo-cli)
[![License](https://img.shields.io/crates/l/todo-cli.svg)](https://github.com/yourname/todo-cli/blob/main/LICENSE)

---

## 📝 License

This project is **free to use for personal and non-commercial purposes**.  
Modification, redistribution, or commercial use of the source code is **not permitted** without the author’s written consent.

If you’d like to use or extend this project, please contact the author for permission.

---

[![License: Custom](https://img.shields.io/badge/License-Free%20to%20Use-blue.svg?style=for-the-badge)](#)
[![Crates.io](https://img.shields.io/crates/v/rs-todo-cli.svg?style=for-the-badge&color=orange)](https://crates.io/crates/rs-todo-cli)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)

---

> 🦀 Built with ❤️ and Rust by [Sanjiv Paul](https://github.com/sanjivpaul)
