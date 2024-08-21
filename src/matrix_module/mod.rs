#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use rand::prelude::*;
use std::fmt;
use crate::matrix_module::utils::Arithmetic;

mod utils;
// structs
/// The Matrix struct represents a two-dimensional structured vector implementing key algebra operations
#[derive(Debug,Clone,PartialEq)]
pub struct Matrix {
    /// Number of rows 
    pub r: usize,
    /// Number of columns
    pub c: usize,
    /// Two-dimentional vector of f64 values
    pub m: Vec<Vec<f64>>,
}

// Matrix struct - builders
/// This implementation block defines the builder methods for different characteristic matrices
impl Matrix {
    /// The method returns a Matrix of size (rows,cols) containing random f64 values between 0.0 and 1.0 
    pub fn rand(rows: usize, cols: usize) -> Matrix {
        let mut matrix = vec![vec![1f64; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                matrix[r][c] = rand::thread_rng().gen_range(0.0..1.0);
            }
        }
        Matrix {
            r: rows,
            c: cols,
            m: matrix,
        }
    }
    
    /// The method returns a Matrix of size (rows,cols) containing ones
    pub fn one(rows: usize, cols: usize) -> Matrix {
        let mut matrix = vec![vec![1f64; cols]; rows];
        Matrix {
            r: rows,
            c: cols,
            m: matrix,
        }
    }

    /// The method returns an identity Matrix of size (size)
    pub fn id(size: usize) -> Matrix {
        let mut matrix = vec![vec![1f64; size]; size];
        for r in 0..size {
            for c in 0..size {
                if r == c {
                    println!("{}:{}", r, c);
                    matrix[r][c] = 1.0;
                } else {
                    matrix[r][c] = 0.0;
                }
            }
        }
        Matrix {
            r: size,
            c: size,
            m: matrix,
        }
    }
    
    /// The method returns a Matrix of which the content is defined by the user. See ```matrix!``` macro.
    pub fn usr(m: Vec<Vec<f64>>) -> Matrix {
        let r = m.len();
        let c = m[0].len();
        for t in 1..r {
            if m[0].len() != m[t].len() {
                panic!("Matrix - malformed data")
            }
        }
        Matrix { r, c, m }
    }
}

// Matrix struct - user methods
/// This implementation block defines the algebra operations available for the struct Matrix.
/// Note that these are all implemented as instance methods.
/// Note also that, where a second Matrix operand its involved, it needs to be passed as reference to the method.
impl Matrix {    
    #[allow(non_snake_case)]
    /// The method returns the transposed of the Matrix instance
    pub fn T(&self) -> Matrix {
        let mut res = vec![vec![1f64; self.r]; self.c];
        for r in 0..self.c {
            for c in 0..self.r {
                res[r][c] = self.m[c][r];
            }
        }
        return Matrix {
            r: self.c,
            c: self.r,
            m: res,
        };
    }

    /// The method returns the dot multiplication of the Matrix instance with a second Matrix
    pub fn dot(&self, other: &Matrix) -> Matrix {
        if self.c != other.r {
            panic!("Dot - dimissions mismatch");
        } else {
            let mut res = vec![vec![1f64; other.c]; self.r];
            for r in 0..self.r {
                for c in 0..other.c {
                    let a = &self.m[r];
                    let b = match utils::vector_to_column(&other, c) {
                        Err(msg) => panic!("{}", msg),
                        Ok(m) => m,
                    };
                    res[r][c] = match utils::vector_sumproduct(a, &b) {
                        Err(msg) => panic!("{}", msg),
                        Ok(r) => r,
                    }
                }
            }
            return Matrix {
                r: self.r,
                c: other.c,
                m: res,
            };
        }
    }

    /// The method returns the arithmetic item-wise addition of the Matrix instance with a second Matrix
    pub fn add(&self, other: &Matrix) -> Matrix {
        match utils::arithmetic_op(self, other, utils::Arithmetic::Add) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }

    /// The method returns the arithmetic item-wise subtraction of the Matrix instance with a second Matrix
    pub fn sub(&self, other: &Matrix) -> Matrix {
        match utils::arithmetic_op(self, other, utils::Arithmetic::Sub) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }

    /// The method returns the arithmetic item-wise multiplication of the Matrix instance with a second Matrix
    pub fn mul(&self, other: &Matrix) -> Matrix {
        match utils::arithmetic_op(self, other, utils::Arithmetic::Mul) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }

    /// The method returns the arithmetic item-wise division of the Matrix instance with a second Matrix
    pub fn div(&self, other: &Matrix) -> Matrix {
        match utils::arithmetic_op(self, other, utils::Arithmetic::Div) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }

    /// The method returns the arithmetic item-wise multiplication of the Matrix instance with a scalar
    pub fn sca(&self, factor: &f64) -> Matrix {
        let mut mul_result = vec![vec![1f64; self.c]; self.r];
        for r in 0..self.r {
            for c in 0..self.c {
                mul_result[r][c] = self.m[r][c] * factor
            }
        }
        return Matrix {
            r: self.r,
            c: self.c,
            m: mul_result,
        };
    }
}

// Matric struct - user utils
/// This implementation block defines basic utilities for the struct Matrix
impl Matrix {
    /// The method displays the Matrix is a nicely formatted way
    pub fn show(&self) {
        println!("{}", self);
    }

    /// The method returns the two-dimentional vector content of the Matrix
    pub fn vector(&self) -> Vec<Vec<f64>> {
        self.m.to_owned()
    }

    /// The method returns the a tuple representing row and column size of the Matrix
    pub fn dims(&self) -> (usize,usize) {
        (self.r,self.c)
    }
}
// Matrix struct - traits
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Matrix ({},{})", self.r, self.c);
        for r in 0..self.r {
            println!("{:?}", self.m[r]);
        }
        write!(f, "")
    }
}
impl Default for Matrix {
    fn default() -> Self {Matrix::one(1, 1)}
}

