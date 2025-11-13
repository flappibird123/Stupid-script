# Stupid Script

Stupid Script is a **static, interpreted programming language** with manual memory management. It features a C-style syntax with curly braces, semicolons, and `let`/`const` variable declarations.

---

## Features

* **Static typing** with `int`, `float`, `bool`, and `string` primitive types
* **Manual memory management**
* Curly-brace syntax (`{}`) for code blocks
* `let` and `const` variable declarations
* Interpreted execution

---

## Installation

1. **Clone the repository:**

```bash
git clone https://github.com/yourusername/stupid-script.git
cd stupid-script
```

2. **Build the interpreter (Rust):**

```bash
cargo build --release
```

3. **Run the interpreter:**

```bash
cargo run -- path/to/your/code.sst
```

> Replace `code.ss` with your Stupid Script source file.

---

## Usage

Example of a Stupid Script program:

```c
let int x = 5;
const int y = 10;

if (x < y) {
    print("x is less than y");
}
```

Run the program with:

```bash
cargo run -- examples/test.sst
```

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Commit (`git commit -am 'Add new feature'`)
5. Push (`git push origin feature/your-feature`)
6. Create a Pull Request

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Future features

* Add support for arrays and structs
* Implement a standard library with I/O functions
* Add unit testing framework for Stupid Script programs
* Improve error handling and debugging tools
