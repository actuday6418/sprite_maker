extern crate image;

use std::fs::File;
use std::io::prelude::*;

fn as_u8_slice(v: &[i32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}

#[no_mangle]
pub extern "C" fn edit() {
    let dimensions = vec![50 as i32, 50 as i32];

    let img = image::open("glass.png").expect("Image loading failed!");

    let mut img = img.thumbnail(dimensions[0] as u32, dimensions[1] as u32);

    let img = img.as_mut_rgba8().unwrap();

    let mut file = File::create("glass.sprite").expect("Couldn't create that file");
    file.write(as_u8_slice(&dimensions)).unwrap();

    let mut ivec = Vec::new();
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        let image::Rgba(data) = *pixel;
        let mut avg: i32 = (data[0] as i32 + data[2] as i32 + data[1] as i32) / 3;
        if avg == 0 {
            avg += 1;
        }
        if data[3] == 0 {
            avg = 0;
        }
        ivec.push(avg as u8);
    }

    file.write(&ivec[..]).unwrap();
}
