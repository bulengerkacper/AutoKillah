use crate::world::graphic::Move;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Translation3, Vector3};
use rand::Rng;
use std::collections::LinkedList;
use std::path::Path;
// here are parts not to use in future

pub fn create_spaceship(window: &mut Window) -> SceneNode {
    let space_ship_obj = Path::new("./src/resources/spaceship/statek.obj");
    let mut space_ship = window.add_obj(
        &space_ship_obj,
        &space_ship_obj,
        Vector3::new(0.2, 0.2, 0.2),
    );
    space_ship.append_translation(&Translation3::new(0.0, -0.8, 2.0));
    space_ship.add_rotation_in_axis(3.15, 'y');
    let metal_texture = include_bytes!("../resources/textures/metal_texture.png");
    space_ship.set_texture_from_memory(metal_texture, "metal_texture");
    space_ship
}

pub fn generate_plantes(planet_number: u32, window: &mut Window) -> LinkedList<SceneNode> {
    let mut n = 0;
    let mut rng = rand::thread_rng();

    let mut planets = LinkedList::new();
    while n < planet_number {
        let mut sphere = window.add_sphere(0.1);
        let sphere_texture = include_bytes!("../resources/textures/sphere_texture.png");
        sphere.set_texture_from_memory(sphere_texture, "sphere_texture");
        sphere.append_translation(&Translation3::new(
            rng.gen_range(-3.0, 3.0),
            rng.gen_range(-2.0, 2.0),
            rng.gen_range(3.0, 6.0),
        ));
        n += 1;
        planets.push_back(sphere);
    }
    planets
}

pub fn move_planets(plane: &SceneNode, planets: &mut LinkedList<SceneNode>) {
    for planet in planets {
        planet.move_obj(0.0, 0.0, -0.01);
        if planet.data().local_transformation().translation.vector.z <= 0.5 {
            let mut rng = rand::thread_rng();
            planet.move_obj(
                rng.gen_range(-3.0, 3.0),
                rng.gen_range(-2.0, 2.0),
                rng.gen_range(3.0, 6.0),
            );
        }
        plane.detect_collision_with(planet);
    }
}
