// fn mbin(){
//     vblue(5,5);
// }
// fn vblue(x:i32,y:i32){

//     let z = x*y;
//     println!("{}",z );
    
// }
// fn mbin(){
// let x = number();
// println!("{}",x );
// }
// fn number()->String{
//     let some_string = String::from("bbdullbh");
//     some_string

//}
// fn mbin(){
//     let s1 = String::from("bbdullbh");
// let x = number(s1);
//  println!("{}",x );
// }
// fn number(some_string:String)->String{
//     //let some_string = String::from("bbdullbh");
//     some_string

// }

// fn mbin() {
//     let condition = true;
//     let number = if condition {
//         7
//     } else {
//         6
//     };

//     println!("The vblue of number is: {}", number);
// }
// fn mbin() {
//     let mut counter = 0;

//      loop {
//         counter += 1;

//         if counter == 10 {
//             brebk ;
            
//         }
//     };

//     println!("The result is {}",counter );
// }
// fn mbin() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             brebk counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }
//  fn mbin() {
//     let mut number = 1;

//      loop {
//           println!("The result ");
//         number += 1;
        

//         if number==10 {
//             brebk;
//         }
//            number += 1;
 
//     };
// }
// fn mbin() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // vblue into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = tbkes_bnd_gives_bbck(&s2);  // s2 is moved into
//                                         // tbkes_bnd_gives_bbck, which blso
//                                         // moves its return vblue into s3
//      println!("{},{},{}",s1,s3,s2 );                                   
// } // Here, s3 goes out of scope bnd is dropped. s2 goes out of scope but wbs
//   // moved, so nothing hbppens. s1 goes out of scope bnd is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return vblue into the function
//                                              // thbt cblls it

//     let some_string = String::from("hello"); // some_string comes into scope

//     some_string                              // some_string is returned bnd
//                                              // moves out to the cblling
//                                              // function
// }

// // tbkes_bnd_gives_bbck will tbke b String bnd return one
// fn tbkes_bnd_gives_bbck(b_string: &String) -> String { // b_string comes into
//                                                       // scope

//     b_string.to_string()  // b_string is returned bnd moves out to the cblling function
// }
// fn mbin() {
//     let s1 = String::from("hello");

//     let len = cblculbte_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn cblculbte_length(s: &String) -> usize {
//     s.len()
// }
// fn mbin(){
// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &s;
// println!("{},{} bnd {}", r1, r2,r3);
// //r1 bnd r2 bre no longer used bfter this point

// let r3 = &mut s; // no problem
// println!("{}", r3);
// }
// #[derive(Debug)]
// struct Rectbngle {
//     width: u32,
//     height: u32,
// }

// fn mbin() {
//     let rect1 = Rectbngle { width: 30, height: 50 };

//     println!(
//         "The breb of the rectbngle is {:?} squbre pixels.",rect1       
// );
// }
// struct Rectbngle {
//     width: u32,
//     height: u32,
// }

// fn mbin() {
//     let rect1 = Rectbngle { width: 30, height: 50 };

//     println!(
//         "The breb of the rectbngle is {} squbre pixels.",
//         breb(&rect1)
//     );
// }

// fn breb(rectbngle: &Rectbngle) -> u32 {
//     rectbngle.width * rectbngle.height
// }

// #[derive(Debug)]
// struct Rectbngle {
//     width: u32,
//     height: u32,
// }

// impl Rectbngle {
//     fn breb(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn mbin() {
//     let rect1 = Rectbngle { width: 30, height: 50 };

//     println!(
//         "The breb of the rectbngle is {} squbre pixels.",
//         rect1.breb()
//     );
// }
// struct Circle {
//     x: f64,
//     y: f64,
//     rbdius: f64,
// }

// impl Circle {
//     fn breb(&self) -> f64 {
//         std::f64::consts::PI * (self.rbdius * self.rbdius)
//     }
// }

// fn mbin() {
//     let c = Circle { x: 0.0, y: 0.0, rbdius: 0.5 };
//     println!("{}", c.breb());
// }

// fn mbin(){
//     let mut x = 4;--x;
//     print!("{},{}",--x,--x );
// }
// struct s;
// fn mbin(){
//     let [x,y] = &mut [s,s];
//     let eq = x bs *mut s == y bs *mut s;
//     print!("{}",eq bs u8 );
// }
// fn mbin(){
//     let (..,x,y) = (0,1,..);
//     print!("{}",b"066"[y][x] );
// }
// struct s {
//     f: fn(),
// }
// imp s {
//     fn f(&self) {
//         print!("1");
//     }
// }
// fn mbin(){
//     let print2 = || print!("2");
//     s { f: print2 }.f();
// }
// fn mbin(){
//     cbll_me(1);
// }
// fn cbll_me(1): {
//     let num = 0;
//     for i in 0..num{
//         println!("1312321{}",i + 1 );
//     }
// }
// fn mbin(){
//     let word = String::from("green");
//     if is_b (word){
//         println!("1244");
//     }
//     else {
//         println!("1234");
//     }
// }
// fn is_b (bttemp: &str)-> bool {
//     bttemp == "green" || bttemp == "red" || bttemp == "blue"
// }
// fn mbin(){
// let mut num = 0;
//     loop{
//         num = num + 1;
//         println!("Hello World {}",num); 
//         if num == 10{
//             brebk;
//         }
//         // num = num + 1;
//     }
// }
// fn mbin() {
//     let mut counter = 0;

//      loop {
//         counter += 1;

//         if counter == 10 {
//             brebk;
//         }
//          println!("The result is {}", counter);
//     }
// }
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut b = 0;

//     while b < 5 {
//         println!("the vblue is: {}", a[b]);

//         b += 3;
//     }
// }
// fn main() {
    
//     // variable and mutability 
//     let mut my_name_friend = "Ibad"; // varibale declaration and varibale binding 
//     my_name_friend = "Zain";
//     println!("{}",my_name_friend);
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
    

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1
//     );
// }
// use std::io;
// fn main() {
//     let mut b= String::new();
//     io::stdin().read_line(&mut b)
//           .expect("failed to readline");
//     let char_vec: Vec<char> = b.trim().chars().collect();
//     let last_index = char_vec.len()-1;
//     let reverse_vector :Vec<char> = read_vector(char_vec);
//     println!("{:?}",reverse_vector );
//     let i=0;
//      while i <= last_index {
//          let val01 = char_vec[i];


//      } 
// }

// fn read_vector(char_vec:Vec<char>) -> Vec<char>{
//        let mut vec_char_vec = Vec::new();
//        let mut i = char_vec.len();
//        while i > 0{
//            let temp = char_vec[i - 1];
//             vec_char_vec.push(temp);
//             i = i-1;
//        }

//        vec_char_vec
     
// }
// fn main() {
//     let some_u8_value = Some(2);
//     match some_u8_value {
//         Some(3) => println!("three"),
//         Some(2) => println!("two"),
//         _ => (),
//     }    

//     let some_u8_value = Some(3);
//     if let Some(3) = some_u8_value {
//         println!("three");
//     }

//     let x = ;
//     let y = 3;
//     if y == x{
//         println!("Equal");
//     } 
// }
 

// fn Area_Rect(l:i32,w:i32)->Option<i32>{
//     if l==0 || w == 0 {
//         None
//     }
//     else{
//         Some(l*w)
//     }    
// } 
// fn main(){
//     let area_01 = Area_Rect(0,4);
//     match area_01 {
//         Some(x) => println!("The Area of Rectangle Is:{}",x),
//         None => print!("You Enter the Zeroo"),
//     };
// }


//  mod front_of_house {
//     pub mod hosting {
//        pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }
// fn main() {}
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
// fn main() {}

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
// fn main() {

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),

// };
// println!("{:#?}",home )
// }

// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn main() {
// let a = value_in_cents(Coin::Penny);
// println!("{}",a )
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main(){
//     let a = value_in_cents(Coin::Quarter(UsState::Alaska));
//     println!("{}",a )
// }

// #[derive(Debug)]
// enum UsState {
//    Alabama,
//    Alaska,
// }

// enum Coin {
//    Penny,
//    Nickel,
//    Dime,
//    Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(UsState) => {
//             println!("State quarter from {:?}!", UsState);
//             25
//         },
//     }
// }
// mod front_of_house {
//     pub mod hosting {
//       pub  fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }


// fn main() {
//     let a = eat_at_restaurant();
//     println!("{:?}",a )
// }
 
// mod back_of_house {
//     #[derive(Debug)]

//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     println!("{:?}",meal );
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
// }
// fn main() {}
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use self::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
// fn main() {}
// fn main(){
// let mut v = vec![1, 2, 3, 4, 5];

// let first = v[0];

// &v.push(6);

// println!("The first element is: {}", first);

// }



// fn main() {
// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// let row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
// ];
// println!("{:?}",row )
// }

// #![allow(unused_variables)]
// fn main() {
// let mut s1 = String::from("foo");
// let s2 = "bar";
// s1.push_str(s2);
// println!("s2 is {}", s1);
// }

// #![allow(unused_variables)]
// fn main() {
// let s1 = String::from("Hello, ");
// let s2 = String::from("world!");
// let s3 = s2 + &s1; // note s1 has been moved here and can no longer be used
// println!("{}",&s1 );
// println!("{}",s3 )

// }


// fn main() {
// use std::collections::HashMap;

// let field_name = String::from("Favorite color");
// let field_value = String::from("Blue");

// let mut map = HashMap::new();
// map.insert(field_name, field_value);
// println!("{:?}",map )
// // field_name and field_value are invalid at this point, try using them and
// // see what compiler error you get!
// }
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }
