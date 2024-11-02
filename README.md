![Alt text](assets/ruscalculator-logo.jpg)


# 📐 Simple Expression Parsing Calculator

A simple and efficient expression parsing calculator that evaluates mathematical expressions with basic arithmetic, exponentiation, and trigonometric functions. This calculator parses input expressions like `7 / 9 + 15 ^ 8`, calculates the result, and supports the following features:

## Features

### Supported Operations
- **Addition** (`+`)
- **Subtraction** (`-`)
- **Multiplication** (`*`)
- **Division** (`/`)
- **Exponentiation** (`^`)

### Supported Trigonometric Functions
- **sin**: Calculates the sine of an angle (input in degrees)
- **cos**: Calculates the cosine of an angle (input in degrees)
- **tan**: Calculates the tangent of an angle (input in degrees)

### Example Expressions
- `3 + 4 * 2 / (1 - 5) ^ 2 ^ 3`
- `sin(30) + cos(45) - tan(60)`
- `2 ^ 3 + 5 * 4`

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/RamezEssam/Rusculator
   cd calculator
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

To run the calculator:

```bash
cargo run
```

You can then enter expressions like:

```
7 / 9 + 15 ^ 8
sin(30) + cos(45)
2 ^ 10 - 3
```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any suggestions or improvements.

## License

This project is licensed under the MIT License.

---

Enjoy calculating! 🧮
