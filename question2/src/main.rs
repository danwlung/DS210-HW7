struct Polygon{
    num_sides: f64,
    side_length: f64
}

trait PolyTrait{
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64;
}

impl Polygon {
    fn new_polygon(num_sides: f64, side_length: f64) -> Self{
        Polygon{num_sides, side_length}
    }
}

impl PolyTrait for Polygon {
    fn perimeter(&self) -> f64{
        self.num_sides * self.side_length
    }
    fn area(&self) -> f64{
        self.perimeter() * self.radius() * 0.5
    }
    fn radius(&self) -> f64{
        self.side_length / (2.0 * (std::f64::consts::PI / self.num_sides).tan())
    }
}
fn main() {
    let polygon_sizes = [6, 12, 24, 128, 256, 512, 1024, 2048, 65536];
    let side_lengths = [1.0, 5.0, 10.0];
    
    for side in polygon_sizes{
        for length in side_lengths{
            let polygon = Polygon::new_polygon(side as f64, length);
            let circle = std::f64::consts::PI * polygon.radius() * polygon.radius();

            println!("For a Polygon with {:} sides and side lengths of {:.1}", side, length);
            println!("Apothem is: {:.2}", polygon.radius());
            println!("Area is: {:.2}", polygon.area());
            println!("Circle Area: {:.2}", circle);
        }
    }
}

