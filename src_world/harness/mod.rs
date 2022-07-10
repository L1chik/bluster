use bluster::prelude::ObjectSet;
use plugin::HarnessPlugin;
pub mod plugin;

pub struct RunState {
    pub timestep_id: usize,
    pub time: f32,
}

impl RunState {
    pub fn new() -> Self {
        RunState {
            timestep_id: 0,
            time: 0.0,
        }
    }

    pub fn num_threads(&self) -> usize {
        1
    }
}

pub struct Harness {
    pub objects: ObjectSet,
    pub state: RunState,
    plugins: Vec<Box<dyn HarnessPlugin>>
}

impl Harness {
    pub fn new_empty() -> Self {
        let state = RunState::new();

        Harness {
            state,
            objects: ObjectSet::new(),
            plugins: Vec::new(),
        }
    }

    pub fn new(objects: ObjectSet) -> Self {
        let mut result = Self::new_empty();
        result.init_world(objects);

        result
    }

    pub fn init_world(&mut self, objects: ObjectSet) {
        self.objects = objects;

        self.state.timestep_id = 0;
        self.state.time = 0.0;
    }
}