// rwtodo "Rng" here is a trait. What is a trait? is "Write" a trait?
use glam::Vec3;
use rand::Rng;
use show_image::{create_window, ImageInfo, ImageView};
use std::vec::Vec;

// rwtodo can I put this declaration inside Surface?
enum SurfaceReflectivity {
    Light,
    Rough,
}

// rwtodo this will be come a SphereSurface, then we can wrap it in a Surface enum (Sphere, Triangle etc)
struct Surface {
    position: Vec3,
    radius: f32,
    reflectivity: SurfaceReflectivity,
}

impl Surface {
    // rwtodo i think I actually want to return the world pos of the intersection?
    fn intersection(&self, ray: &Ray) -> Option<Vec3> {
        // Project the sphere's position onto the ray direction
        let sphere_relative_pos = self.position - ray.position;
        let projection = ray.direction.dot(sphere_relative_pos);

        if projection < 0.0 {
            return None;
        }

        // Check the distance of the ray-relative position of projection against the radius
        let projection_relative_pos = ray.direction * projection;
        let projection_distance = projection_relative_pos.distance(sphere_relative_pos);
        if projection_distance < self.radius {
            // Find intersection position
            // x^2 + y^2 = r^2
            // therefore y = sqrt(radius^2 - proj^2)
            let y = (self.radius * self.radius - projection_distance * projection_distance).sqrt();
            let relative_intersection_pos = ray.direction * (projection - y);
            Some(ray.position + relative_intersection_pos)
        } else {
            None
        }
    }
}

struct Ray {
    direction: Vec3, // rwtodo can I do something clever to enforce normalization?
    position: Vec3,
    originating_surface: usize,
}

fn trace_ray(ray: &Ray, surfaces: &[Surface]) -> f32 {
    // rwtodo I could combine these into an Option<struct> but I need to understand lifetime specifiers first.
    let mut closest_intersection = Vec3::ZERO;
    let mut closest_intersecting_surface: Option<usize> = None;

    // rwtodo if i can figure out how to add a Surface reference to the Ray, I can do a normal forloop overt surfaces.iter().
    for s in 0..surfaces.len() {
        // rwtodo skip the originating surface of the ray. how about having a Surface reference in the Ray struct?
        // rwtodo not choosing the closest surface yet
        if let Some(intersection) = surfaces[s].intersection(ray) {
            if s == ray.originating_surface {
                continue;
            }

            closest_intersecting_surface = Some(s);
            closest_intersection = intersection;
        }
    }

    // Spawn ray(s) at the nearest surface
    if let Some(s) = closest_intersecting_surface {
        match surfaces[s].reflectivity {
            SurfaceReflectivity::Light => 2.0,
            SurfaceReflectivity::Rough => {
                let normal = (closest_intersection - surfaces[s].position).normalize(); // rwtodo refactor this into the Surface struct
                let mut sum = 0.0;
                for _n in 0..1000 {
                    let mut r = random_point_on_sphere(); // rwtodo use random_point_on_sphere_2
                    if r.dot(normal) < 0.0 {
                        r *= -1.0;
                    }
                    let ray = Ray {
                        direction: r,
                        position: closest_intersection,
                        originating_surface: s,
                    };
                    sum += trace_ray(&ray, surfaces); // rwtodo spawn a random ray in the hemisphere of the surface normal.
                                                      // rwtodo spawn multiple rays.
                }
                sum / 500.0
            }
        }
    } else {
        0.0 // rwtodo 0.0
    }
}

// rwtodo make hemisphere style
fn random_point_on_sphere() -> Vec3 {
    // rwtodo I'm not fond of this brute force approach
    loop {
        let v = Vec3::new(
            rand::random::<f32>() * 2.0 - 1.0,
            rand::random::<f32>() * 2.0 - 1.0,
            rand::random::<f32>() * 2.0 - 1.0,
        );

        let len = v.length();
        if len > 0.001 && len < 1.0 {
            return v.normalize(); // rwtodo can't do the shorthand return here and I don't know why
        }
    }
}

fn random_point_on_sphere_2() -> Vec3 {
    let mut rng = rand::thread_rng();

    // Generate two random numbers for spherical coordinates
    let theta = rng.gen_range(0.0..std::f32::consts::TAU);
    let phi = rng.gen_range(0.0..std::f32::consts::PI);

    // Calculate the Cartesian coordinates
    let x = phi.sin() * theta.cos();
    let y = phi.sin() * theta.sin();
    let z = phi.cos();

    // Create a Vec3 from the Cartesian coordinates
    Vec3::new(x, y, z)
}

#[show_image::main]
fn main() {
    println!("begin");
    let surfaces = vec![
        Surface {
            position: Vec3::new(10.0, -10.0, 30.0),
            radius: 20.0,
            reflectivity: SurfaceReflectivity::Rough,
        },
        Surface {
            position: Vec3::new(-10.0, 5.0, 5.0),
            radius: 10.0,
            reflectivity: SurfaceReflectivity::Light,
        },
    ];

    let mut pixel_data = Vec::new();

    let width = 64;
    let height = 64;

    let mut starting_rays: Vec<Ray> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            starting_rays.push(Ray {
                direction: Vec3::new(0.0, 0.0, 1.0),
                position: Vec3::new(
                    x as f32 - ((width as f32) / 2.0),
                    -(y as f32) + ((height as f32) / 2.0),
                    0.0,
                ),
                originating_surface: 1000000000,
            });
        }
    }

    for starting_ray in starting_rays {
        let intensity = trace_ray(&starting_ray, &surfaces);
        pixel_data.push((intensity * 255.0) as u8);
    }

    let image = ImageView::new(ImageInfo::mono8(width, height), &pixel_data);
    let window = create_window("image", Default::default()).expect("Couldn't create window");
    window
        .set_image("image-001", image)
        .expect("Couldn't set image");

    std::thread::sleep(std::time::Duration::new(2, 0));

    println!("end");
}

// rwtodo learn the ins and outs of specifying crate versions. e.g. rand
