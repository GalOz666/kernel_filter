use std::path::Iter;
use image::{self, DynamicImage, GenericImageView, Rgba, Rgb, Luma};
use nalgebra::{DMatrix, Matrix};
use std::iter::FromIterator;

fn matched_addition_rgba<T, F> (results: &Vec<T>, kernel_size: u8) -> F // refactor for rgba and another one for grey
    where T: ExactSizeIterator<Item = i8>,
    F: FromIterator<Item = u8> {
    let mut fin: [i8; 4] = [0,0,0,0];
    let it_len = &results;
    for idx in 0..it_len {
        fin[idx] = results.iter().fold(0, |acc, x| acc + x[idx])
    }
    fin[0..it_len].iter().map( |x| *x as u8).collect()
}

fn matched_multiplication<T>(vector: Vec<T>, kernel: DMatrix<i8>, image: &DynamicImage, pos: &(u32, u32)) -> Vec<T>
    where T: ExactSizeIterator<Item = i8> {

    // move logic to "new"
    let kernel_size = kernel.len() as f64;
    let col = (kernel_size).sqrt();
    assert_eq!(col.trunc(), col);
    // TODO: add image traversal logic !!
        // break into own function - rgb version, rgba and greyscale!! This is for RGBA
    let col = (kernel_size as f64).sqrt();
    assert_eq!(col.trunc(), col, "kernel size does not have a square root!");
    let mut results= Vec::with_capacity(kernel_size as usize);
    for y in pos.1..(col as u32 +pos.1) {
        for x in pos.0..(col as u32 +pos.0) {
            let color = image.get_pixel(x, y);
            let point_val: i8 = kernel.index((x, y));
            let len: usize;
            let mut val = [0i8, 0, 0, 0];
            match color {
                Rgba(va) => {
                    len = val.len();
                     for (idx, v) in va.enumarate(){
                            val[idx] = v as i8
                     }
                },
                Luma(va) => {
                    len = 1;
                    val[0] = va as i8
                },
                _ => panic!("unsupported pixel type"),
            };
            let newval = val[0..len].iter().map(|x| (x * point_val)/kernel_size as i8).collect();
            results.push(newval)

            }
        }
    results
}

// move to ::new Logic
//fn image_kernel_ops(kernel_size: u8, image_path: &str){
//
//    let kernel = DMatrix::from_vec(col as u8, col as u8, vector);
//    let img: DynamicImage = image.open(image).unwrap();
//    let (w, h) = image::image_dimensions(image_path).unwrap();
//}
//
//fn kernel_proc<T, F>(vector: Vec<i8>, kernel_size: u8, image: &DynamicImage, pos: &(u32, u32)) -> F
//    where T: Iterator<Item = i8>,
//    F: Iterator<Item = u8> {
//    let multiplied: Vec<T> = matched_multiplication(vector, kernel_size, image, pos);
//    let added: F = matched_addition(multiplied, kernel_size);
//}
//
//
//fn rgba_pixle(rgba_pxl: [u8;4],pos: &(u32, u32)){
//
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn identity_test(){
//
//    }
//}
//
