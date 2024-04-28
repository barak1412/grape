#![feature(is_sorted)]
#![feature(core_intrinsics)]
#![feature(sync_unsafe_cell)]
#![feature(exclusive_range_pattern)]

/*#![warn(unused_macros)]
#![feature(iter_advance_by)]
#![feature(impl_trait_in_assoc_type)]
#![feature(string_remove_matches)]
#![feature(exit_status_error)]
#![feature(pattern)]
#![deny(unconditional_recursion)]
#![type_length_limit = "3764086"]*/
pub mod utils;
pub use utils::*;

mod constructors;
pub use constructors::*;
mod operators;

mod getters;
mod getters_boolean;
mod getters_cached;
mod iters;
mod nodes_sampling;
mod iter_queries;
mod queries;
mod queries_boolean;
mod dijkstra;
pub use self::dijkstra::*;
mod dijkstra_queue;
pub use dijkstra_queue::*;
mod parameters_validators;

mod hash;
mod distributions;
pub mod walks_parameters;
mod walks;
mod queries_walk;
mod trees;
mod types;
pub use types::*;
mod cache;
use cache::*;
mod vocabulary;
pub use self::vocabulary::*;
mod node_type_vocabulary;
pub use self::node_type_vocabulary::*;
mod edge_type_vocabulary;
pub use self::edge_type_vocabulary::*;
mod graph;
pub use graph::Graph;
mod builder;
pub use builder::*;