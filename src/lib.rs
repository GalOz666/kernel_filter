use image::{self, DynamicImage, GenericImageView, Rgba, Rgb, Luma, ImageBuffer, GenericImage, GrayImage};
use imageproc::contrast::threshold;

// make into generic iterator for rgba - cf. grey example
fn matched_addition_rgba(results: &[[u8; 4]]) -> Vec<u8> {
    let mut fin= Vec::with_capacity(4);
    for idx in 0..4 {
        fin.push(results.to_vec().iter().fold(0, |acc: u8, x| acc.saturating_add(x[idx])))
    }
    fin
}

fn matched_multiplication<'a>(kernel: &[i8], pixel_cell: &'a mut [[u8; 4]]) -> &'a [[u8; 4]] {
    for y in 0..kernel.len() {
        let mut pixl = pixel_cell[y];
        let point_val = kernel[y];
        for idx in 0..pixel_cell[0].len() {
            pixl[idx] = ((((pixl[idx] as i16) *point_val as i16)/9) as u8)
        }
        pixel_cell[y] = pixl
        }
    pixel_cell
}

fn pixel_cell_9(image: &DynamicImage, position: (u32, u32)) -> [[u8; 4];9]  {
    let mut cell: [[u8; 4];9] = Default::default();
    for y in 0..3 {
        for x in 0..3 {
            let idx = (x + y) as usize;
            if let Rgba(color) = image.get_pixel(x + position.0, y + position.1) {
                cell[idx] = color
            }
        }
    }
    cell
}

    fn pixel_cell_25(image: &DynamicImage, position: (u32, u32)) -> [[u8; 4];25]  {
    let mut cell: [[u8; 4];25] = Default::default();
    for y in 0..5 {
        for x in 0..5 {
            let idx = (x+y) as usize;
            cell[idx] = match image.get_pixel(x+ position.0, y+position.1) {
                Rgba(color) => color,
                _ => panic!("problem processing pixel!")
            }
        }
    }
    cell
}

// TODO: genericise with kernel as slice and dispatch accordingly: either 25/9.
pub fn traverse_image(mut image: DynamicImage, kernel: &[i8]) -> DynamicImage {
    let (w, h) = image.dimensions();
    let limit = (kernel.len() as f64).sqrt() as u32;
    for y in 0..h-limit {
        for x in 0..w-limit {
            let mut cell = pixel_cell_9(&image, (x, y));
            let result = matched_addition_rgba(&matched_multiplication(kernel, &mut cell));
            image.put_pixel( x+1, y+1,Rgba([result[0], result[1], result[2], result[3]]));
        }
    }
    image
}

fn traverse_image_25 (mut image: DynamicImage, kernel: &[i8]) -> DynamicImage { unimplemented!() }


pub fn costume_filter(image: &DynamicImage, kernel: &[i8]) -> DynamicImage  {
    let copy = image.clone();
    match kernel.len() {
        9 => return traverse_image(copy, kernel),
        25 => return traverse_image_25(copy, kernel),
        _ => panic!("current kernel sizes are either 9 (3*3) or 25 (5*5)!")
    }
}

pub fn gaussian_filter(image: &DynamicImage) -> DynamicImage {
    let copy = image.clone();
    let kernel = [1, 2, 1, 2, 4, 2, 1, 2, 1];
    traverse_image(copy, &kernel)
}

pub fn edge_detection(image: &DynamicImage, otsu_threshold: u8) -> GrayImage {
    let copy = gaussian_filter(image);
    let copy = copy.grayscale();
    let kernel = [1, 1, 1, 1, -8, 1, 1, 1, 1];
    let fin = traverse_image(copy, &kernel);
    let fin= gaussian_filter(&fin).to_luma();
    threshold(&fin, otsu_threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_test(){
        let kernel = [
            0i8, 0, 0,
            0, 9, 0,
            0, 0, 0];
        let mut pixel_cell: [[u8; 4];9] = [
            [1,2,3,4],[1,2,3,4], [1,253,3,4],
            [1,2,3,4], [1,1,0,1], [5,2,3,4],
            [1,2,3,4], [1,2,5,5], [1,2,3,4],
        ];
        let multiplied = matched_multiplication(&kernel, &mut pixel_cell);
        assert_eq!(multiplied,
                   [[0u8,0,0,0],[0,0,0,0], [0,0,0,0],
                   [0,0,0,0], [1,1,0,1], [0,0,0,0],
                   [0,0,0,0], [0,0,0,0], [0,0,0,0]]);
        assert_eq!(matched_addition_rgba(&multiplied), vec!(1,1, 0,1));
    }
    #[test]
    fn test_save_gaussian() {
        let img = image::open("src/img.jpg").expect("could not find file to open!");
        let edged = gaussian_filter(&img);
        edged.save("copy.jpg").expect("could not save the file");
    }
    #[test]
    fn test_save_edge_grey() {
        let img = image::open("src/img.jpg").expect("could not find file to open!");
        let img = img.grayscale();
        let edged = edge_detection(&img, 230);
        edged.save("copy_grey.jpg").expect("could not save the file");
    }
}