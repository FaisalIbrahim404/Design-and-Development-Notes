# Rust Syntax Essentials 🦀

### Indentation and Line Endings
- Rust style prefers indentation with four spaces (not tabs) 🛠.
- Lines end with a semicolon (`;`), signaling the end of an expression.

### Compiling and Running Rust Programs
- Compile Rust programs with the Rust compiler (`rustc`) followed by the source file name:
  ```
  rustc file_name.rs
  ```
- Run the compiled program:
  ```
  ./file_name
  ```

### Creating and Managing Rust Projects with Cargo 📦
- **First Method:** As described above.
- **Second Method:** Use `cargo`, Rust's package manager and build system, to create and manage projects:
  - Create a new project:
    ```
    cargo new project_name
    cd ./project_name
    ```
  - Compile your project:
    ```
    cargo build
    ```
  - Run your project in two ways:
    - Direct execution:
      ```
      ./target/debug/project_name.exe
      ```
    - Using cargo:
      ```
      cargo run
      ```

### Key Rust Concepts
- **Cargo.toml:** Rust's configuration file, similar to `package.json` in React.
- **Mutability:** Use `mut` to declare mutable variables.
- **Associated Functions:** Use `::new` for associated functions, such as constructing a new instance.
- **String Input:** `read_line` appends user input to a string, preserving its content.
- **References:** Use `&` for references, allowing multiple parts of your code to access the same data without multiple copies.
- **Enums and Result Types:** Rust uses enums (like `Result`) for handling multiple possible states or outcomes.
- **Dependencies:** In `Cargo.toml`, specify external crate dependencies and versions in `[dependencies]`.

### Tips for Better Code 📘
- **Breaking Long Lines:** For readability, break long lines at method calls:
  ```rust
  // Less readable
  io::stdin().read_line(&mut guess).expect("Failed to read line");
  
  // More readable
  io::stdin().read_line(&mut guess)
             .expect("Failed to read line");
  ```
- **Trimming Input:** Use the `trim` method to remove whitespace from string inputs, particularly useful for processing user input.

### Additional Resources and Tools 🔍
- **The Rust Programming Language Book:** A comprehensive guide to learning Rust. Highly recommended (5/5 👌✨).
- **Documentation:** Generate and browse local documentation for dependencies using:
  ```
  cargo doc --open
  ```
  
### Syntax Notes Taken on 28/02/2024

---

This version aims to keep the original notes' informative nature while making them more engaging and easier to navigate.
