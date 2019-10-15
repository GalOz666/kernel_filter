//use image::{self, DynamicImage, GenericImageView, Rgba, Rgb, Luma};
//
//
//fn main() {
//
//    let image = Image::open("something.jpg");
//    let identity_kernel = vec!([0i8,0,0], [0,1,0], [0,0,0]);
//    let filter = FilteredImage(img_path, 9, identity_kernel);
//
//    let ident_image: DynamicImage = filter.to_grey(i);
//    let rgb = filter.to_rgba();
//
//    ident_image.save("somethingnew.jpg");
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
//}

