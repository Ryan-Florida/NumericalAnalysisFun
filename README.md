# Solving the heat equation using the finite difference method in Rust.

This is just a short numerical analysis project I did to gain a bit of familiarity with Rust.

## Prerequisites

This project does require that you have some means by which to run Rust code and, if you want to visualize the results,
a working install of Python3 with, at least, numpy and matplotlib installed (I recommend just using Anaconda).

## Installing
```
$ curl -sf -L https://static.rust-lang.org/rustup.sh | sh && curl -O https://repo.continuum.io/archive/Anaconda3-5.0.1-Linux-x86_64.sh && source ~/.bashrc

```

## Running the program

From the command line, perform one of the following steps:
* Without visualization
```
cargo clean && cargo build && cargo run
```
* With visualization
```
cargo clean && cargo build && cargo run | hMap.py
```
**Note that you do not have to clean and build every time you want to run.**


## Author

* **Ryan Florida**

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.