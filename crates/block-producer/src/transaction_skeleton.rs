use gw_types::{
    bytes::Bytes,
    packed::{CellDep, CellInput, CellOutput, WitnessArgs},
};

#[derive(Default)]
pub struct TransactionSkeleton {
    inputs: Vec<CellInput>,
    cell_deps: Vec<CellDep>,
    witnesses: Vec<WitnessArgs>,
    cell_outputs: Vec<(CellOutput, Bytes)>,
}

impl TransactionSkeleton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inputs(&mut self) -> &mut Vec<CellInput> {
        &mut self.inputs
    }

    pub fn cell_deps(&mut self) -> &mut Vec<CellDep> {
        &mut self.cell_deps
    }

    pub fn outputs(&mut self) -> &mut Vec<(CellOutput, Bytes)> {
        &mut self.cell_outputs
    }

    pub fn witnesses(&mut self) -> &mut Vec<WitnessArgs> {
        &mut self.witnesses
    }
}
