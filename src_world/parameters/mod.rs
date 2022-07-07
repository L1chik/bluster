use bluster::prelude::ObjectSet;

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
}

pub struct Harness {
    pub objects: ObjectSet,
    pub state: RunState,
}

impl Harness {
    pub fn new_empty() -> Self {
        Harness {
            state: RunState::new(),
            objects: ObjectSet::new(),
        }
    }

    pub fn init_world(&mut self, objects: ObjectSet) {
        self.objects = objects;

        self.state.timestep_id = 0;
        self.state.time = 0.0;
    }
}