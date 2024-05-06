use std::{
    fmt::{Display, Formatter},
    ops::{AddAssign, Mul, MulAssign},
};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Display> Matrix<T> {
    fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Self { rows, cols, data }
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{} ", self.data[i * self.cols + j])?;
            }
            writeln!(f)?;
            // write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Display + Copy + AddAssign + MulAssign + Mul<Output = T>,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!(
            "Cannot multiply matrices with dimensions {}x{} and {}x{}",
            a.rows,
            a.cols,
            b.rows,
            b.cols
        ));
    }

    // multiply the matrix
    let mut result = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                result[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix::new(a.rows, b.cols, result))
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn it_works() -> Result<()> {
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, vec![5, 6, 7, 8, 5, 5]);
        let c = multiply(&a, &b)?;
        println!("{}", c);
        assert_eq!(
            c,
            Matrix::new(2, 2, vec![34, 37, 85, 94])
        );
        Ok(())
    }
}