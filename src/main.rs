use modinverse::modinverse;

struct EllipticCurve {
    a: i128,
    b: i128,
    n: i128
}

#[derive(Clone)]
#[derive(Debug)]
struct Point {
    x: i128,
    y: i128
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl EllipticCurve {

    fn init(a: i128, b: i128) -> EllipticCurve {
        EllipticCurve {a, b, n: 0}
    }

    fn init_ord(a: i128, b: i128, n: i128) -> EllipticCurve {
        EllipticCurve {a, b, n}
    }

    fn is_point_in(&self, point: Point) -> bool {
        let left = point.y.pow(2).rem_euclid(self.n);
        let right = (point.x.pow(3) + self.a * point.x + self.b).rem_euclid(self.n);

        left == right
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for x in 0..self.n {
            for y in 0..self.n {
                let test_point = Point { x, y };
                
                if self.is_point_in(test_point.clone()) {
                    points.push(test_point);
                }
            }
        }

        points
    }

    fn add(&self, p1: Point, p2: Point) -> Point {
        if p1 == p2 {
            return self.double(p1);
        }

        let lambda = (((p2.y - p1.y).rem_euclid(self.n)) * modinverse((p2.x - p1.x).rem_euclid(self.n), self.n).unwrap()).rem_euclid(self.n);
        let x3 = (lambda.pow(2) - p1.x - p2.x).rem_euclid(self.n);
        let y3 = (lambda*(p2.x-p1.x)-p1.y).rem_euclid(self.n);

        Point { x: x3, y: y3 }
    }

    fn double(&self, point: Point) -> Point {
        let lambda = (((3*(point.x.pow(2)) + self.a).rem_euclid(self.n)) * modinverse((2*point.y).rem_euclid(self.n), self.n).unwrap()).rem_euclid(self.n);
        let x3 = (lambda.pow(2) - 2*point.x).rem_euclid(self.n);
        let y3 = (2 * lambda * point.x - point.y).rem_euclid(self.n);

        Point { x: x3, y: y3 }
    }

    fn print(&self) {
        if self.a != 0 && self.b != 0 {
            println!("y^3 = x^3 + {}x + {} mod {}", self.a, self.b, self.n);
        } else if self.a == 0 && self.b != 0 {
            println!("y^3 = x^3 + {} mod {}", self.b, self.n);
        } else if self.b == 0 &&  self.a != 0{
            println!("y^3 = x^3 + {}x mod {}", self.a, self.n);
        } else {
            println!("y^3 = x^3 mod {}", self.n);
        }
    }
}


fn main() {
    let ec_test = EllipticCurve::init_ord(3, 8, 13);
    ec_test.print();
    let points = ec_test.get_points();
    for point in points {
        println!("{:?}", point);
    }

    println!("{:?}", ec_test.double(Point { x: 12, y:2 }));
}
