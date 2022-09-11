//! The `monkeys` example explores the idea of the Shakespeare's monkeys also
//! known as the
//! [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem).
use pokablocks::animation::is_challenger_winner;
use pokablocks::*;
use num_bigint::{BigInt, BigUint, RandomBits};
use rand::Rng;

use genevo::{
  operator::prelude::*, population::ValueEncodedGenomeBuilder, prelude::*, types::fmt::Display,
};

#[derive(Debug)]
struct Parameter {
  population_size: usize,
  generation_limit: u64,
  num_individuals_per_parents: usize,
  selection_ratio: f64,
  num_crossover_points: usize,
  mutation_rate: f64,
  reinsertion_ratio: f64,
}

impl Default for Parameter {
  fn default() -> Self {
      Parameter {
          population_size: 100,
          generation_limit: 2000,
          num_individuals_per_parents: 2,
          selection_ratio: 0.7,
          num_crossover_points: 2,
          mutation_rate: 0.02, // (32 as f64).ln(),
          reinsertion_ratio: 0.7,
      }
  }
}

/// The phenotype
type Text = String;

/// The genotype
type BlockGenome = Vec<u8>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
  fn as_text(&self) -> Text;
}

impl AsPhenotype for BlockGenome {
  fn as_text(&self) -> Text {
    BigUint::from_bytes_be(&self).to_string()
  }
}

// impl Genotype for BlockGenome {
//   type Dna = Vec<u8>;
// }

// impl PartialEq for BlockGenome {
//   fn eq(&self, other: &Self) -> bool {
//     self.0 == other.0
//   }

//   fn ne(&self, other: &Self) -> bool {
//     self.0 != other.0
//   }
// }

/// The fitness function for `BlockGenome`s.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<BlockGenome, usize> for FitnessCalc {
  fn fitness_of(&self, genome: &BlockGenome) -> usize {
      let n = 100;
      let mut win_cnt = 0;

      let mut rng = rand::thread_rng();
      for _ in 0..n {

        let unsigned: BigUint = rng.sample(RandomBits::new(256));
        let defender_id = block_id_from_big_uint(&unsigned);

        let challenger_id: &[u8; 32] = &genome[0..32].try_into().unwrap();
        if is_challenger_winner(challenger_id, &defender_id, 100) {
          win_cnt += 1;
        }
      }
      // println!("{}", win_cnt);

      win_cnt

      // let fraction = win_cnt as f32 / n as f32;

      // (fraction * fraction * 100_00. + 0.5).floor() as usize
  }

  fn average(&self, fitness_values: &[usize]) -> usize {
      fitness_values.iter().sum::<usize>() / fitness_values.len()
  }

  fn highest_possible_fitness(&self) -> usize {
      100_00
  }

  fn lowest_possible_fitness(&self) -> usize {
      0
  }
}

fn main() {
  let params = Parameter::default();

  let initial_population: Population<BlockGenome> = build_population()
      .with_genome_builder(ValueEncodedGenomeBuilder::new(32, 0, 255))
      .of_size(params.population_size)
      .uniform_at_random();

  let mut monkeys_sim = simulate(
      genetic_algorithm()
          .with_evaluation(FitnessCalc)
          .with_selection(MaximizeSelector::new(
              params.selection_ratio,
              params.num_individuals_per_parents,
          ))
          .with_crossover(MultiPointCrossBreeder::new(params.num_crossover_points))
          .with_mutation(RandomValueMutator::new(params.mutation_rate, 0, 255))
          .with_reinsertion(ElitistReinserter::new(
              FitnessCalc,
              true,
              params.reinsertion_ratio,
          ))
          .with_initial_population(initial_population)
          .build(),
  )
  .until(or(
      FitnessLimit::new(FitnessCalc.highest_possible_fitness()),
      GenerationLimit::new(params.generation_limit),
  ))
  .build();

  println!("Starting Shakespeare's Monkeys with: {:?}", params);

  loop {
      let result = monkeys_sim.step();
      match result {
          Ok(SimResult::Intermediate(step)) => {
              let evaluated_population = step.result.evaluated_population;
              let best_solution = step.result.best_solution;
              println!(
                  "Step: generation: {}, average_fitness: {}, \
                   best fitness: {}, duration: {}, processing_time: {}",
                  step.iteration,
                  evaluated_population.average_fitness(),
                  best_solution.solution.fitness,
                  step.duration.fmt(),
                  step.processing_time.fmt()
              );
              println!("      {}", best_solution.solution.genome.as_text());
              //                println!("| population: [{}]", result.population.iter().map(|g| g.as_text())
              //                    .collect::<Vec<String>>().join("], ["));
          },
          Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
              let best_solution = step.result.best_solution;
              println!("{}", stop_reason);
              println!(
                  "Final result after {}: generation: {}, \
                   best solution with fitness {} found in generation {}, processing_time: {}",
                  duration.fmt(),
                  step.iteration,
                  best_solution.solution.fitness,
                  best_solution.generation,
                  processing_time.fmt()
              );
              println!("      {}", best_solution.solution.genome.as_text());
              break;
          },
          Err(error) => {
              println!("{}", error);
              break;
          },
      }
  }
}