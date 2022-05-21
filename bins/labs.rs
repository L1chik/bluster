use world::{World, WorldApp};

mod robot;

fn main() {
    let mut builders: Vec<(_, fn(&mut World))> = vec![
        ("Ferbot", robot::init_world),
    ];

    let test = WorldApp::from_builders(0, builders);
    test.run()
}