
/// Sorcellerie pour simplifier la généricité
pub trait Number: where
    Self: Copy,
    Self: std::ops::Add<Self, Output=Self>,
    Self: std::ops::Mul<Self, Output=Self>,
    Self: std::ops::AddAssign<Self>,
    Self: std::ops::MulAssign<Self>,
    Self: From<u8>,
    {}

impl <T> Number for T where 
    Self: Copy,
    Self: std::ops::Add<Self, Output=Self>,
    Self: std::ops::Mul<Self, Output=Self>,
    Self: std::ops::AddAssign<Self>,
    Self: std::ops::MulAssign<Self>,
    Self: From<u8>,
{}

#[allow(unused_macros)]
macro_rules! matrix {
    ( $( $( $x:expr),*  );* ) => {{
        let mut m = Vec::new();
        $(
            let mut row = Vec::new();
            $(
                row.push($x);
            )*
            m.push(row);
        )*    
        super::check(m).expect("invalid matrix in macro")
    }}
}

#[allow(unused_imports)]
pub(crate) use matrix;