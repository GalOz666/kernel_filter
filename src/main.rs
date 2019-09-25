use std::path::Iter;
use image::{self, DynamicImage, GenericImageView, Rgba};
use nalgebra::{DMatrix, Matrix};

fn kernel_filter(vector: Vec<i8>, kernel_size: u8, image_path: &str) {

    let col = (kernel_size as f64).sqrt();
    assert_eq!(col.trunc(), col);

    let kernel = DMatrix::from_vec(col as u8, col as u8, vector);
    let img: DynamicImage = image.open(image).unwrap();
    let (w, h) = image::image_dimensions(image_path).unwrap();
    // TODO: add image traversal logic !!
        // break into own function - rgb version, rgba and greyscale!! This is for RGBA
    let mut results: Vec<[i8; 4]>;
        for y in 0u32..kernel_size {
            for x in 0u32..kernel_size {
                let rgba = img.get_pixel(x, y);
                let point_val = kernel.index(x, y);
                let Rgba(val) = rgba;
                let newval = val.iter().map(|x| x as i8 * point_val).collect();
                results.push(newval)

            }
        }

    let mut fin: [u8; 4];
    for idx in 0..results[0].len() {
        for pix in results {
            fin[idx] = results.iter().fold(1, |acc ,x| acc+x[idx]) as u8
        }
    }
}

fn main() {

    let image = Image::open("something.jpg");
    let identity_kernel = &[[0i8,0,0], [0,1,0], [0,0,0]];

    let ident_image: DynamicImage = kernel_filter(identity_kernel, image_path);
    ident_image.save("somethingnew.jpg");
//
//    let dm1 = DMatrix::from_row_slice(3, 3, &[
//    0, 0, 0,
//    0, 1, 0,
//    0, 0, 0,
//    ]);
//    let dm2 = DMatrix::from_row_slice(3, 3, &[
//    255, 255, 255,
//    255, 69, 255,
//    255, 255, 255,
//    ]);
//   for x in dm2.into_iter() {
//    println!("{:?}", x)
//        }
}

