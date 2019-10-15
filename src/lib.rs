use image::{self, DynamicImage, GenericImageView, Rgba, Rgb, Luma};

pub fn matched_addition_rgba_9 (results: [[u8; 4];9]) -> Vec<u8> {
    let mut fin= Vec::with_capacity(4);
    for idx in 0..4 {
        fin.push(results.to_vec().iter().fold(0, |acc, x| acc + x[idx]))
    }
    fin
}

fn matched_multiplication_9(kernel: [u8;9], mut pixel_cell: [[u8; 4];9]) -> [[u8; 4];9] {
    for y in 0..kernel.len() {
        let mut pixl = pixel_cell[y];
        let point_val = kernel[y];
        for idx in 0..4usize {
            pixl[idx] = (pixl[idx]*point_val)/9
        }
        pixel_cell[y] = pixl
        }
    pixel_cell
}

fn pixel_cell_9(image: &DynamicImage) -> [[u8; 4];9]  {
    let mut cell: [[u8; 4];9] = Default::default();
    for y in 0..3 {
        for x in 0..3 {
            let idx = (x+y) as usize;
            cell[idx] = match image.get_pixel(x, y) {
                Rgba(color) => color,
                _ => panic!("problem processing pixel!")
            }
        }
    }
    cell
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

//}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn identity_test(){
        let kernel = [
            0u8, 0, 0,
            0, 9, 0,
            0, 0, 0];
        let pixel_cell: [[u8; 4];9] = [
            [1,2,3,4],[1,2,3,4], [1,253,3,4],
            [1,2,3,4], [1,1,0,1], [5,2,3,4],
            [1,2,3,4], [1,2,5,5], [1,2,3,4],
        ];
        let multiplied = matched_multiplication_9(kernel, pixel_cell);
        assert_eq!(multiplied,
                   [[0u8,0,0,0],[0,0,0,0], [0,0,0,0],
                   [0,0,0,0], [1,1,0,1], [0,0,0,0],
                   [0,0,0,0], [0,0,0,0], [0,0,0,0]]);
        assert_eq!(matched_addition_rgba_9(multiplied), vec!(1,1, 0,1));

    }
}

