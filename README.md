# Q'
Q' (Q Prime) is an experimental programming language for quantum and weather calculations, formally known as Weather.

## Table of Contents
- [Q'](#q)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Syntax](#syntax)
  - [Examples](#examples)
  - [Contributing](#contributing)
  - [License](#license)
- [Constants](docs/constants.md)
- [Functions](docs/functions.md)

## Introduction
Q' is a programming language for quantum and weather calculations. It is designed to be simple and easy to use, while still being powerful enough to perform complex calculations. The language is inspired by FORTRAN and Q#, but aims to be more modern and user-friendly.

## Features
1. **High Precision**: Q' values high precision calculations for accurate quantum and weather calculations. Q' uses BigInts and BigRationals to ensure that calculations are as accurate as possible. We even have a built-in Pi constant with 100 digits of precision!
2. **Built-in Functions**: Q' has a few built-in functions for conversions and calculations. These include functions for calculating the dew point, converting between different temperature units, and more.
3. **Built-in Constants**: Q' has many built-in constants that can be used in calculations. These include constants such as the Kelvin constant, Pi, the gas constant for dry air, and more.
4. **Simple Syntax**: Q' has a simple and easy-to-understand syntax that is near identical to Python. This makes it easy for beginners to learn and use the language.
5. **Dynamic Typing**: Q' is dynamically typed, meaning that you don't need to specify the type of a variable when you declare it. This makes the language more flexible and easier to use.
6. **Quick Interpreted Language**: Q' is a quick interpreted language, meaning that you can write and run code quickly without needing to compile it first. This makes it easy to test and debug code.

## Installation
Q' is still in development and is not yet ready for general use. However, you can try it out by cloning the repository and running the code in the `src` directory using Cargo:

```bash
git clone https://github.com/zanderlewis/qprime
cd qprime
cargo run script.qpr
```

or, you can install via Cargo:

```bash
cargo install qprime
```

## Usage
To use Q', write your code in a file with a `.qpr` extension and run it using the Q' interpreter. For example, to run a script called `script.qpr`, you would use the following command:

```bash
cargo run script.qpr
```

or, if you installed Q' via Cargo:

```bash
qprime script.qpr
```

## Syntax
The syntax of Q' is similar to that of Python, with some improvements for quantum/weather applications. Here is an example:

```qpr
# Assign values to variables
temp = 0
humidity = 7
fahrenheit = 86
celsius = 30

# Calculate and print the dew point
dew_point = dewpoint(temp, humidity)
print("Dew Point: ")
print(dew_point)

# Convert Fahrenheit to Celsius and print the result
celsius_converted = ftoc(fahrenheit)
print("Fahrenheit to Celsius: ")
print(celsius_converted)

# Convert Celsius to Fahrenheit and print the result
fahrenheit_converted = ctof(celsius)
print("Celsius to Fahrenheit: ")
print(fahrenheit_converted)

# Initialize 2 qubits
q1 = qubit(0, 1) # State of |0>, 1 qubit
q2 = qubit(1, 1) # State of |1>, 1 qubit

# Apply Hadamard gate to qubit q1
h = hadamard(q1)

# Apply Pauli-X gate to qubit q1
x = pauli_x(q1)

# Apply CNOT gate to qubits q1 and q2
_cnot = cnot(q1, q2)

# Measure the CNOT gate
m = measure(_cnot)

# Print the result of the measurement
print(m)
print(q1)
print(q2)
```

## Examples
[script.qpr](examples/import/script.qpr) and [quantum.qpr](examples/quantum.qpr) are good places to start.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you would like to contribute to Q'.

## License
Q' is licensed under the Apache-2.0 License. See [LICENSE](LICENSE) for more information.
