use std::{
    fmt::{Display, Formatter},
    ops::{AddAssign, Deref, Mul, MulAssign},
    sync::mpsc,
    thread,
};

use anyhow::Result;

const NUM_THREADS: usize = 4;

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

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct Msg<T> {
    msg: MsgInput<T>,
    // send result back
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub struct MsgOutput<T> {
    idx: usize,
    result: T,
}

fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Display + Copy + AddAssign + MulAssign + Mul<Output = T> + Send + 'static,
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

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let val = dot_product(msg.msg.row, msg.msg.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.msg.idx,
                        result: val,
                    }) {
                        eprint!("Error sending {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // multiply the matrix
    let matrix_len = a.rows * b.cols;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    // Map phase
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new((&a.data[i * a.cols..(i + 1) * a.cols]).to_vec());
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.cols + j;

            let input = MsgInput {
                idx,
                row,
                col,
            };

            let (tx, rx) = oneshot::channel::<MsgOutput<T>>();
            let msg = Msg {
                msg: input,
                sender: tx,
            };
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    // Reduce phase
    for rx in receivers {
        let msg_output = rx.recv()?;
        data[msg_output.idx] = msg_output.result;
    }

    Ok(Matrix::new(a.rows, b.cols, data))
}

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Vector { data }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Default + Copy + Mul<Output = T> + AddAssign<T>,
{
    if a.data.len() != b.data.len() {
        return Err(anyhow::anyhow!(
            "Cannot multiply vectors with dimensions {} and {}",
            a.data.len(),
            b.data.len()
        ));
    }

    let mut result = T::default();
    for i in 0..a.data.len() {
        result += a.data[i] * b.data[i];
    }

    Ok(result)
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
        assert_eq!(c, Matrix::new(2, 2, vec![34, 37, 85, 94]));
        Ok(())
    }
}
