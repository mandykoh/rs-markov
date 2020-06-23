//! # markov
//!
//! `markov` is a library for training Markov models and using them for
//! prediction and generation.

mod accumulator;
mod generator;
mod model;
mod predictor;
mod sequence;
mod table;

pub use self::accumulator::Accumulator;
pub use self::generator::Generator;
pub use self::model::Model;
pub use self::predictor::Predictor;

use self::sequence::Sequence;
use self::table::Table;
