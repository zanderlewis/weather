# Weather
An experimental programming language for weather calculations, trying to succeed FORTRAN.

## Table of Contents
- [Weather](#weather)
  - [Introduction](#introduction)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Syntax](#syntax)
  - [Examples](#examples)
  - [Contributing](#contributing)
  - [License](#license)

## Introduction
Weather is a programming language for weather calculations. It is designed to be simple and easy to use, while still being powerful enough to perform complex calculations. The language is inspired by FORTRAN, but aims to be more modern and user-friendly.

The world has a big issue, no one is learning FORTRAN anymore. This is a big problem because FORTRAN is the best language for weather calculations. Weather aims to solve this problem by providing a modern alternative to FORTRAN that is easier to learn and use.

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

