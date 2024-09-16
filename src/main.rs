mod vector3;
mod matrix;
mod rigid_body;
mod game_world;
mod constraint;

use crate::game_world::GameWorld;
use crate::rigid_body::RigidBody;
use crate::vector3::Vector3;

const NUM_BODIES: usize = 100;

fn main() {
    
    let mut bodies: Vec<RigidBody> = vec![];

    for i in 0..NUM_BODIES {

        let pos_vec = Vector3::new(0.0 + i as f32, 0.0 + i as f32, 0.0 + i as f32);
        let vel_vec = Vector3::new(0.0 + i as f32, 0.0 + i as f32, 0.0 + i as f32);
        
        // Create a RigidBody with mass and inertia, and add to bodies vector
        let body = RigidBody::new(100.0, 100.0, pos_vec, vel_vec);
        bodies.push(body);
    }

    let mut system = GameWorld::new();
    println!("Hello, world!");
}
