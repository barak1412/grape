#![feature(impl_trait_in_assoc_type)]

mod utils;
pub use utils::*;
mod graph_embedder;
pub use graph_embedder::*;
mod node2vec;
pub use node2vec::*;
mod walk_transformer;
pub use walk_transformer::*;
mod cbow;
mod skipgram;