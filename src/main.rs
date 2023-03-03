struct EllipticCurve {
    a: i128,
    b: i128,
    c: i128,
    n: i128
}

struct Point {
    x: i128,
    y: i128
}

impl EllipticCurve {
    fn init_default() -> EllipticCurve {
        EllipticCurve {a: 0, b: 0, c: 0, n: 0}
    }

    fn init_points(a: i128, b: i128, c: i128) -> EllipticCurve {
        EllipticCurve {a: a, b: b, c: c, n: 0}
    }

    fn init_points_ord(a: i128, b: i128, c:i128, n: i128) -> EllipticCurve {
        EllipticCurve {a: a, b: b, c: c, n: n}
    }

    fn is_point_in(&self, point: Point) -> bool {
        let left = point.y.pow(2) % self.n;
        let right = (point.x.pow(3) + self.a * point.x.pow(2) + self.b * point.x + self.c) % self.n;

        left == right
    }

    fn print(&self) {
        println!("a: {}, b: {}, c: {}, n: {}", self.a, self.b, self.c, self.n);
    }
}


fn main() {
    let ec_test = EllipticCurve::init_points_ord(0, 0, 1, 2);
    ec_test.print();
    let success = Point { x: 0, y: 1 };
    let fail = Point { x: 0, y: 0 };
    println!("Success is {}", ec_test.is_point_in(success));
    println!("Fail is {}", ec_test.is_point_in(fail));
}
