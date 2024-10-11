# Weather
An experimental programming language for weather calculations, trying to succeed FORTRAN.

## Table of Contents
- [Weather](#weather)
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
Weather is a programming language for weather calculations. It is designed to be simple and easy to use, while still being powerful enough to perform complex calculations. The language is inspired by FORTRAN, but aims to be more modern and user-friendly.

The world has a big issue, no one is learning FORTRAN anymore. This is a big problem because FORTRAN is the best language for weather calculations. Weather aims to solve this problem by providing a modern alternative to FORTRAN that is easier to learn and use.

## Features
1. **High Precision**: Weather values high precision calculations for accurate weather calculations. Weather uses BigInts and BigRationals to ensure that calculations are as accurate as possible. We even have a built-in Pi constant with 100 digits of precision!
2. **Built-in Functions**: Weather has a few built-in functions for conversions and calculations. These include functions for calculating the dew point, converting between different temperature units, and more.
3. **Built-in Constants**: Weather has many built-in constants that can be used in calculations. These include constants such as the Kelvin constant, Pi, the gas constant for dry air, and more.
4. **Simple Syntax**: Weather has a simple and easy-to-understand syntax that is near identical to Python. This makes it easy for beginners to learn and use the language.
5. **Dynamic Typing**: Weather is dynamically typed, meaning that you don't need to specify the type of a variable when you declare it. This makes the language more flexible and easier to use.
6. **Quick Interpreted Language**: Weather is a quick interpreted language, meaning that you can write and run code quickly (with the help of `rayon` in the backend) without needing to compile it first. This makes it easy to test and debug code.

## Installation
Weather is still in development and is not yet ready for general use. However, you can try it out by cloning the repository and running the code in the `src` directory using Cargo:

```bash
git clone https://github.com/zanderlewis/weather
cd weather
cargo run script.wthr
```

## Usage
To use Weather, write your code in a file with a `.wthr` extension and run it using the Weather interpreter. For example, to run a script called `script.wthr`, you would use the following command:

```bash
cargo run script.wthr
```

## Syntax
The syntax of Weather is similar to that of Python, with some improvements for weather applications. Here is an example:

```wthr
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
```

## Examples
[script.wthr](script.wthr) is a good place to start.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you would like to contribute to Weather.

## License
Weather is licensed under the Apache-2.0 License. See [LICENSE](LICENSE) for more information.

