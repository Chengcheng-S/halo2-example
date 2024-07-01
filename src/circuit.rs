use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
};

#[derive(Clone, Copy)]
pub struct MyCircuit;

impl<F: Field> Circuit<F> for MyCircuit {
    type Config = ();

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        *self
    }

    fn configure(_meta: &mut ConstraintSystem<F>) -> Self::Config {}

    fn synthesize(&self, _config: Self::Config, _layouter: impl Layouter<F>) -> Result<(), Error> {
        Ok(())
    }
}
