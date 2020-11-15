
extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

extern crate image;

use std::fs::File;
use std::io::prelude::*;

py_module_initializer!(libedit, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "edit", py_fn!(py, edit(val: &str, xdim: &str, ydim: &str)))?;
    Ok(())
});

fn as_u8_slice(v: &[i32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}

fn edit(_py: Python, file_dir: &str, xdim: &str, ydim: &str) -> PyResult<i32>{

    let mut dimensions = vec![];
    dimensions.push(xdim.parse::<i32>().expect("X dimension was not an integer!"));
    dimensions.push(ydim.parse::<i32>().expect("Y dimension was not an integer!"));

    let img = image::open(file_dir).expect("Image loading failed!");

    let mut img = img.thumbnail(dimensions[0] as u32, dimensions[1] as u32);

    let img = img.as_mut_rgba8().unwrap();

    let mut file = File::create("output.sprite").expect("Couldn't create that file");
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

    file.write(&ivec[..]).expect("unable to write to file!");
    Ok(1)
}
