// //QNO: 01
// use std::io;
// fn main() {
//     println!("INPUT RADIUS");
//     let mut radius= String::new();
//     io::stdin().read_line(&mut radius)
//             .expect("failed to readline");
//      let radius: f64 = radius.trim().parse().expect("invalid input");
//     const PI: f64 = 3.14159265359;
//     let c = PI*radius*radius;

//     println!("area of circle is {}",c )
// }

// //QNO: 02
// use std::io;
// fn main(){
//     let mut num = String::new();
//     println!("ENTER NUMBER");
//     io::stdin().read_line(&mut num)
//        .expect("failed to read line");
//           let num: f32 = num.trim().parse().expect("invalid input");
//      if num < 0.0{
//          println!("NEGATIVE NUMBER ENTERED");
//      }     
//      else if num > 0.0 {
//          println!("POSITIVE NUMBER ENTERED");
//      }
//      else {
//          println!("ZERO ENTERED");
//      }
// }
// //QNO: 03
// use std::io;
// fn main(){
//        println!("enter numerator");
//     let mut numerator = String::new();
//     io::stdin().read_line(&mut numerator)
//         .expect("failed to read line");
//         let numerator: f32 = numerator.trim().parse().expect("invalid input");

//        println!("enter denominator");
//         let mut denominator = String::new();
//     io::stdin().read_line(&mut denominator)
//         .expect("failed to read line");
//         let denominator: f32 = denominator.trim().parse().expect("invalid input");
//         let answer = numerator/denominator;
//         println!("{}",answer );
//         if answer > 0.0{
//         println!( "NUMBER {} IS COMPELETLY DIVISIBLE BY {}",numerator,denominator );
//         }
//         else {
//             println!( "NUMBER {} IS NOT COMPELETLY DIVISIBLE BY {}",numerator,denominator )
//         }
// }
// //QNO: 04
// use std::io;
// fn main() {
//     println!("ENTER RADIUS OF SPHERE");
//     let mut radius= String::new();
//     io::stdin().read_line(&mut radius)
//             .expect("failed to readline");
//      let radius: f64 = radius.trim().parse().expect("invalid input");
//      let v : f64 = 1.333333333;
//     const PI: f64 = 3.14159265359;
//     let c = v*PI*radius*radius*radius;

//     println!("volume of sphere with radius is {}",c );
// }
// //QNO: 05
// use std::io;
// fn main() {
//     println!("enter string");
//     let mut string = String::new();
//     io::stdin().read_line(&mut string)
//             .expect("failed to readline");
            
//            println!("how many copies of string you need");
//            let mut rate = String::new();
//     io::stdin().read_line(&mut rate)
//         .expect("failed to read line");
//         let rate: u32 = rate.trim().parse().expect("invalid input");
//         for i in 0..= rate {
//              println!("{}",string );
//         }
        
//  }
// //QNO: 06
// use std::io;
// fn main() {
//     println!("ENTER NUMEBR");
//     let mut num = String::new();
//     io::stdin().read_line(&mut num)
//             .expect("failed to readline");
//             let num: i32 = num.trim().parse().expect("invalid input");
//             let x = num % 2;
//     if x == 0 {
//         println!("{} is even",num );}
//     else {
//         println!("{} is odd",num );
//     }
//     }
// //QNO: 07
// use std::io;
// fn main() {
//     println!("ENTER CHARACTER");
//     let mut vowels = String::new();
//     io::stdin().read_line(&mut vowels)
//             .expect("failed to readline");
//             let vowels: char = vowels.trim().parse().expect("invalid input");
//             match vowels{
//               'a'| 'A' =>println!("LETTER {} IS VOWEL",vowels ),
//               'e'| 'E' =>println!("LETTER {} IS VOWEL",vowels ),
//               'i'| 'I' =>println!("LETTER {} IS VOWEL",vowels ),
//               'o'| 'O' =>println!("LETTER {} IS VOWEL",vowels ),
//               'u'| 'U' =>println!("LETTER {} IS VOWEL",vowels ),
//               _=>println!("LETTER {} IS NOT VOWEL",vowels ),
//             }          
// }
// //QNO: 08
// use std::io;
// fn main() {
//     println!("enter magnitude of triangle base");
//     let mut base = String::new();
//     io::stdin().read_line(&mut base)
//             .expect("failed to readline");
//      let base: f64 = base.trim().parse().expect("invalid input");

    
//     println!("enter magnitude of triangle heght");
//     let mut height= String::new();
//     io::stdin().read_line(&mut height)
//             .expect("failed to readline");
//      let height: f64 = height.trim().parse().expect("invalid input");
//      const c : f64 = 0.5;
//      let triangle = height * base * c;
//      println!("area of triangle with height {} and base{} is {}",height,base,triangle );
// }
// //QNO: 10
// use std::io;
// fn main() {
//     println!("enter coordinate for x1");
//     let mut x1 = String::new();
//     io::stdin().read_line(&mut x1)
//             .expect("failed to readline");
//             let x1: f32 = x1.trim().parse().expect("invalid input");

//             println!("enter coordinate for x2");
//                 let mut x2 = String::new();
//     io::stdin().read_line(&mut x2)
//             .expect("failed to readline");
//             let x2: f32 = x2.trim().parse().expect("invalid input");

//             println!("enter coordinate for y1");
//                 let mut y1 = String::new();
//     io::stdin().read_line(&mut y1)
//             .expect("failed to readline");
//             let y1: f32 = y1.trim().parse().expect("invalid input");

//             println!("enter coordinate for y2");
//                 let mut y2 = String::new();
//     io::stdin().read_line(&mut y2)
//             .expect("failed to readline");
//             let y2: f32 = y2.trim().parse().expect("invalid input");

//             let b = (x2-x1)*(x2-x1);

//             let c = (y2-y1)*(y2-y1);

//             const a : f32 = 0.5 ;

//             let d = b as f32 + c as f32 ;
        
//             let distance = d * a ;
//             println!("Distance between point{:?}and{:?} is {}",(x1,x2),(y1,y2),distance );
// }
// //QNO: 13
// use std::io;
// fn main(){
//     println!("ENTER VALUE OF N");
//     let mut rate= String::new();
//   io::stdin().read_line(&mut rate)
//              .expect("failed to readline");
//      let rate: i32 = rate.trim().parse().expect("invalid input");

// let mut num: i32 = rate;
// let mut result: i32 = 0;
//     while num > 0{
//         result = result + num;
//         num = num - 1;
//     }
//     println!("{}",result );
// }
// //QNO: 11
// use std::io;
// fn main(){
//     println!("enter height in feet ");
//    let mut height= String::new();
//   io::stdin().read_line(&mut height)
//              .expect("failed to readline");
//      let height: f32 = height.trim().parse().expect("invalid input");
//      let cm :f32 = 30.48;
//      let H = height*cm;
//      println!("There are {} C M in {} ft", H , height );
// }
// //QNO: 12
// use std::io;
// fn main(){
//     println!("enter height in cm ");
//    let mut height= String::new();
//   io::stdin().read_line(&mut height)
//              .expect("failed to readline");
//      let height: f32 = height.trim().parse().expect("invalid input");
//        let A = height / 100 as f32;
//        let W = A * A;

//     println!("enter weight in kg ");
//    let mut weight= String::new();
//   io::stdin().read_line(&mut weight)
//              .expect("failed to readline");
//      let weight: f32 = weight.trim().parse().expect("invalid input");

//      let BMI = weight/W;
//      println!("{}",BMI );
// }
// //QNO: 14
// use std::io;
// fn main(){
//     println!("enter a number");
//    let mut string= String::new();
//   io::stdin().read_line(&mut string)
//              .expect("failed to readline");

//     let mut result = 0 ;
//     for c in string.chars(){

//     let c:i32 = (c.to_string()).parse().unwrap();

//          result = result + c ;
//                println!("sum of {} is {}",string,result );
//          }
         
//     } 

// // QNO: 15 decimal to binary
// use std::io;
// fn main(){
//     println!("enter decimal no :");
//     let mut a= String::new();
//   io::stdin().read_line(&mut a)
//              .expect("failed to readline");
//      let a: i32 = a.trim().parse().unwrap(); 

// let mut num: i32 = a;
// let mut rem: i32 = 0;
//     while num > 0{
//          rem = num % 2;
//         num = num / 2;
//       println!("{}",rem);

//     }
// }
// //QNO: 09
// use std::io;
// fn main(){
//   println!("Please enter principal amount");
//     let mut principal= String::new();
//   io::stdin().read_line(&mut principal)
//              .expect("failed to readline");
//      let principal: f32 = principal.trim().parse().unwrap(); 

//         println!("Please Enter Rate of interest in");
//          let mut a= String::new();
//   io::stdin().read_line(&mut a)
//              .expect("failed to readline");
//      let a: f32 = a.trim().parse().unwrap(); 

//         println!("Enter number of years for investment");
//          let mut year= String::new();
//   io::stdin().read_line(&mut year)
//              .expect("failed to readline");
//      let year: i32 = year.trim().parse().unwrap(); 
//      let inter:f32;
//      let mut interest:f32 = principal;

//     for i in 1..=year{
//         let inter = interest *a;
//          interest =interest + inter;
//     }
//     println!("After {} years your principle amount {} over an interest rate of {}% will be {}",year,principal,a,interest )
// }
// //QNO: 20
use std::io;
fn main(){
    println!("select the following option which you need");
    println!("1: + , 2: - , 3: * 4: / , 5: ^");
    println!("enter option :");
     let mut num_1= String::new();
  io::stdin().read_line(&mut num_1)
         .expect("failed to readline");
  let num_1: u32 = num_1.trim().parse().expect("invalid input");   
  

  let option=[1,2,3,4,5];

  if num_1== option[0]{
      println!("enter first number");
         let mut a= String::new();
  io::stdin().read_line(&mut a)
         .expect("failed to readline");
  let a: f32 = a.trim().parse().expect("invalid input");
    println!("enter second number");
     let mut b= String::new();
  io::stdin().read_line(&mut b)
         .expect("failed to readline");
  let b: f32 = b.trim().parse().expect("invalid input");
  let addition = a + b ;
  println!("{}",addition );
  }          
  else if num_1==option[1] {
       println!("enter first number");
      let mut a= String::new();
  io::stdin().read_line(&mut a)
         .expect("failed to readline");
  let a: f32 = a.trim().parse().expect("invalid input");
println!("enter second number");
  let mut b= String::new();
  io::stdin().read_line(&mut b)
         .expect("failed to readline");
  let b: f32 = b.trim().parse().expect("invalid input");
  let subtraction = a - b ;
  println!("{}",subtraction );
  }
  else if num_1==option[2]{
       println!("enter first number");
      let mut a= String::new();
  io::stdin().read_line(&mut a)
         .expect("failed to readline");
  let a: f32 = a.trim().parse().expect("invalid input");
println!("enter second number");
  let mut b= String::new();
  io::stdin().read_line(&mut b)
         .expect("failed to readline");
  let b: f32 = b.trim().parse().expect("invalid input");
  let multiplication = a * b;
  println!("{}",multiplication );
  }
  else if num_1==option[3]{
       println!("enter first number");
      let mut a= String::new();
  io::stdin().read_line(&mut a)
         .expect("failed to readline");
  let a: f32 = a.trim().parse().expect("invalid input");
println!("enter second number");
  let mut b= String::new();
  io::stdin().read_line(&mut b)
         .expect("failed to readline");
  let b: f32 = b.trim().parse().expect("invalid input");
  let division = a / b ;
  println!("{}",division );
  }
  else if num_1==option[4]{
 println!("enter first number");
      let mut a= String::new();
  io::stdin().read_line(&mut a)
         .expect("failed to readline");
  let a: u32 = a.trim().parse().expect("invalid input");
println!("enter second number");
  let mut b= String::new();
  io::stdin().read_line(&mut b)
         .expect("failed to readline");
  let b: u32 = b.trim().parse().expect("invalid input");
  let exponent=u32::pow(a,b) ;
  println!("{}",exponent );
  }
  println!("BYE NOW");
}
// //QNO: 18
// use std::io;
// fn main() {
//      println!("Enter the text : ");
//     let mut a = String ::new();  
//      io::stdin().read_line(&mut a);
//      let a:String = a.trim().parse().unwrap();

//      let mut b :String = a.chars().rev().collect();
//      if a == b{
//       println!("{} is palindrome",a);}
//      else {
//      println!("{} is not palindrome",a);}
// }
//QNO: 17
// use std::io;
// fn main() {
//   let mut input = String ::new();
//   println!("Enter alphabet");
//      io::stdin().read_line(& mut input);
//      let mut a = input.len();
//      let mut vowel :i32 = 0; 
//      let mut cons :i32 = 0;
// let v : Vec<char> = input.chars().collect();
// for i in 0..a {
// if v[i] == 'a' { vowel = vowel+1;}
// else if v[i] == 'e' { vowel = vowel+1;}
// else if v[i] == 'i' { vowel = vowel+1;}
// else if v[i] == 'o' { vowel = vowel+1;}
// else if v[i] == 'u' { vowel = vowel+1;}
// else if v[i] == 'A' { vowel = vowel+1;}
// else if v[i] == 'E' { vowel = vowel+1;}
// else if v[i] == 'I' { vowel = vowel+1;}
// else if v[i] == 'O' { vowel = vowel+1;}
// else if v[i] == 'U' { vowel = vowel+1;}
// else { cons = cons+1;}
// }
// println!( "Vowels = {}
// Consonants = {}",vowel,cons-2);
// } 
// //QNO: 19
// use std::io;
// fn main() {
//   let mut input = String ::new();
//   println!("Enter the text");
//      io::stdin().read_line(& mut input);
//      let mut a = input.len();
//      let mut alphabets :i32 = 0; 
//      let mut numbers :i32 = 0;
//      let mut special :i32 = 0;
// let v : Vec<char> = input.chars().collect();
// for i in 0..a {
// if v[i] >= 'a' as  char && v[i] <= 'z' as char || v[i] >= 'A' as  char && v[i] <= 'Z' as char{ alphabets = alphabets+1;}
// else if v[i] >= '0' as char && v[i] <= '9' as char { numbers = numbers+1;}
// else { special = special+1;}
// }
// println!( "Alphabets = {} , Numbers = {} , Special Symbols = {}",alphabets,numbers,special-2);
// } 
// //QNO: 16
// use std::io;
// fn main(){
//          let mut binary= String::new();
//          println!("Enter a Binary number:");
//   io::stdin().read_line(&mut binary)
//           .expect("failed to readline");
//           let binary: i32 = binary.trim().parse().expect("invalid input");
//           let mut num = binary;
//           let mut decimal_val = 0;
//           let mut base = 1;
//           let mut rem = 0;
//           while (num > 0)
//     {
//         rem = num % 10;
//         decimal_val = decimal_val + rem * base;
//         num = num / 10 ;
//         base = base * 2;
//     }
//     println!("Decimal Representation of {} is {}",binary,decimal_val )
// }
