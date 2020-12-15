# tsptw
ddo-based solver for the travelling salesman problem with time window

## Pre requisites
To compile the solver, you will need a working toolchain for Rust. Luckily, 
it is a no brainer to set up. For that installation, you are redirected to 
the official rust documentation website

https://www.rust-lang.org/learn/get-started

## Installation
Once you have a working rust toolchain installed, all you need to do to 
compile the tsptw solver is to run the following command:

```
cargo build --release
```

Your binary will be located in `$project/target/release/tsptw`.

## Solving an instance
Basically, all you need to do to solve an instance is to launch the following 
command: 
```
tsptw <instance>
```

In case you need more help (or I add exrtra options), the tool provides some 
built in help (`tsptw -h`).
