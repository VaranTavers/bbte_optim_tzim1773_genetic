#![crate_name = "bbte_optim_tzim1773_genetic"]
use rand::prelude::*;

pub struct Genetic<'a, T> {
    /// Population size: with increased size comes increased accuracy but decreased speed
    /// Suggested value: 100
    pub population: usize,
    /// Max generation: with increased generation comes increased accuracy but decreased speed
    /// Suggested value: 1000
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

impl<'a, T> Genetic<'a, T>
    where T: Clone {
    fn get_population(&self) -> Vec<T> {
        vec![0; self.population]
            .iter()
            .map(|&_| (self.get_random_agent)())
            .collect::<Vec<T>>()
    }

    fn generate_parents(&self, xg: &'a Vec<T>) -> Vec<(&T, &T)> {
        let l = xg.len();
        let mut rng = thread_rng();

        let p = xg.iter()
            .map(|_| {
                let x = rng.gen_range(0, l);
                let mut y = rng.gen_range(0, l);
                while y == x {
                    y = rng.gen_range(0, l);
                }
                &xg[y]
            })
        .collect::<Vec<&T>>();

        xg.iter()
            .zip(p.iter())
            .map(|(a,b)| (a, *b))
            .collect::<Vec<(&T, &T)>>()
    }

    /// Returns the index of the best agent from a vector of agents
    ///
    /// # Arguments:
    ///
    /// * `u` a vector of agents
    /// 
    /// # Examples:
    ///  See at run()

    pub fn get_best(&self, u: &Vec<T>) -> usize {
        let mut best_i = 0;
        let mut f_best = (self.f_fitness)(&u[0]);

        for (i, x) in u.iter().enumerate() {
            let f_x = (self.f_fitness)(&x);
            if f_x > f_best {
                best_i = i;
                f_best = f_x;
            }
        }

        best_i
    }

    fn mutate(&self, xg: &Vec<T>) -> Vec<T> {
        let mut rng = thread_rng();

        xg.iter()
            .map(|x| {
                if rng.gen_range(0.0, 1.0) >= self.pm {
                    return x.clone();
                }
                (self.f_mutate)(x)
            })
        .collect::<Vec<T>>()
    }

    fn selection(&self, xg: &mut Vec<T>) -> Vec<T> {
        let mut new_generation = Vec::new();

        for _i in 0..self.population {
            let best_i = self.get_best(xg);
            new_generation.push(xg.remove(best_i));
        }

        new_generation
    }

    /// Returns agents from the given generation.
    ///
    /// # Arguments:
    ///
    /// * `u` a vector of agents
    /// 
    /// # Examples:
    /// ```
    /// use rand::prelude::*;
    /// use bbte_optim_tzim1773_genetic::Genetic;
    ///
    /// fn main() {
    ///    let agent = || 123;
    ///    let fit = |_a: &usize| 1.0;
    ///    let muta = |a: &usize| *a + 1;
    ///    let off = |a: &usize, b: &usize| (*a + *b) / 2;
    ///    let test: Genetic<usize> = Genetic {
    ///        population: 10,
    ///        max_generation: 1,
    ///        pc: 0.5,
    ///        pm: 1.0,
    ///        get_random_agent: &agent,
    ///        f_fitness: &fit,
    ///        f_mutate: &muta,
    ///        f_offspring: &off,
    ///    };
    ///    
    ///    let pop = test.run();
    ///    println!("{}", pop[0]); // since all agents are mutated (pm = 1.0)
    ///                             // all agents should hold the value 124
    /// }
    /// ```
    ///
    /// Maximising the -x^2 + 5 function:
    /// ```
    /// use rand::prelude::*;
    /// use bbte_optim_tzim1773_genetic::Genetic;
    ///
    /// fn main() {
    ///     let agent = || {
    ///         let mut rng = thread_rng();
    ///         rng.gen_range(-5.0, 5.0)
    ///     };
    ///     let fit = |a: &f64| 5.0 - a * a;
    ///     let muta = |a: &f64| {
    ///         let mut rng = thread_rng();
    ///         *a + rng.gen_range(-0.01, 0.01)
    ///     };
    ///     let off = |a: &f64, b: &f64| (*a + *b) / 2.0;
    ///     let test: Genetic<f64> = Genetic {
    ///         population: 100,
    ///         max_generation: 20,
    ///         pc: 0.5,
    ///         pm: 0.4,
    ///         get_random_agent: &agent,
    ///         f_fitness: &fit,
    ///         f_mutate: &muta,
    ///         f_offspring: &off,
    ///     };
    ///
    ///     let simul = test.run();
    ///     let best = test.get_best(&simul);
    ///
    ///     println!("{}", simul[best]); // should be a number close to 0
    /// }
    /// ```
    pub fn run(&self) -> Vec<T> {
        let mut xg:Vec<T> = self.get_population(); 
        let mut rng = thread_rng();

        for _g in 0..self.max_generation {
            let parents = &self.generate_parents(&xg);
            let mut population = xg.clone();
            for (a, b) in parents {
                if rng.gen_range(0.0, 1.0) < self.pc {
                    population.push((self.f_offspring)(&a, &b));
                }
            }
            let mut mutated = self.mutate(&population);

            xg = self.selection(&mut mutated);
        }

        xg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_agent() {
        let agent = || 123;
        let fit = |_a: &usize| 1.0;
        let muta = |a: &usize| *a + 1;
        let off = |a: &usize, b: &usize| (*a + *b) / 2;
        let test: Genetic<usize> = Genetic {
            population: 10,
            max_generation: 10,
            pc: 0.5,
            pm: 0.5,
            get_random_agent: &agent,
            f_fitness: &fit,
            f_mutate: &muta,
            f_offspring: &off,
        };
        
        let pop = test.get_population();
        assert_eq!(pop[0], 123);
    }
    
    #[test]
    fn correct_mutation() {
        let agent = || 123;
        let fit = |_a: &usize| 1.0;
        let muta = |a: &usize| *a + 1;
        let off = |a: &usize, b: &usize| (*a + *b) / 2;
        let test: Genetic<usize> = Genetic {
            population: 10,
            max_generation: 1,
            pc: 0.5,
            pm: 1.0,
            get_random_agent: &agent,
            f_fitness: &fit,
            f_mutate: &muta,
            f_offspring: &off,
        };
        
        let pop = test.run();
        assert_eq!(pop[0], 124);    
    }
    
    #[test]
    fn correct_crossover() {
        let agent = || 121;
        let fit = |a: &usize| 10.0 - (*a as f64 - 244.0).abs();
        let muta = |a: &usize| *a + 2;
        let off = |a: &usize, b: &usize| (*a + *b);
        let test: Genetic<usize> = Genetic {
            population: 2,
            max_generation: 1,
            pc: 1.0,
            pm: 1.0,
            get_random_agent: &agent,
            f_fitness: &fit,
            f_mutate: &muta,
            f_offspring: &off,
        };
        
        let pop = test.run();
        assert_eq!(pop[0], 244);
    }
    
    #[test]
    fn convergence() {
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
        assert!((simul[best]).abs() < 1.0);
    }
}
