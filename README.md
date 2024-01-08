# Generic Runge-kutta integration of dynamical systems

Different runge-kutta integration methods can be expressed 
through "Butcher-Tableaus". This program aims to integrate 
a double pendulum with various runge-kutta integration methods
in order to campare the results.

## for people unfamiliar with Rust
TL;DR:
```bash
$ cargo run --release
```

Go into the project folder, compile the project with ```$ cargo build --release``` ("release" means *with* optimizations). The binary is placed into the newly created folder '/target/release/'. You can also run the code with ```$ cargo run --release```.

This code contains the following:
- a function that does runge kutta integration for a generic type with the constraints/traits "dynamical system"
- a "trait" which defines the interface/methods/associated types a type must implement in order to be usable by the runge-kutta-method. The trait is defined like this:
    - it defines an associated type "StateT" that holds the system's state (like position, momentum, etc)
    - it defines an associated type "ParamT" that holds a specific system's (fixed) parameters like the masses, lengths and gravitational constant of a specific double pendulum system. 
    - if defines a function "f" that takes the current state and the systems parameters and calculates the derivative. It is the function f in the typical equation "x'(t) = f(t, x'(t))
- a struct for the butcher-tableau which defines the specific runge-kutta-methods
- a struct "Integration" that holds the state of the dynamical system, the parameters, the stepsize and the handle to the output file and a string with the name of the output file. It also has methods for performing one integration step and writing the state into the file.

Put together everything works like this:
The 'Integration'-struct is instantiated 6 times with various integration methods, then the integration is perfomed for many steps. Not every step is written into the file to save space.

## After running the program
In order to view the results a python script "plot.py" is provided. Used as is it plots the trajectories of every integration method. It can also be used with the option "-f" of "-files" in order to only plot specific files. Use like this "./plot.py -f 0 3" to only plot the first and forth file's contents.


## Todos
- stepsize control
    -> needs a function that measures distance (or distance squared) of a system's state.
- curve simplification to write only necessary points of trajectories i.e. compression.