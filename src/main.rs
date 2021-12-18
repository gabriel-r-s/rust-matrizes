use matrizes::prelude::*;


macro_rules! mat {
    (MAT, BUF) => {
        (mat!(MAT), Vec::<u8>::new())
    };
    (MAT) => {
        Matrix::<u8>::from_slice(
            &[
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
            ],
            3,
            3,
        )
    };
}


fn main() {
    for func in [
        indexing, slice_iter, row_iter, transposed
    ] {
        (func)();
        println!();
    }

}


fn indexing() {
    let mat = mat!(MAT);
    println!("indexing");
    for i in 0..mat.height() {
        println!("mat[{}] = {:?}", i, &mat[i]);
    }

    println!("indexing iter");
    for m in 0..mat.height() {
        for n in 0..mat.width() {
            print!("m={} n={}", m, n);
            println!(" {}", &mat[m][n]);
        }
    }
}


fn slice_iter() {
    let mat = mat!(MAT);
    println!("slices iter");
    for line in mat.iter() {
        for num in line {
            println!("{}", num);
        }
    }
}

fn row_iter() {
    let mat = mat!(MAT);
    println!("rows iter");
    for col in mat.cols() {
        for num in col {
            println!("{}", num);
        }
    }
}

fn transposed() {
    let (mat, buf) = mat!(MAT, BUF);
    let transp = mat.transpose_to(buf);
    let compare = &[
        1, 4, 7,
        2, 5, 8,
        3, 6, 9,
    ][..];
    
    for line in mat.iter() {
        println!("orig {:?}", line);
    }
    println!();

    for line in transp.iter() {
        println!("trsp {:?}", line);
    }
    println!();

    for line in compare.chunks(3) {
        println!("test {:?}", line);
    }
    println!();
    
    assert_eq!(transp.as_slice(), compare);
}

