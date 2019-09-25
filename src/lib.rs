//use ndarray::{Array, Array3};
//
//fn rgb_identity(pixel_arr: &Array3<f32>, kernel: &Array3<f32>) -> f32 {
//    let result_m = pixel_arr.dot(kernel);
//    result_m[[2,2]]
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use ndarray::arr3;
//
//    #[test]
//    fn identity_test(){
//        let rgb = [4f32,255,20];
//        let null = [1f32, 1, 1];
//        let pixel_arr = arr3(&[&null, &rgb, &null]);
//        let kernel = arr3(&[[0f32,0,0], [0f32,1,0], [0f32, 0, 0]]);
//        let result = rgb_identity(&pixel_arr, &kernel);
//        assert_eq!(result, rgb)
//    }
//}
//
