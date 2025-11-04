
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




// without Enum and pattern matching
// fn main (){
//   let indi=  find_str(String::from("lmit"));
//   if indi!=-1{
//     println!("it contains a in the string")
//   }
//   else{
//     println!("no it does not contain string")
//   }
// fn find_str(x:String)->i32{
// for(index , val) in x.chars().enumerate(){
//     if val=='a'{
// return index as i32;

// }

// }
// return -1;
// }
// }


fn main(){
    let indi=find_str(String::from("mita"));
match indi {
    Some(val)=>println!("yes it contains {}" , val ),
    None=>println!("a is not in the string")
}

    fn find_str(x:String)->Option<i32>{
for(ind , val) in x.chars().enumerate(){
if val=='a'{
    return Some(ind as i32) ;
}

    }
    return None;
}
}