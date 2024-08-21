//! The crate provides basic functionalities to create 2d matrices and apply basic algebra operations.
//! This is intended to be a toy implementation and not suitable for production.
//! 
//! 
//! ## Examples
//! 
//! Create an identity matrix
//! 
//! ```
//! use rmat::Matrix;
//! let s = 3; // size of identity matrix
//! let x = Matrix::id(s);
//! x.show();
//! ```
//! 
//! Create a matrix of random numbers between 0.0 and 1.0
//! 
//! ```
//! use rmat::Matrix;
//! let r = 3; // number of rows
//! let c = 5; // number of columns 
//! let x = Matrix::rand(r,c);
//! x.show();
//! ```
//! 
//! Create two matrices and calculate the dot product
//! 
//! ```
//! use rmat::Matrix;
//! let r1 = 3; // matrix 1 number of rows
//! let c1 = 5; // matrix 1 number of columns 
//! let m1 = Matrix::rand(r1,c1);
//! let r2 = 5; // matrix 2 number of rows - NOTE: c1 and r2 are equal
//! let c2 = 2; // matrix 2 number of columns 
//! let m2 = Matrix::rand(r2,c2);
//! let mdot = m1.dot(&m2);
//! mdot.show();
//! ```

mod matrix_module;
mod macros;
pub use crate::matrix_module::Matrix;

#[cfg(test)]
mod test;

