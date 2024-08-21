#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use rand::prelude::*;
use std::fmt;
use crate::rmat::utils::Arithmetic;

pub mod utils;
// structs
#[derive(Debug,Clone,PartialEq)]
pub struct Matrix {
    pub r: usize,
    pub c: usize,
    pub m: Vec<Vec<f64>>,
}

// Matrix struct - builders
impl Matrix {
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
    pub fn one(rows: usize, cols: usize) -> Matrix {
        let mut matrix = vec![vec![1f64; cols]; rows];
        Matrix {
            r: rows,
            c: cols,
            m: matrix,
        }
    }
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
impl Matrix {    
    #[allow(non_snake_case)]
    pub fn T(&self) -> Matrix {
        let mut res = vec![vec![1f64; self.r]; self.c];
        for r in 0..self.c {
            for c in 0..self.r {
                res[r][c] = self.m[c][r];
                // res[r][c] = 1.0;
            }
        }
        return Matrix {
            r: self.c,
            c: self.r,
            m: res,
        };
    }
    pub fn dot(&self, other: &Matrix) -> Matrix {
        if self.c != other.r {
            panic!("Dot - dimissions mismatch");
        } else {
            let mut res = vec![vec![1f64; other.c]; self.r];
            for r in 0..self.r {
                for c in 0..other.c {
                    let a = &self.m[r];
                    let b = match Matrix::vector_to_column(&other, c) {
                        Err(msg) => panic!("{}", msg),
                        Ok(m) => m,
                    };
                    res[r][c] = match Matrix::vector_sumproduct(a, &b) {
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
    pub fn add(&self, other: &Matrix) -> Matrix {
        match Matrix::arithmetic_op(self, other, Arithmetic::Add) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }
    pub fn sub(&self, other: &Matrix) -> Matrix {
        match Matrix::arithmetic_op(self, other, Arithmetic::Sub) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }
    pub fn mul(&self, other: &Matrix) -> Matrix {
        match Matrix::arithmetic_op(self, other, Arithmetic::Mul) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }
    pub fn div(&self, other: &Matrix) -> Matrix {
        match Matrix::arithmetic_op(self, other, Arithmetic::Div) {
            Err(msg) => panic!("{}", msg),
            Ok(mat) => return mat,
        }
    }
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
impl Matrix {
    pub fn show(&self) {
        println!("{}", self);
    }
    // pub fn owned_copy(&self) -> Matrix {
    //     let mut res = vec![vec![1f64; self.c]; self.r];
    //     for r in 0..self.r {
    //         for c in 0..self.c {
    //             res[r][c] = self.m[r][c]
    //         }
    //     }
    //     return Matrix {
    //         r: self.r,
    //         c: self.c,
    //         m: res,
    //     };
    // }
    pub fn to_vector(&self) -> Vec<Vec<f64>> {
        self.m.to_owned()
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

// macros
#[macro_export]
macro_rules! matrix {
    ($(    [ $($x: expr),*]    ),* ) => {
        {
            let mut v = Vec::new();
            $(

                v.push({let mut v = Vec::new();
                $(
                    v.push($x);
                )*
                v
                });

            )*
            v
        }
    };
}