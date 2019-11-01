pub mod functions;
use crate::Dims;
use functions::DirectedBijectiveConnectionGraphFunctions;
use std::marker::PhantomData;

mod lemma1;
mod lemma2;
mod node_to_node;
mod node_to_set;
mod set_to_set;

pub struct DirectedBijectiveConnectionGraph<F: DirectedBijectiveConnectionGraphFunctions> {
    dimension: Dims,
    functions: PhantomData<F>,
}

impl<F> DirectedBijectiveConnectionGraph<F>
where
    F: DirectedBijectiveConnectionGraphFunctions,
{
    pub fn new(n_dim: Dims) -> Self {
        debug_assert_ne!(n_dim, 0);
        debug_assert!(n_dim <= 64);

        DirectedBijectiveConnectionGraph {
            dimension: n_dim,
            functions: PhantomData,
        }
    }
}
