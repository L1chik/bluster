pub trait ParametersPlugin {
    fn run_callbacks(
        &mut self,
        events: &Events,
        state: &RunState,
    );
}