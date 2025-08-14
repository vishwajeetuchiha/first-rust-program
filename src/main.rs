// fn main() {
//     let num:u8 = 5;
//     println!("this is stored in num {}", num);

// }

// &str string
// String - Dynamic string - Heap Allocated
// &str - fixed lenght string - Special memory

// fn main() {
//     let mut string_literal = String::from("this is a string literal");
//     string_literal.push_str(" and this is added to the string literal");
//     println!("this is a string literal {}", string_literal);
//     }


// Tupple 

// fn main() {
//     let emp_info:(&str, u8) = ("vishu" , 25);

//     let emp_name = emp_info.0;
//     let emp_age = emp_info.1;

//     //destructuring
//     let (employee_name, employee_age) = emp_info;
//     println!("Employee name is {}, Employee Age= {}", employee_age,employee_name );
//     println!("Employee name is {}, Employee Age= {}", emp_age,emp_name );
// }

// Fuctions

// fn main(){
//     let num1:u8 = 10;
//     let num2:u8 = 20;
//     let result:u8 = add(num1,num2);
//     print!("the sum of num1 and num2 is {}", result);
    
// }

// fn add(item1:u8, item2:u8)->u8{
//    return item1 + item2;
// }

// fn print_value(item:u8){
//     println!("{}",item);
// }

// fn main() {
//     let outside_varibale = 10;
// {    
//     let inside_variable = 20;
//  println!("inside block {}", inside_variable);
//  println!("outside block {}", outside_varibale);
//     }

//     println!("outside variable {}", outside_varibale);
    

// } 

// fn main() {
//     let str1 = String::from("hello");//str1 in the owner of hello
//     let str2 = str1;//transfer of str2 is the new owner of hello
//     println!("str1 = {}", str1);
//     println!("str2 = {}", str2);
// }

// fn main() {
// let x:u8 = 5;// x memory
// process_integer(x);
// println!("x = {}", x);
// }

// fn process_integer(x:u8){
//     println!("x = {}", x);
// }
// fn process_integer(item:u8){
//     println!("item = {}", item);
// }


// fn main() {
// let x:String = String::from("hello");//x is the owner of hello
// process_string(x);//transfer of ownership
// // println!("x = {}", x);
// }


// fn process_string(item:String){//hello new owner is item
//     println!("item = {}", item);
// }

// fn main (){
//     let s1:String = get_string();
//     println!("s1 = {}", s1);//s1 is the owner of hello

//     let s2 = String::from("world");//s2 is the owner of world
//     let s3 = return_string(s2);//1.transfer of ownership 2.s2 is the new owner of world 3.s3 is the new owner of world
//     println!("s3 = {}", s3);//s3 is the new owner of world
// }

// fn get_string()->String{
//     let new_string = String::from("hello");//new_string is the owner of hello
//     return new_string;//transfer of ownership
// }
// //receive ownership of world
// fn return_string(recevied_string:String)->String{
//     return recevied_string;//transfer of ownership
// }

// fn main() {
//     let s1:String = String::from("hello");//s1 owner
//     let len = calculate_length(s1.clone());
//     println!("s1 = {}, len = {}", s1, len);
// }

// fn calculate_length(item:String)->usize{
//     let length:usize = item.len();
//     return length;
// }

// fn main() {
//     let s1:String = String::from("hello");
//     let len:usize = calculate_length(&s1);//borrowing operation
//     println!("s1 = {}, len = {}", s1, len);
// }

// fn calculate_length(s2:&String)->usize{
//     return s2.len();
// }

// fn main(){
//     let mut s1:String = String::from("hello");
//     let w1 = &mut s1;
//     w1.push_str(" world");
//     println!("w1 = {}", w1);

//     let w2 = &mut s1;
//     w2.push_str(" world");
//     println!("w2 = {}", w2);
// } 

// fn main() {
//     let mut s1:String = String::from("hello");
//     let r1 = &s1;
//     let r2 = &s1;

//     println!("r1 = {}, r2 = {}", r1, r2);
//     {
//         let w1 = &mut s1;
//         w1.push_str(" world");
//         println!("w1 = {}", w1);
//     }
//     let w2 = &mut s1;
//     w2.push_str(" world");
//     println!("w2 = {}", w2);
// }

// fn main() {
//     let x =5;
//     let y = &x;//y is reference to the value of x 
//     print!("y = {}", y);
// }

// fn main() {
//     let s1: String = String::from("hello");
//     let len: usize = calculate_length(&s1);
//     print!("s1 = {}, len = {}", s1, len);
// }

// fn calculate_length(s2: &String) -> usize {
//     return s2.len();// auto dereferencing
// }

// fn main() {
//     let reference_to_nothing = create_string_ref();
// }

// dangling reference
// fn create_string_ref() -> &String {
//     let s = String::from("hello");
//     return &s;
// }

//Array
// fn main() {
//     let mut arr1:[&str;3] = ["hillo", "hello", "world"];
//     write_arr(&mut arr1); //arry directly pass
//     println!("arr1 = {:?}", arr1);
// }

// fn write_arr(mut arr2:[&str;3]){//arr2 new copy     
//     arr2[0] = "world";
//     println!("arr2 = {:?}", arr2);
// }

// passing refernce of array

// fn write_arr(arr2:&mut [&str;3]){
//     arr2[0] = "world";
//     println!("arr2 = {:?}", arr2);

// }

//vector

// fn main(){
//     let mut v:Vec<i32> = Vec::new();//declration

//     v.push(1);
//     v.push(2);
//     v.push(3);
//     println!("v = {:?}", v);
// }

// fn main(){
//     let vrr: Vec<&str> = vec!["hello", "world", "hii"];
//     write_vrr(vrr);//vrr ownership transfer
//     // println!("vrr={:?}", vrr);
// }

// fn write_vrr(vrr2: Vec<&str>){ // vrr2 is the new owner of vrr
//     println!("vrr2 = {:?}", vrr2);
// }

//loops

// fn main() {
//     let arr = [1,2,3];

//     println!("arr[0] = {}", arr[0]);
//     println!("arr[1] = {}", arr[1]);
//     println!("arr[2] = {}", arr[2]);

//     // for element in &arr{
//     //     print!("{} ", element);

//     // }
// }

//match

fn match(){
    
}