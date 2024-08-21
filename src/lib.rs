mod rmat;

#[cfg(test)]
mod tests {
    use crate::{matrix, rmat::Matrix};
    #[test]
    fn test_identity_matrix() {
        let x = Matrix::id(3);
        let y = matrix![
            [1.0,0.0,0.0],
            [0.0,1.0,0.0],
            [0.0,0.0,1.0]
        ];
        assert_eq!(y,x.to_vector());
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
}

