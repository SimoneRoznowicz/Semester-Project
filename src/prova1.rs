/*fn main() {
    println!("Hello, world!");
}*/
struct Point{
    xx: i32,
    yy: i32,
} 

impl Point {
    fn to_string(&mut self){
        println!("xx == {} e yy == {}", self.xx, self.yy);
    }
    fn change_coordinates(&mut self, a: i32, b:i32){
        self.xx = a;
        self.yy = b;
    }
    fn originate()-> Point{
        return Point{xx: 0, yy: 0};
    }
}

fn main(){
    let mut point: Point = Point{xx: 1,yy: 2};
    point.to_string();
    point.change_coordinates(100, 200);
    point.to_string();
    //let point2: <Point as Trait>::originate;//Point::originate();
    let mut point2 = Point::originate();
    point2.to_string();
    let mut ss1 = String::from("ciao ");
    let ss2 = String::from("mondo");
    let mut sstot = ss1.push_str(&ss2);
    println!("{}", ss1);
    println!("{}", ss2);
    println!("{:?}", sstot);

    let x = 2;
    for i in 1..10{
        match x {
            1 => println!("one"),
            2 => println!("two"),
            _ => println!("something else"),
        };
    }
    
}