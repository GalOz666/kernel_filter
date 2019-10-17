use image::{self, DynamicImage, GenericImageView, Rgba, Rgb, Luma, ImageBuffer, GenericImage};

pub fn matched_addition_rgba_9(results: [[u8; 4];9]) -> Vec<u8> {
    let mut fin= Vec::with_capacity(4);
    for idx in 0..4 {
        fin.push(results.to_vec().iter().fold(0, |acc: u8, x| acc.saturating_add(x[idx])))
    }
    fin
}

fn matched_multiplication_9(kernel: [i8;9], mut pixel_cell: [[u8; 4];9]) -> [[u8; 4];9] {
    for y in 0..kernel.len() {
        let mut pixl = pixel_cell[y];
        let point_val = kernel[y];
        for idx in 0..4usize {
            pixl[idx] = (((pixl[idx] as i16) *point_val as i16)/9) as u8
        }
        pixel_cell[y] = pixl
        }
    pixel_cell
}

fn pixel_cell_9(image: &DynamicImage, position: (u32, u32)) -> [[u8; 4];9]  {
    let mut cell: [[u8; 4];9] = Default::default();
    for y in 0..3 {
        for x in 0..3 {
            let idx = (x+y) as usize;
            cell[idx] = match image.get_pixel(x+ position.0, y+position.1) {
                Rgba(color) => color,
                _ => panic!("problem processing pixel!")
            }
        }
    }
    cell
}

fn traverse_image_9(image: &DynamicImage, kernel: [i8;9]) -> DynamicImage {
    let mut copy = image.clone();
    let (w, h) = image.dimensions();

    for y in 0..h-3 {
        for x in 0..w-3 {
            let pixle = copy.get_pixel(x+1, y+1);
            let mut cell = pixel_cell_9(image, (x, y));
            let result = matched_addition_rgba_9(matched_multiplication_9(kernel, cell));
            copy.put_pixel( x+1, y+1,Rgba([result[0], result[1], result[2], result[3]]));
        }
    }
    copy
}

fn costume_filter(image: &DynamicImage, kernel: [i8;9]) -> DynamicImage  {
    traverse_image_9(image, kernel)
}

fn gaussianish_filter(image: &DynamicImage) -> DynamicImage {
    let kernel = [1, 2, 1, 2, 4, 2, 1, 2, 1];
    traverse_image_9(image, kernel)
}

fn edge_detection(image: &DynamicImage) -> DynamicImage {
    let kernel = [1, 1, 1, 1, -8, 1, 1, 1, 1];
    traverse_image_9(image, kernel)
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
    #[test]
    fn test_save_edge_detc() {
        let img = image::open("src/img.jpg").expect("could not find file to open!");
        let edged = edge_detection(&img);
        edged.save("copy.jpg").expect("could not save the file");
    }
}

