use core::panic;
use std::{path::PathBuf, str::FromStr};

use opencv::{highgui, prelude::*, videoio, Result};
pub fn take_photo() {
    match take() {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {}", e),
    }
}

fn take() -> Result<()> {
    let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY)?;
    let is_opened = videoio::VideoCapture::is_opened(&cam)?;
    if !is_opened {
        panic!("Unable to open default camera!");
    }

    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            highgui::destroy_window(window)?;
            break;
        }
    }
    let filename = "captured_image.png";
    let path = PathBuf::from_str("src/camera").unwrap().join(filename);
    opencv::imgcodecs::imwrite(
        &path.to_str().unwrap(),
        &frame,
        &opencv::core::Vector::new(),
    )?;
    println!("Image saved as: {}", filename);

    Ok(())
}
