pub mod matrix {

    pub trait Matrix {
        fn new(_rows: usize, _cols: usize) -> Self;
        fn get_rows(&self) -> usize;
        fn get_cols(&self) -> usize;
    }
    pub struct  Matrix2D {
        m_rows : usize,
        m_cols : usize,
        m_matrix : Vec<Vec<isize>>
    }

    impl Matrix2D {
        pub fn get_clone_data(&self) -> Vec<Vec<isize>> {
            return self.m_matrix.clone();
        }
    }

    impl Matrix for Matrix2D {
        fn new(_rows: usize, _cols: usize) -> Matrix2D {
            return Matrix2D { m_rows: _rows, m_cols: _cols, m_matrix : vec![vec![0 ; _cols]; _rows] };
        }
        fn get_rows(&self) -> usize {
            return self.m_rows;
        }
        fn get_cols(&self) -> usize {
            return self.m_cols;
        }
    }
}