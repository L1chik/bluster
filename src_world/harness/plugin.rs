use bluster::mesh::ObjectSet;
use crate::harness::RunState;

pub trait HarnessPlugin {
    fn run_callbacks(
        &mut self,
        objects: &mut ObjectSet,
        state: &RunState,
    );
}