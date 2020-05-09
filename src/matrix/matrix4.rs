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

    pub fn size(&self) -> usize {
        4
    }

    pub fn transpose(&mut self) -> &mut Self {
        for y in 0..self.size() - 1 {
            for x in (y + 1..self.size()).rev() {
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
        let mut m = Matrix4::zero();

        for y in 0..self.size() {
            for x in 0..self.size() {
                let mut sum = 0.0;
                for k in 0..self.size() {
                    sum += self[y][k] * other[k][x];
                }
                m[y][x] = sum;
            }
        }

        m
    }
}

impl ops::Neg for Matrix4 {
    type Output = Matrix4;

    fn neg(self) -> Self::Output {
        let mut m = Matrix4::zero();

        for y in 0..self.size() {
            for x in 0..self.size() {
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
    use super::*;

    #[test]
    fn zero_creates_zero_matrix() {
        let expects = Matrix4([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(expects, Matrix4::zero(), "Did not create a zero matrix");
    }

    #[test]
    fn identity_creates_identity_matrix() {
        let expects = Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(
            expects,
            Matrix4::identity(),
            "Did not create an identity matrix"
        );
    }

    #[test]
    fn size_return_4() {
        let expects = 4;
        let matrix = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(expects, matrix.size(), "Did not return size of 4");
    }

    #[test]
    fn transpose_transposes_matrix() {
        let expects = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let mut matrix = Matrix4([
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
        let expects = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix = Matrix4([
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
    fn index_returns_correct_row() {
        let expects = [9.0, 10.0, 11.0, 12.0];
        let matrix = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(expects, matrix[2], "Did not return correct row");
    }

    #[test]
    fn index_mut_updates_correct_row() {
        let expects = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let mut matrix = Matrix4([
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
        let expects = Matrix4([
            [110.0, 128.0, 138.0, 132.0],
            [202.0, 248.0, 254.0, 240.0],
            [314.0, 392.0, 398.0, 380.0],
            [361.0, 466.0, 467.0, 465.0],
        ]);

        let matrix1 = Matrix4([
            [1.0, 6.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        let matrix2 = Matrix4([
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
        let expects = Matrix4([
            [-1.0, -2.0, -3.0, -4.0],
            [-5.0, -6.0, -7.0, -8.0],
            [-9.0, -10.0, -11.0, -12.0],
            [-13.0, -14.0, -15.0, -11.0],
        ]);

        let matrix = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 11.0],
        ]);

        assert_eq!(expects, -matrix, "Did not correctly negate matrix");
    }
}
