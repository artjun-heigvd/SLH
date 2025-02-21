#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Unit {
    Volts,
    Amps,
    Watts,
    Ohms,
}

impl Unit {
    fn mul(self, rhs: Unit) -> Unit {
        use Unit::*; 
        match (self, rhs) {
            (Volts, Amps) => Watts,
            _ => todo!(),
        }
    }

    fn div(self, rhs: Unit) -> Unit {
        use Unit::*;
        todo!()
    }
}

#[derive(Debug,Clone,Copy)]

struct Physical {
    value: f64,
    unit: Unit,
}

impl Physical {
    fn add(self, rhs: Physical) -> Physical {
        todo!()
    }

    fn mul(self, rhs: Physical) -> Physical {
        todo!()
    }

    fn div(self, rhs: Physical) -> Physical {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn phys_eq(x: Physical, y: Physical) {
        if x.unit != y.unit {
            panic!("Wrong unit: expected {:?} and got {:?}", y.unit, x.unit);
        }
        let (x,y) = (x.value, y.value);
        if (x - y).abs() > 0.00001 * x.abs() {
            panic!("{x} and {y} are too different");
        }
    }

    #[test]
    fn addition() {
        let u1 = Physical { value: 1.0, unit: Unit::Volts };
        let u2 = Physical { value: 2.0, unit: Unit::Volts };
        let u = u1.add(u2);
        phys_eq(u, Physical { value: 3.0, unit: Unit::Volts });
    }

    #[test]
    #[should_panic]
    fn bad_add() {
        Physical { value: 1.0, unit: Unit::Volts }
            .add(Physical { value: 1.0, unit: Unit::Amps });
    }

    #[test]
    fn loi_de_puissance() {
        let u = Physical { value: 5.5, unit: Unit::Volts };
        let i = Physical { value: 0.01, unit: Unit::Amps };
        let p = u.mul(i);
        phys_eq(p, Physical { value: 0.055, unit: Unit::Watts });
    }


    #[test]
    fn loi_d_ohm() {
        let r = Physical { value: 50.0, unit: Unit::Ohms };
        let i = Physical { value: 0.01, unit: Unit::Amps };
        let u = r.mul(i);
        phys_eq(u, Physical { value: 0.5, unit: Unit::Volts });
    }

    #[test]
    fn loi_d_ohm_2() {
        let u = Physical { value: 0.5, unit: Unit::Volts };
        let i = Physical { value: 0.01, unit: Unit::Amps };
        let r = u.div(i);
        phys_eq(r, Physical { value: 50.0, unit: Unit::Ohms });
    }

    #[test]
    #[should_panic]
    fn bad_mul() {
        Physical { value: 1.0, unit: Unit::Volts }
            .mul(Physical { value: 1.0, unit: Unit::Volts });
    }

}

