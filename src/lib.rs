extern crate aljabar;
use aljabar::{matrix, vector, Vector};

fn identity() -> Vector<u8, 4> {
    let pixel = vector![0,255,255,15];
    let kernel = matrix![
    [0,0,0],
    [0,1,0],
    [0,0,0]];
    pixel*kernel
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_test(){
        let result = identity();
        assert_eq!(result, vector![0,255,255,15])
    }
}
