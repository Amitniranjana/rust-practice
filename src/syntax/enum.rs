
enum Shape {
Circle(f32) ,
Rectangle(f32 ,f32)
}

fn main(){
let circle =Shape::Circle(3.0);
let rect=Shape::Rectangle(2.0, 4.0);
println!("circle area {}",calculated_area(circle));
println!("rectangle area {}" ,calculated_area(rect));
}
fn calculated_area(shape :Shape)->f32{
  let area= match shape{
    Shape::Rectangle(a ,b)=>a*b,
    Shape::Circle(r)=>3.14*r*r,
  };
  return area;
}