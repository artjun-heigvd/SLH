#[derive(Clone,Copy,PartialEq,PartialOrd,Debug)]
struct Volts(f64);

#[derive(Clone,Copy,PartialEq,PartialOrd,Debug)]

struct Amps(f64);


#[derive(Clone,Copy,PartialEq,PartialOrd,Debug)]
struct Watts(f64);

#[derive(Clone,Copy,PartialEq,PartialOrd,Debug)]
struct Ohms(f64);


impl Volts {
    fn mul_current(i: Amps) -> Watts {
        todo!()
    }

    fn div_current(i: Amps) -> Ohms {
        todo!()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    fn phys_eq(x: f64, y: f64) {
        if (x - y).abs() > 0.00001 * x.abs() {
            panic!("{x} and {y} are too different");
        }
    }

    /* 
    #[test]
    fn addition() {
        let u1 = Volts(1.0);
        let u2 = Volts(2.0);
        let u: Volts = u1.add_volts(u2);
        phys_eq(u.0, 3.0);
    }

    #[test]
    fn loi_de_puissance() {
        let u = Volts(5.5);
        let i = Amps(0.01);
        let p: Watts = u.mul_amps(i);
        phys_eq(p.0, 0.055);
    }

    #[test]
    fn loi_d_ohm() {
        let r = Ohms(50.0);
        let i = Amps(0.01);
        let u: Volts = r.mul_amps(i);
        phys_eq(u.0, 0.5);
    }

    #[test]
    fn loi_d_ohm_2() {
        let u = Volts(0.5);
        let i = Amps(0.01);
        let r: Ohms = u.div_amps(i);
        phys_eq(r.0, 50.0);
    }
    */


}