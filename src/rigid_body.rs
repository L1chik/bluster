#[derive(Debug, Clone)]
pub struct RigidBody {
    // pub pos: RigidBodyPosition,
    // pub ids: RigidBodyIds,
    pub user_data: u128,
}

impl RigidBody {
    fn new() -> Self {
        RigidBody {
            user_data: 0,
        }
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::new()
    }
}