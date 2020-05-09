use std::fmt;
use std::ops;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn dimensions(&self) -> usize {
        4
    }

    pub fn transpose(&mut self) -> &mut Self {
        for y in 0..self.dimensions() - 1 {
            for x in (y + 1..self.dimensions()).rev() {
                let tmp = self[y][x];
                self[y][x] = self[x][y];
                self[x][y] = tmp;
            }
        }

        self
    }

    pub fn transposed(&self) -> Self {
        let mut m = self.clone();
        m.transpose();
        m
    }

    pub fn row_echelon_form(&mut self) -> &mut Self {
        for d in 0..self.dimensions() {
            for y in d + 1..self.dimensions() {
                if self[d][d] == 0.0 {
                    self[d][d] = 1.0e-18;
                }

                let scaler = self[y][d] / self[d][d];

                for x in 0..self.dimensions() {
                    self[y][x] = self[y][x] - scaler * self[d][x];
                }
            }
        }

        self
    }

    /// Returns the determinant of the matrix (read-only)
    ///
    /// This is done by finding the row echelon form of the matrix and then the determinant is the
    /// product of its diagonal.
    pub fn determinant(&self) -> f32 {
        let mut m = self.clone();
        m.row_echelon_form();

        let mut det: f32 = m[0][0];
        for d in 1..self.dimensions() {
            det *= m[d][d]
        }
        det
    }

    fn trivial_mul(self, other: Matrix4) -> Matrix4 {
        let mut m = Matrix4::zero();

        for y in 0..self.dimensions() {
            for x in 0..self.dimensions() {
                let mut sum = 0.0;
                for k in 0..self.dimensions() {
                    sum += self[y][k] * other[k][x];
                }
                m[y][x] = sum;
            }
        }

        m
    }

    // #[cfg(feature = "strassen")]
    fn strassen_mul(self, other: Matrix4) -> Matrix4 {
        fn mul2(a: [[f32; 2]; 2], b: [[f32; 2]; 2]) -> [[f32; 2]; 2] {
            let mut m = [[0.0, 0.0], [0.0, 0.0]];

            for y in 0..2 {
                for x in 0..2 {
                    let mut sum = 0.0;
                    for k in 0..2 {
                        sum += a[y][k] * b[k][x];
                    }
                    m[y][x] = sum;
                }
            }

            m
        }

        fn add2(a: [[f32; 2]; 2], b: [[f32; 2]; 2]) -> [[f32; 2]; 2] {
            let mut m = [[0.0, 0.0], [0.0, 0.0]];
            for i in 0..2 {
                for j in 0..2 {
                    m[i][j] = a[i][j] + b[i][j];
                }
            }

            m
        }

        fn sub2(a: [[f32; 2]; 2], b: [[f32; 2]; 2]) -> [[f32; 2]; 2] {
            let mut m = [[0.0, 0.0], [0.0, 0.0]];
            for i in 0..2 {
                for j in 0..2 {
                    m[i][j] = a[i][j] - b[i][j];
                }
            }

            m
        }

        let a11 = [[self[0][0], self[0][1]], [self[1][0], self[1][1]]];
        let a12 = [[self[0][2], self[0][3]], [self[1][2], self[1][3]]];
        let a21 = [[self[2][0], self[2][1]], [self[3][0], self[3][1]]];
        let a22 = [[self[2][2], self[2][3]], [self[3][2], self[3][3]]];

        let b11 = [[other[0][0], other[0][1]], [other[1][0], other[1][1]]];
        let b12 = [[other[0][2], other[0][3]], [other[1][2], other[1][3]]];
        let b21 = [[other[2][0], other[2][1]], [other[3][0], other[3][1]]];
        let b22 = [[other[2][2], other[2][3]], [other[3][2], other[3][3]]];

        let m1 = mul2(add2(a11, a22), add2(b11, b22));
        let m2 = mul2(add2(a21, a22), b11);
        let m3 = mul2(a11, sub2(b12, b22));
        let m4 = mul2(a22, sub2(b21, b11));
        let m5 = mul2(add2(a11, a12), b22);
        let m6 = mul2(sub2(a21, a11), add2(b11, b12));
        let m7 = mul2(sub2(a12, a22), add2(b21, b22));

        let c11 = add2(sub2(add2(m1, m4), m5), m7);
        let c12 = add2(m3, m5);
        let c21 = add2(m2, m4);
        let c22 = add2(add2(sub2(m1, m2), m3), m6);

        Matrix4([
            [c11[0][0], c11[0][1], c12[0][0], c12[0][1]],
            [c11[1][0], c11[1][1], c12[1][0], c12[1][1]],
            [c21[0][0], c21[0][1], c22[0][0], c22[0][1]],
            [c21[1][0], c21[1][1], c22[1][0], c22[1][1]],
        ])
    }
}

impl ops::Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Self::Output {
        self.strassen_mul(other)
    }
}

impl ops::Neg for Matrix4 {
    type Output = Matrix4;

    fn neg(self) -> Self::Output {
        let mut m = Matrix4::zero();

        for y in 0..self.dimensions() {
            for x in 0..self.dimensions() {
                m[y][x] = -self[y][x];
            }
        }

        m
    }
}

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..4 {
            let line = format!(
                "\t[{}, {}, {}, {}]\n",
                self[y][0], self[y][1], self[y][2], self[y][3],
            );

            s.push_str(&line);
        }

        writeln!(f, "Matrix4 (\n{})", s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn zero_creates_zero_matrix() {
        let expects = super::Matrix4([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(
            expects,
            super::Matrix4::zero(),
            "Did not create a zero matrix"
        );
    }

    #[test]
    fn identity_creates_identity_matrix() {
        let expects = super::Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(
            expects,
            super::Matrix4::identity(),
            "Did not create an identity matrix"
        );
    }

    #[test]
    fn dimensions_return_4() {
        let expects = 4;
        let matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(
            expects,
            matrix.dimensions(),
            "Did not return dimensions of 4"
        );
    }

    #[test]
    fn transpose_transposes_matrix() {
        let expects = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let mut matrix = super::Matrix4([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);

        matrix.transpose();
        assert_eq!(expects, matrix, "Did not transpose matrix");
    }

    #[test]
    fn transposed_returns_transposed_matrix() {
        let expects = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix = super::Matrix4([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);

        assert_eq!(
            expects,
            matrix.transposed(),
            "Did not return transposed matrix"
        );
    }

    #[test]
    fn to_row_echelon_form_returns_row_echelon_form_matrix() {
        let expects = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 4.0, 8.0, 12.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let mut matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        matrix.row_echelon_form();
        assert_eq!(
            expects, matrix,
            "Did not return correct echelon form matrix"
        );
    }

    #[test]
    fn determinant_returns_correct_value() {
        let expects = -160.0;
        let matrix = super::Matrix4([
            [1.0, 6.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(
            expects,
            matrix.determinant(),
            "Did not return correct determinant"
        );
    }

    #[test]
    fn index_returns_correct_row() {
        let expects = [9.0, 10.0, 11.0, 12.0];
        let matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(expects, matrix[2], "Did not return correct row");
    }

    #[test]
    fn index_mut_updates_correct_row() {
        let expects = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let mut matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [0.0, 0.0, 0.0, 0.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let new_row = [9.0, 10.0, 11.0, 12.0];
        matrix[2] = new_row;

        assert_eq!(expects, matrix, "Did not update correct row");
    }

    #[test]
    fn mul_returns_product_of_matrices() {
        let expects = super::Matrix4([
            [110.0, 128.0, 138.0, 132.0],
            [202.0, 248.0, 254.0, 240.0],
            [314.0, 392.0, 398.0, 380.0],
            [361.0, 466.0, 467.0, 465.0],
        ]);

        let matrix1 = super::Matrix4([
            [1.0, 6.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        let matrix2 = super::Matrix4([
            [1.0, 6.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(
            expects,
            matrix1 * matrix2,
            "Did not return correct product of matrices"
        );
    }

    #[test]
    fn neg_returns_negative_of_matrix() {
        let expects = super::Matrix4([
            [-1.0, -2.0, -3.0, -4.0],
            [-5.0, -6.0, -7.0, -8.0],
            [-9.0, -10.0, -11.0, -12.0],
            [-13.0, -14.0, -15.0, -11.0],
        ]);

        let matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(expects, -matrix, "Did not correctly negate matrix");
    }

    #[test]
    fn mul_identity_returns_arg() {
        let matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(
            matrix,
            super::Matrix4::identity().trivial_mul(matrix),
            "A * I should equal A"
        );
        assert_eq!(
            matrix,
            super::Matrix4::identity().strassen_mul(matrix),
            "A * I should equal A"
        );
    }

    #[test]
    fn mul_zero_returns_zero() {
        let matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(
            super::Matrix4::zero(),
            super::Matrix4::zero().trivial_mul(matrix),
            "A * 0 should equal 0"
        );
        assert_eq!(
            super::Matrix4::zero(),
            super::Matrix4::zero().strassen_mul(matrix),
            "A * 0 should equal 0"
        );
    }

    #[test]
    fn mul_strassen_returns_like_trivial() {
        let matrix = super::Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(
            matrix.trivial_mul(matrix),
            matrix.strassen_mul(matrix),
            "Trivial multiplication and Strassen multiplcation should return the same value"
        );
    }
}
