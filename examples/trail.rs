extern crate kiss3d;
extern crate nalgebra as na;

use std::time::{Duration, SystemTime};

use na::Point3;

use kiss3d::camera::{Camera, FirstPerson};
use kiss3d::event::Key;
use kiss3d::light::Light;
use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Kiss3d: lines");

    window.set_light(Light::StickToCamera);

    let mut last_update = SystemTime::now();
    let time_per_frame = Duration::from_secs_f32(1.0 / 10.0);
    let eye: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    let at: Point3<f32> = Point3::new(-10.0, -10.0, -10.0);
    let mut camera = utils::get_camera();
    let mut c = window.add_sphere(0.2);
    c.set_color(1.0, 1.0, 1.0);
    let mut dphi = 0.1;
    let mut phi = 0.0;
    let mut q = (0.0, 0.0, 0.0);
    let mut point = Point3::new(0.0, 0.0, 0.0);

    while window.render_with_camera(&mut camera) {
        if last_update.elapsed().expect("Could not evaluate elapsed time") > time_per_frame {
            last_update = SystemTime::now();
            phi += dphi;
            point.x = f32::cos(phi);
            point.y = f32::sin(phi);
            window.update_trail(&point);
        }
    }
}

mod utils {
    use super::{Camera, FirstPerson, Key, Point3};

    pub fn get_camera() -> FirstPerson {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let at = Point3::new(0.0, 0.0, -1.0);
        let mut camera = FirstPerson::new(at, eye);
        camera.rebind_up_key(Some(Key::W));
        camera.rebind_down_key(Some(Key::S));
        camera.rebind_left_key(Some(Key::A));
        camera.rebind_right_key(Some(Key::D));
        camera
    }
}
