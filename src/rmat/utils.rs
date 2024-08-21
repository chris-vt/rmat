use crate::rmat::Matrix;

// enums
pub enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
}

impl Matrix {
    
    pub fn vector_sumproduct(v1: &Vec<f64>, v2: &Vec<f64>) -> Result<f64, &'static str> {
        if v1.len() != v2.len() {
            return Err("Arrays are not same length");
        } else {
            let mut sum: f64 = 0.0;
            for i in 0..v1.len() {
                sum += v1[i] * v2[i]
            }
            return Ok(sum);
        }
    }
    pub fn vector_to_column(mat: &Matrix, c: usize) -> Result<Vec<f64>, &'static str> {
        if c >= mat.c {
            return Err("Column out of bound");
        } else {
            let mut res: Vec<f64> = Vec::new();
            for r in 0..mat.r {
                res.push(mat.m[r][c])
            }
            return Ok(res);
        }
    }
    pub fn arithmetic_op(&self, other: &Matrix, op: Arithmetic) -> Result<Matrix, &'static str> {
        if self.c != other.c || self.r != other.r {
            return Err("Dimissions mismatch");
        } else {
            let mut mul_result = vec![vec![1f64; self.c]; self.r];
            for r in 0..self.r {
                for c in 0..self.c {
                    let t1 = self.m[r][c];
                    let t2 = other.m[r][c];
                    mul_result[r][c] = match op {
                        Arithmetic::Add => t1 + t2,
                        Arithmetic::Sub => t1 - t2,
                        Arithmetic::Mul => t1 * t2,
                        Arithmetic::Div => t1 / t2,
                    }
                }
            }
            return Ok(Matrix {
                r: self.r,
                c: self.c,
                m: mul_result,
            });
        }
    }
}