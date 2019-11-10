# bbte_optim_tzim1773_genetic
Genetic (like) algortihm implemented with generics by VaranTavers

This library was only meant for personal use, so it's probably worse than the other available options, however you can have fun trying it out.

This library is available on crates.io, you can include it in a cargo based project with:
```toml
[dependencies]                                                                    
bbte_optim_tzim1773_genetic = "0.1.0"
```

This project is subject to interface changes, user caution is advised while using "^0.1.0" notation.

Example of usage:

(We try to maximise the function: 5 - x^2) 
```rust
use rand::prelude::*;
use bbte_optim_tzim1773_genetic::Genetic;

fn main() {
    let agent = || {
        let mut rng = thread_rng();
        rng.gen_range(-5.0, 5.0)
    };
    
    let fit = |a: &f64| 5.0 - a * a;
    
    let muta = |a: &f64| {
        let mut rng = thread_rng();
        *a + rng.gen_range(-0.01, 0.01)
    };
    
    let off = |a: &f64, b: &f64| (*a + *b) / 2.0;
    
    let test: Genetic<f64> = Genetic {
        population: 100,
        max_generation: 20,
        pc: 0.5,
        pm: 0.4,
        get_random_agent: &agent,
        f_fitness: &fit,
        f_mutate: &muta,
        f_offspring: &off,
    };
    
    let simul = test.run();
    let best = test.get_best(&simul);
    
    println!("{}", simul[best]); // should be a number close to 0
}

```

Genetic<T> is defined as:
```rust
pub struct Genetic<'a, T> {
    /// Population size: with increased size comes increased accuracy but decreased speed
    pub population: usize,
    /// Max generation: with increased generation comes increased accuracy but decreased speed
    /// Depends on the complexity of the task. Bigger tasks require more generations.
    pub max_generation: usize,
    /// Probability of crossover ((never) 0.0 <= pc <= 1.0 (always))
    pub pc: f64,
    /// Probability of mutation ((never) 0.0 <= pm <= 1.0 (always))
    pub pm: f64,
    /// Function that returns one agent which is used in the 0th generation
    /// You can start from a given point, or use a random generator like the rand crate
    pub get_random_agent: &'a dyn Fn()->T,
    /// Function that evaluates an agent and returns it's fitness (this algorithm maximises this function)
    pub f_fitness: &'a dyn Fn(&T) -> f64,
    /// Function that mutates an agent and returns the mutated version of it
    pub f_mutate: &'a dyn Fn(&T) -> T,
    /// Function that crossovers two agents and creates an offspring
    pub f_offspring: &'a dyn Fn(&T, &T) -> T,

}

```
