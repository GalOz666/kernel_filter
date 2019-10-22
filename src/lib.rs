use image::{self, DynamicImage, GenericImageView, Rgba, Rgb, Luma, ImageBuffer, GenericImage, GrayImage};
use imageproc::contrast::threshold;

fn matched_addition_rgba(results: Vec<[f32; 4]>) -> Vec<u8> {
    let mut fin= Vec::with_capacity(4);
    for idx in 0..4 {
        fin.push(results.iter().fold(0.0, |acc: f32, x: &[f32; 4]| acc + x[idx].round()));
    }
    fin.iter().map(|x| (x / 9_f32) as u8).collect()
}

fn matched_multiplication(kernel: &[i8], pixel_cell: &[[u8; 4]]) -> Vec<[f32; 4]> {
    let mut fin = [0u8, 0, 0, 0];
    let mut multip_list = Vec::with_capacity(pixel_cell.len());
    for (idx, m) in kernel.iter().enumerate() {
        let mut pixl = [0_f32, 0.0, 0.0, 0.0];
        for color in 0usize..4 {
            pixl[color] = (pixel_cell[idx][color] as f32) * (kernel[idx] as f32);
        }
        multip_list.push(pixl);
    }
    multip_list
}


fn pixel_cell(image: &DynamicImage, position: (u32, u32)) -> [[u8; 4];9]  {
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

pub fn filter_image(mut image: DynamicImage, kernel: &[i8], dimensions: &(u32, u32)) -> DynamicImage {
    let (w, h) = image.dimensions();
    let limit = (kernel.len() as f64).sqrt() as u32;
    for y in 0..h-limit {
        for x in 0..w-limit {
            let mut cell = pixel_cell(&image, (x, y));
            let result = matched_addition_rgba(matched_multiplication(kernel, &cell));
            image.put_pixel( x+1, y+1,Rgba([result[0], result[1], result[2], result[3]]));
        }
    }
    image
}

fn filter_image_25 (mut image: DynamicImage, kernel: &[i8], dimensions: &(u32, u32)) -> DynamicImage {
    let (w, h) = image.dimensions();
    for y in 0..h-5 {
        for x in 0..w-5 {
            let mut cell = pixel_cell_25(&image, (x, y));
            let result = matched_addition_rgba(matched_multiplication(kernel, &mut cell));
            image.put_pixel( x+1, y+1,Rgba([result[0], result[1], result[2], result[3]]));
        }
    }
    image
}

pub fn costume_filter(image: &DynamicImage, kernel: &[i8]) -> DynamicImage  {
    let copy = image.clone();
    let dimension = image.dimensions();
    match kernel.len() {
        9 => return filter_image(copy, kernel, &dimension),
        25 => return filter_image_25(copy, kernel, &dimension),
        _ => unimplemented!("current kernel sizes are either 9 (3*3) or 25 (5*5)!")
    }
}

pub fn gaussian_blur(image: &DynamicImage) -> DynamicImage {
    let copy = image.clone();
    let dimension = image.dimensions();
    let kernel = [1, 2, 1, 2, 4, 2, 1, 2, 1];
    filter_image(copy, &kernel, &dimension)
}

pub fn mean_blur(image: &DynamicImage) -> DynamicImage {
    let copy = image.clone();
    let dimension = image.dimensions();
    let kernel = [1, 1, 1, 1, 1, 1, 1, 1, 1];
    filter_image(copy, &kernel, &dimension)
}

pub fn edge_detection(image: &DynamicImage, otsu_threshold: u8) -> GrayImage {
    let dimension = image.dimensions();
    let copy = gaussian_blur(image);
    let copy = copy.grayscale();
    let kernel = [1, 1, 1, 1, -8, 1, 1, 1, 1];
    let fin = filter_image(copy, &kernel, &dimension).to_luma();
    threshold(&fin, otsu_threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn identity_test(){
//        let kernel = [
//            0i8, 0, 0,
//            0, 9, 0,
//            0, 0, 0];
//        let mut pixel_cell: [[u8; 4];9] = [
//            [1,2,3,4],[1,2,3,4], [1,253,3,4],
//            [1,2,3,4], [1,1,0,1], [5,2,3,4],
//            [1,2,3,4], [1,2,5,5], [1,2,3,4],
//        ];
//        let multiplied = matched_multiplication(&kernel, &mut pixel_cell);
//        assert_eq!(multiplied,
//                   [[0u8,0,0,0],[0,0,0,0], [0,0,0,0],
//                   [0,0,0,0], [1,1,0,1], [0,0,0,0],
//                   [0,0,0,0], [0,0,0,0], [0,0,0,0]]);
//        assert_eq!(matched_addition_rgba(multiplied), vec!(1,1, 0,1));
//    }
    #[test]
    fn test_mean_blur_cell(){
        let kernel = [
            1i8, 1, 1,
            1, 1, 1,
            1, 1, 1];
        let pixel_cell  = [
            [1,2,3,4],[1,2,3,4], [1,2,3,4],
            [1,2,3,4], [1,1,0,1], [5,2,3,4],
            [1,2,3,4], [1,2,5,5], [1,2,3,4]
        ];
        let outcome = matched_multiplication( &kernel, &pixel_cell);
        for (idx, pixel) in pixel_cell.iter().enumerate() {
            let repr: Vec<f32> = pixel.iter().map(|x| *x as f32).collect();
            assert_eq!(repr, outcome[idx].to_vec())
        }

    }
    #[test]
    fn test_m_sum() {
        let kernel = [
            1i8, 1, 1,
            1, 1, 1,
            1, 1, 1];
        let pixel_cell  = [
        [1,2,3,4],[1,2,3,4], [1,2,3,4],
        [1,2,3,4], [1,2,3,4], [1,2,3,4],
        [1,2,3,4], [1,2,3,4], [1,2,3,4]];

        let outcome = matched_multiplication(&kernel, &pixel_cell);
        let sum = matched_addition_rgba(outcome);
        assert_eq!(sum, vec!(1,2,3,4))

    }
    #[test]
    fn test_save_gaussian() {
        let img = image::open("src/img.jpg").expect("could not find file to open!");
        let edged = mean_blur(&img);
        edged.save("copy.jpg").expect("could not save the file");
    }
    #[test]
    fn test_save_edge_grey() {
        let img = image::open("src/img.jpg").expect("could not find file to open!");
        let img = img.grayscale();
        let edged = edge_detection(&img, 200);
        edged.save("copy_grey.jpg").expect("could not save the file");
    }
}