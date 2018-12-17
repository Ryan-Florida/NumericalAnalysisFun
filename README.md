# Solving the heat equation using the finite difference method in Rust.
This is just a short numerical analysis project I did to gain a bit of familiarity with Rust. The examples were all taken from 
*Numerical Solution of Partial Differential Equations - Finite Difference Methods 3rd edition* by G.D. Smith.
## Prerequisites
This project does require that you have some means by which to run Rust code and, if you want to visualize the results,
a working install of Python3 with, at least, numpy and matplotlib installed (I recommend just using Anaconda).
## Installing
* Rust
```
$ curl -sf -L https://static.rust-lang.org/rustup.sh | sh
```
* Python (Anaconda)
```
$ curl -O https://repo.continuum.io/archive/Anaconda3-5.0.1-Linux-x86_64.sh && source ~/.bashrc
```
## Running the program
From the command line, perform one of the following steps:
* With visualization
```
cargo clean && cargo build && cargo run | ./hMap.py
```
* Without visualization
```
cargo clean && cargo build && cargo run
```
**Note that you do not have to clean and build every time you want to run.**
## Author
* **Ryan Florida**
