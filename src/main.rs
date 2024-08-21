use rmat::{matrix,Matrix};

fn main() {
    let x = Matrix::usr(
        matrix![[1.0,2.0],[3.0,4.0]]
    );
    x.show();
}