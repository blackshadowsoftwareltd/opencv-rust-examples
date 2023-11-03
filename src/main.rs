pub mod camera;
pub mod face_detect;
// use crate::camera::camera::take_photo;
use crate::face_detect::face_detect::face_detect;
fn main() {
    // take_photo();
    face_detect();
}
