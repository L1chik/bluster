use bluster::mesh::ObjectSet;
use crate::parameters::RunState;

pub trait ParametersPlugin {
    fn run_callbacks(
        &mut self,
        objects: &mut ObjectSet,
        state: &RunState,
    );
}