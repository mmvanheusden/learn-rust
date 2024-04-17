## **Learning Rust**  
Every piece of leaning code is in the `Experiments` folder. And are called Experiments.  
I use the [Rust programming Book](https://doc.rust-lang.org/book/) as a reference. Note 'reference', I merely use this book as a reference, I don't strictly follow it. You'll see me adding other stuff from time to time as well.

To run the code, first install Rust.  
Then run the code: `./run.sh <experiment_name>`.
For example: `./run.sh HelloWorld` will run the HelloWorld experiment.

```text
$ ./run.sh HelloWorld
 Clearing old build...
 Compiling HelloWorld.rs...
 Running HelloWorld...

 Hello, world!
```

### Creating a new experiment
To create a new experiment, run `./newExeriment.sh <experiment_name>`.  
Make sure Cargo.toml stays tidy as the script will append the new experiment to it.  

### Motivation
I'm learning Rust because I want to learn a new language.  
I see C being used a lot, and I want to learn a language that is similar to C, but with more modern features.  
I also want to learn a language that has a good package manager and build system, and has a vibrant community.
Rust seems to mark all the boxes.


## Resources:

* [The Rustonomicon](https://doc.rust-lang.org/nomicon/)  
  *The Rustonomicon digs into all the awful details that you need to understand when writing Unsafe Rust programs.*
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)  
  *Rust by Example (RBE) is a collection of runnable examples that illustrate various Rust concepts and standard libraries.*
* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)  
    *The Rust Programming Language is the official book about Rust, made by everyone.*
* [Rustlings](https://rustlings.cool/)  
  *Small exercises to get used to reading, writing and debugging Rust code.*