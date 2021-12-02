use matrizes::prelude::*;

fn main() {
    let mat = Matrix::<u8>::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 3, 4);
    // dbg!(&mat[0], &mat[1], &mat[2], &mat[3]);

    // println!("indexing iter");
    // for m in 0..mat.height() {
    //     for n in 0..mat.width() {
    //         println!("{}", &mat[m][n]);
    //     }
    // }

    println!("slices iter");
    for line in mat.iter() {
        for num in line {
            println!("{}", num);
        }
    }

    // dbg!(mat.mul(8));

    println!("rows iter");
    for col in mat.cols() {
        for num in col {
            println!("{}", num);
        }
    }
}
