# Motivation
I have used DMenu and Rofi in dmenu mode for a while as my main command launchers (with scripts around them),
but I've found that they don't give me enough control about how the output is filtered.
This is my attempt at creating my own launcher menu - one that gives me full control about what is displayed.

# Adding as a dependency
This is a library so you will need to create a new binary project using Cargo.
You can call it something like `my_rmenu`.
Then you will need to add `rmenu` as a dependency.
At the last line of Cargo.toml, add
```
rmenu = { version = "*", git = "https://www.github.com/SuperCuber/rmenu" }
```

# Examples
Examples can be found in the `examples` folder.
You can probably understand how to use the library without even being a Rust guru if you read the documentation.

# Documentation
(Documentation)[]
