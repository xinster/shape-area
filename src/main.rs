
fn main() {
    //Rectangle
    let r = Rectangle{
        width:4.0,
        height:8.0
    };
    let area: f64 = calc_area(&r);
    println!("The rectangle area is {:.2}", area);
    
    //Circle
    let c = Circular{
        radius:8.0
    };
    let area: f64 = calc_area(&c);
    println!("The circle area is {:.2}", area);
    
    //Triangle
    let t = Triangle{
        end_side:20.0,
        height:18.0,
    };
    let area: f64 = calc_area(&t);
    println!("The triangle area is {:.2}", area);
    
}

//calculate area
fn calc_area<T: Geometry>(geom : &T) -> f64 {
    geom.get_area()
}

//define a trait，with a area calculation function
trait Geometry {
    fn get_area(&self) -> f64;
}

//define a rectangle
struct Rectangle {
    width: f64,
    height: f64
}

//Rectangle
impl Geometry for Rectangle {
    fn get_area(&self) -> f64 {
        return &self.width * &self.height;
    }
}

//define a circle
struct Circular {
    radius: f64,
}

//Circle
impl Geometry for Circular {
    fn get_area(&self) -> f64 {
        const PAI: f64 = 3.14;
        return PAI * &self.radius * &self.radius;
    }
}

//Define a triangle
struct Triangle {
    end_side: f64, 
    height: f64,
}

//Triangle
impl Geometry for Triangle {
    fn get_area(&self) -> f64 {
        return &self.end_side * &self.height / 2.00;
    }
}
