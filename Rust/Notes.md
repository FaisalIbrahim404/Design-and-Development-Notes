# Some important syntax notes:
  - Rust style is to indent with four spaces, not a tab.
  - We end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.
  - Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc` command and passing it the name of your source file, like this:
    ```
    rustc file_name.rs
    ```
    then you can run the compiled version, like this:
    ```
    .\file_name
    ```
  - There is two way to create **Rust** and run it:
      - First one: In last note
      - Second one: Is by create project **Rust** using `cargo` tool, like this:
        ```
        cargo new project_name
        cd .\project_name
        ```
        to compile your project we will use this command
        ```
        cargo build
        ```
        to run your project there is two ways:
          - First way:
            ```
            .\target\debug\project_name.exe
            ```
          - Second way:
            ```
            cargo run
            ```
  - *`Cargo.toml`* like *`package.json`* in `React`
  - `mut` to create mutable variables
  - The `::` syntax in the `::new` line indicates that new is an associated function of the ` String` type.
  - The full job of `read_line` is to take whatever the user types into standard input and append that into a string ***(without overwriting its contents)***, so we therefore pass that string as an argument.
  - The `&` indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.
  - `read_line` puts whatever the user enters into the `string` we pass to it, but it also returns a `Result` value. [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) is an [`enumeration`](https://doc.rust-lang.org/book/ch06-00-enums.html), often called an *enum*, which is a type that can be in one of multiple possible states. We call each possible state a **variant**.
  - In the *`Cargo.toml file`*, everything that follows a header is part of that section that continues until another section starts.
  - In **`[dependencies]`** you tell `Cargo` which external crates your project depends on and which versions of those crates you require. In this case, we specify the rand crate with the semantic version specifier `0.8.5`.
  - `Cargo` understands *Semantic Versioning* **(sometimes called SemVer)**, which is a standard for writing version numbers. The specifier `0.8.5` is actually shorthand for `^0.8.5`, which means **any version that is at least 0.8.5 but below 0.9.0.**
  - When we include an external dependency, `Cargo` fetches the latest versions of everything that dependency needs from the *registry*, which is a copy of data from [**Crates.io**](https://crates.io/). **Crates.io*z* is where people in the Rust ecosystem post their open source Rust projects for others to use.
  - Another neat feature of Cargo is that running the **`cargo doc --open`** command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the `rand crate`, for example, run **`cargo doc --open`** and click `rand` in the sidebar on the left.
  - The **`trim method`** on a *String* instance will eliminate any whitespace at the beginning and end.
  - The user must press *enter* to satisfy `read_line` and *input* their guess, **which adds a newline character to the string**. For example, if the user types 5 and presses enter, guess looks like this: *`5\n`*
  - **(On Windows, pressing enter results in a carriage return and a newline, `\r\n`.)** The `trim` method eliminates `\n` or `\r\n`, resulting in just 5.
  - The `parse` method on *strings* **converts a string to another type**.
  - We use it to convert from a *string* to a *number*. We need to tell `Rust` the exact number type we want by using   `let guess: u32`. The colon `:` after guess tells Rust we’ll annotate the variable’s type


# Best Brackets:
  - one long line is difficult to read, so it’s best to divide it. It’s often wise to introduce a newline and other whitespace to help break up long lines when you call a method with the .method_name() syntax, like:
    ```Rust
    // not best brackets
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // best brackets
    io::stdin().read_line(&mut guess)
               .expect("Failed to read line");
    ```

# Resources: 
  - [The Rust Programming Language Book](https://doc.rust-lang.org/book/), full guide to learn **Rust Language**  `Perfect Book 5/5👌✨`

## This notes taked at `28/2/2024`
