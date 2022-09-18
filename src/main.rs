
mod matrix;
use crate::matrix::matrix::{ Matrix, Matrix2D };

fn main() {
    let mtr = Matrix2D::new(2, 2);

    print!("{}", mtr.get_cols());
}
