use std::ops::Add;
use std::ops::Mul;

#[derive(Clone,Copy,Debug,Default)]
struct Volts;
#[derive(Clone,Copy,Debug,Default)]
struct Amps;
#[derive(Clone,Copy,Debug,Default)]
struct Watts;
#[derive(Clone,Copy,Debug,Default)]
struct Ohms;

trait UnitMul<V> {
    type Output: Default;
}

impl UnitMul<Amps> for Volts { type Output = Watts; }


struct Physical<Unit> {
    value: f64,
    unit: Unit,
}



impl <U,V> Mul<Physical<V>> for Physical<U>
    where U: UnitMul<V>
{
    type Output = Physical<U::Output>;

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
        let u1 = Physical { value: 1.0, unit: Volts };
        let u2 = Physical { value: 2.0, unit: Volts };
        let u = u1 + u2;
        phys_eq(u, Physical { value: 3.0, unit: Volts });
    }


    #[test]
    fn loi_de_puissance() {
        let u = Physical { value: 5.5, unit: Volts };
        let i = Physical { value: 0.01, unit: Amps };
        let p = u.mul(i);
        phys_eq(p, Physical { value: 0.055, unit: Watts });
    }


    #[test]
    fn loi_d_ohm() {
        let r = Physical { value: 50.0, unit: Ohms };
        let i = Physical { value: 0.01, unit: Amps };
        let u = r * i;
        phys_eq(u, Physical { value: 0.5, unit: Volts });
    }

    #[test]
    fn loi_d_ohm_2() {
        let u = Physical { value: 0.5, unit: Volts };
        let i = Physical { value: 0.01, unit: Amps };
        let r = u / i;
        phys_eq(r, Physical { value: 50.0, unit: Ohms });
    }

    */

}

