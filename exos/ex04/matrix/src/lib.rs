mod util;

use util::Number;

#[derive(Debug,PartialEq,Eq)]
pub struct Shape {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Matrix<T: Number>(Vec<Vec<T>>);

#[derive(Debug,PartialEq,Eq)]
pub enum ShapeError {
    MalformedData,
    IncompatibleShapes(Shape, Shape),
}

pub fn shape<T: Number>(m: &Vec<Vec<T>>) -> Option<Shape> {
    todo!()
}

pub fn check<T: Number>(m: Vec<Vec<T>>) -> Result<Matrix<T>, ShapeError> {
    todo!()
}

pub fn empty<T: Number>(shape: Shape) -> Matrix<T> {
    todo!()
}

pub fn identity<T: Number> (n: usize) -> Matrix<T> {
    todo!()
}

pub fn matsum<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, ShapeError> {
    todo!()
}

pub fn matmul<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, ShapeError> {
    todo!()
}

pub fn matsum_inline<T: Number>(a: &mut Matrix<T>, b: &Matrix<T>) -> Result<(), ShapeError> {
    todo!()
}

pub fn bidi_mul<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, ShapeError> {
    todo!()
}


#[cfg(test)]
mod tests {
    use util::matrix;
    use super::*;

    #[test]
    fn get_shape() {
        assert_eq!(shape(&vec![vec![1,2]]), Some(Shape{ rows: 1, columns: 2}));
        assert_eq!(shape(&vec![vec![1], vec![2]]), Some(Shape{ rows: 2, columns: 1}));
        assert_eq!(shape(&vec![vec![1,2], vec![3]]), None);
    }

    #[test]
    fn irregular_shape_check() {
        assert_eq!(check(vec![vec![1,2], vec![3]]), Err(ShapeError::MalformedData))
    }

    #[test]
    fn zero_sized_matrices() {
        assert_eq!(check::<u8>(vec![]), Err(ShapeError::MalformedData));
        assert_eq!(check::<u8>(vec![vec![], vec![]]), Err(ShapeError::MalformedData));
    }

    #[test]
    #[should_panic]
    fn null_identity() {
        identity::<u8>(0);
    }

    #[test]
    fn identities() {
        assert_eq!(identity(2), matrix![1u8,0; 0,1]);
        assert_eq!(identity(4), matrix![1u8,0,0,0; 0,1,0,0; 0,0,1,0; 0,0,0,1])
    }

    #[test]
    fn simple_add() {
        let one: Matrix<i64> = matrix![1, 2, 3; 0, 10, 20];
        let two = matrix![0, 100, 200; 4, 5, 6];
        let three = matrix![1, 102, 203; 4, 15, 26];

        assert_eq!(matsum(&one, &two), Ok(three));
    }

    #[test]
    fn simple_mul() {
        let one: Matrix<i64> = matrix![2; 3];
        let two = matrix![5, 7];
        let three = matrix![10, 14; 15, 21];

        assert_eq!(matmul(&one, &two), Ok(three));
    }

    #[test]
    fn inline_sum() {
        let mut one: Matrix<i64> = matrix![0,1; 1,0];
        let two = matrix![2,0; 0,4];
        matsum_inline(&mut one, &two).expect("matsum_inline fail");
        assert_eq!(one, matrix![2,1; 1,4]);
    }

    #[test]
    fn bidi_test() {
        let a = matrix![2,3];
        let b = matrix![7;11];
        let c = matrix![2,3,5];
        let d = matrix![0,0;0,0];

        assert_eq!(bidi_mul(&a, &b), Ok(matrix![2*7 + 3*11]));
        assert_eq!(bidi_mul(&b, &a), Ok(matrix![2*7, 3*7; 2*11, 3*11]));
        assert_eq!(bidi_mul(&c, &b), Ok(matrix![14, 21, 35; 22, 33, 55]));
        assert_eq!(bidi_mul(&c, &d), Err(ShapeError::IncompatibleShapes(Shape { rows: 1, columns: 3} , Shape { rows: 2, columns: 2 })))

    }

}
