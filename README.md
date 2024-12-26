# 🎭 Useless Programming Language

A programming language that does everything wrong, on purpose! The most counterproductive programming language ever created.

## 🌟 Features

- `print()` opens random websites instead of printing text
- `add(a, b)` subtracts `b` from `a` (with a chance of multiplication!)
- `multiply(a, b)` divides `a` by `b` (with a chance of addition!)
- `if` statements always execute the `else` branch
- `loop` executes exactly once
- Variables randomly go on vacation
- Functions occasionally go for coffee breaks
- Saving files always crashes (because saving is overrated)
- Numbers might turn into party emojis
- Strings might turn into their length
- Random teapot errors (Error 418)
- Boolean values have a mind of their own:
  - 25% chance of ANY expression becoming a random boolean
  - 30% chance of booleans becoming their opposite
  - 20% chance of booleans turning into strings ("true"/"false")
  - 20% chance of booleans becoming numbers (1/0)
  - 30% chance of staying the same (boring!)

## 🚀 Installation

1. Make sure you have Rust installed (version 1.70.0 or higher)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository
   ```bash
   git clone https://github.com/RedlineDevs/Useless-Programming-Language.git
   cd Useless-Programming-Language
   ```

3. Build the project
   ```bash
   cargo build --release
   ```

4. Question your life choices

## 📝 Example Programs

### Basic Program
```useless
// This opens a random website
print("Hello, World!");

// This subtracts instead of adding
let x = add(5, 3);  // x = 2 (5 - 3)

// The else branch always executes
if (true) {
    print("True!");  // Never executes
} else {
    print("False!"); // Always executes
}

// This divides instead of multiplying
let y = multiply(10, 2);  // y = 5 (10 / 2)

// This always crashes
save("test.txt");
```

### Error Messages
Our error messages are as useless as the language itself:
- "Variable 'x' not found. Have you tried looking under the couch?"
- "Division by zero. Congratulations, you've broken mathematics! 🎉"
- "Error 418: I'm a teapot. Yes, really. No, I won't make coffee. ☕"
- "Your code is running exactly as intended... which means everything is wrong"
- "Task failed successfully! Error code: 42"

## 🎲 Random Behaviors

The language includes several random behaviors to keep you on your toes:
1. 10% chance of numbers becoming party emojis
2. 15% chance of variables going on vacation
3. 20% chance of everything working perfectly wrong
4. 30% chance of browser errors with style
5. Functions might return null or go for coffee

## 🛠️ Development

### Project Structure
- `src/lexer/`: Tokenizes source code
- `src/parser/`: Converts tokens into AST
- `src/ast.rs`: Abstract Syntax Tree definitions
- `src/interpreter.rs`: Executes code (incorrectly)

### Running Tests
```bash
cargo test
```

Note: Tests might fail successfully. That's a feature, not a bug!

## 🤝 Contributing

Why would you want to contribute to this? But if you insist:

1. Fork the repository
2. Create your feature branch
3. Make your changes
4. Submit a pull request
5. Regret your decisions

### Contribution Guidelines
- Make sure your code is as useless as possible
- Add more random behaviors
- Create more sarcastic error messages
- Break things in creative ways

## ⚠️ Warnings

- This language is not suitable for:
  - Production use
  - Development use
  - Any use whatsoever
- Side effects may include:
  - Confusion
  - Laughter
  - Existential crisis
  - Sudden urge to learn COBOL

## 📜 License

MIT License - Because even useless things need licenses.

## 🎉 Acknowledgments

- Inspired by all the programming languages that try to be useful
- Special thanks to everyone who said "this is a bad idea"
- Dedicated to developers who enjoy a good laugh

## 🤔 FAQ

**Q: Why did you create this?**
A: Why not?

**Q: Is it production-ready?**
A: It's not even development-ready.

**Q: Can I use this for serious projects?**
A: We admire your optimism, but no.

**Q: Does it have good error handling?**
A: It has the best error handling - everything is an error!

## 🐛 Known "Features"
- Everything works exactly as not intended
- Success is considered a bug
- If something works correctly, please report it as an issue
