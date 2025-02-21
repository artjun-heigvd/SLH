use std::ops::{Div, Mul, Sub, Add};
use typenum::{N1, N2, N3, P1, P2, Z0};

#[derive(Clone,Copy,Debug, Default)]
struct Unit<M, K, S, A>(M,K,S,A);

type Watts = Unit<P2, P1, N3, Z0>; 
type Amps = Unit<Z0, Z0, Z0, P1>;

// V = m² * kg * s^(-3) * A^(-1) 
//type Volts = ()

// Ohm = m² * kg * s^(-3) * a^(-2)
//type Ohms = ()

impl <M1, K1, S1, A1, M2, K2, S2, A2> Mul<Unit<M2,K2,S2,A2>>
for Unit<M1,K1,S1,A1>
    where M1: Add<M2>,
          K1: Add<K2>,
          S1: Add<S2>,
          A1: Add<A2>
{
    type Output = Unit<M1::Output, K1::Output, S1::Output, A1::Output>;

    fn mul(self, rhs: Unit<M2,K2,S2,A2>) -> Self::Output {
        todo!()
    }
}


#[derive(Clone,Copy,Debug)]
struct Physical<U> {
    value: f64,
    unit: U,
}

impl <U: Default> From<f64> for Physical<U> {
    fn from(value: f64) -> Self {
        todo!()
    }
}

impl <U,V> Mul<Physical<V>> for Physical<U>
    where U: Mul<V>
{
    type Output = ();

    fn mul(self, rhs: Physical<V>) -> Self::Output {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn phys_eq<U>(x: Physical<U>, y: Physical<U>) {
        let (x,y) = (x.value, y.value);
        if (x - y).abs() > 0.00001 * x.abs() {
            panic!("{x} and {y} are too different");
        }
    }

    /*
    #[test]
    fn addition() {
        let u1: Physical<Volts> = 1.0f64.into();
        let u2: Physical<Volts> = 2.0f64.into();
        let u: Physical<Volts> = u1 + u2;
        phys_eq(u, 3.0f64.into());
    }


    #[test]
    fn loi_de_puissance() {
        let u: Physical<Volts> = 5.5f64.into();
        let i: Physical<Amps> = 0.01.into();
        let p: Physical<Watts> = u * i;
        phys_eq(p, 0.055.into());
    }


    #[test]
    fn loi_d_ohm() {
        let r: Physical<Ohms> = 50.0f64.into();
        let i: Physical<Amps> = 0.01f64.into();
        let u: Physical<Volts> = r * i;
        phys_eq(u, 0.5f64.into());
    }

    #[test]
    fn loi_d_ohm_2() {
        let u: Physical<Volts> = 0.5f64.into();
        let i: Physical<Amps> = 0.01f64.into();
        let r: Physical<Ohms> = i / u;
        phys_eq(r, 50f64.into());
    }

    */
}