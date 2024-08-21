use crate::{Matrix,matrix};

#[test]
fn test_identity_matrix() {
    let x = Matrix::id(3);
    let y = matrix![
        [1.0,0.0,0.0],
        [0.0,1.0,0.0],
        [0.0,0.0,1.0]
    ];
    assert_eq!(y,x.vector());
}
#[test]
fn test_clone() {
    let x = Matrix::id(3);
    let y = x.clone();
    assert_eq!(x,y);
}
#[test]
fn test_equal() {
    let x = Matrix::id(3);
    let y = Matrix::id(3);
    assert_eq!(x,y);
}
#[test]
fn test_default() {
    let x = Matrix::default();
    let y = matrix![[1.0]];
    assert_eq!(y,x.vector());
}
#[test]
fn test_dot_product() {
    let m1 = Matrix::rand(3, 5);
    let m2 = Matrix::rand(5, 2);
    let mdot = m1.dot(&m2);
    mdot.show();
}