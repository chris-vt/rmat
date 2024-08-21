/// This macro allows to create a two-dimensional vector of f64 that can be fed as input to a Matrix struct
/// ## Examples
/// ```
/// use rmat::{Matrix,matrix};
/// let x = Matrix::usr(
///        matrix![[1.0,2.0],[3.0,4.0]]
///    );
/// x.show();
/// ```
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