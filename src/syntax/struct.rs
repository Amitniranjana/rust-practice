struct Rectangle{
    width:i32,
    length:i32
}
impl Rectangle{
fn area(&self)->i32{
    return self.width *self.length;
}
fn perimeter(&self)->i32{
    return 2*(self.length + self.width);
}
}
fn main(){
let rect1=Rectangle{
    width:10,
    length:20
};
println!("area is {} and perimeter is {}" , rect1.area() , rect1.perimeter())
}