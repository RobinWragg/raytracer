// rwtodo "Rng" here is a trait. What is a trait? is "Write" a trait?
use glam::Vec3;
use rand::Rng; // Specified in the toml as "0.8.0", so it currently jumps to "0.8.5" put will never choose "0.9.0"
use std::vec::Vec;

struct Surface {
    // rwtodo only spheres for now
    position: Vec3,
    radius: f32,
}

impl Surface {
    fn intersects(&self, ray: &Ray) -> bool {
        // rwtodo rename intersection() and return an Option with point of intersection
        true
    }
}

struct Ray {
    direction: Vec3,
    position: Vec3,
}

fn trace_ray(ray: &Ray, surfaces: Vec<Surface>) {
    // rwtodo return photon intensity
    let mut intersecting_surfaces: Vec<&Surface> = Vec::new();
    for surface in surfaces.iter() {
        // rwtodo skip the originating surface of the ray
        if surface.intersects(ray) {
            intersecting_surfaces.push(surface);
        }
    }

    // rwtodo spawn ray(s) at the nearest surface
}

fn main() {
    println!("begin");
    let mut surfaces: Vec<Surface> = Vec::new(); // rwtodo figure out how to do this without mut, and how to move it to 'file' scope or whatever rust calls it
    surfaces.push(Surface {
        position: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    });

    let r = Ray {
        direction: Vec3::new(0.0, 0.0, 0.0),
        position: Vec3::new(0.0, 0.0, 0.0),
    };

    trace_ray(&r, surfaces);
    println!("end");
}

// rwtodo learn the ins and outs of specifying crate versions. e.g. rand
