//Question1
enum Shape {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle)
}

struct Triangle{
    a: f64,
    b: f64,
    c: f64
}

struct Rectangle{
    length: f64,
    width: f64
}

struct Circle{
    radius: f64
}

impl Shape {
    fn make_triangle(a: f64, b: f64, c: f64) -> Result<Shape, String> {
        if a <= 0.0 || b <= 0.0 || c <= 0.0{
            Err("Triangle Side Lengths Must be Non-Negative and Non-Zero.".to_string())
        }
        else if a+b <= c || a+c <= b || b+c <= a{
            Err("Triangle Inequality Does Not Hold".to_string())
        }
        else{
            Ok(Shape::Triangle(Triangle{a,b,c}))
        }
    }
    fn make_rectangle(length: f64, width: f64) -> Result<Shape, String> {
        if length <= 0.0 || width <= 0.0{
            Err("Sides Must be Non-Negative and Non-Zero.".to_string())
        }

        else{
            Ok(Shape::Rectangle(Rectangle{length, width}))
        }
    }
    fn make_circle(radius: f64) -> Result<Shape, String> {
        if radius <= 0.0{
            Err("Radius Must be Non-Negative and Non-Zero.".to_string())
        }
        
        else{
            Ok(Shape::Circle(Circle{radius}))
        }
    }
    fn area(&self) -> f64{
        match self{
            Shape::Triangle(triangle) => triangle.area(),
            Shape::Rectangle(rectangle) => rectangle.area(),
            Shape::Circle(circle) => circle.area()
        }
    }
    fn perimeter(&self) -> f64{
        match self{
            Shape::Triangle(triangle) => triangle.perimeter(),
            Shape::Rectangle(rectangle) => rectangle.perimeter(),
            Shape::Circle(circle) => circle.perimeter()
        }
    }
    fn double_perim(&self) -> f64{
        self.perimeter() * 2.0
    }
 }
 impl Triangle{
    fn area(&self) -> f64{
        let side = self.perimeter() / 2.0;
        side * (side-self.a)* (side-self.b) * (side-self.c).sqrt()
    }
    fn perimeter(&self) -> f64{
        self.a + self.b + self.c
    }
 }
 impl Rectangle{
    fn area(&self) -> f64{
        self.length * self.width
    }
    fn perimeter(&self) -> f64{
        2.0 *self.length + 2.0 *self.width
    }
 }
 impl Circle{
    fn area(&self) -> f64{
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64{
        2.0 * std::f64::consts::PI * self.radius
    }
 }


fn main() {
    let triangle = Shape::make_triangle(3.0, 4.0, 5.0).unwrap();
    let rectangle = Shape::make_rectangle(3.0, 5.0).unwrap();
    let circle = Shape::make_circle(3.0).unwrap();

    println!("Triangle: Area = {:.2}, Perimeter = {:.2}, Double Perimeter = {:.2}", triangle.area(), triangle.perimeter(), triangle.double_perim());
    println!("Rectangle: Area = {:.2}, Perimeter = {:.2}, Double Perimeter = {:.2}", rectangle.area(), rectangle.perimeter(), rectangle.double_perim());
    println!("Circle: Area = {:.2}, Perimeter = {:.2}, Double Perimeter = {:.2}", circle.area(), circle.perimeter(), circle.double_perim());
}
