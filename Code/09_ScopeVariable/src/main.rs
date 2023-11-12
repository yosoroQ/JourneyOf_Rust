fn main() {                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的

    // 使用 s
}                      // 此作用域已结束，s 不再有效

//**可以使用 `from` 函数基于字符串字面值来创建 `String`**。
// fn main(){
//     //原
//     let s = String::from("hello");

//     //可以 修改此类字符串
//     let mut s = String::from("hello");
//     s.push_str(",world");// push_str() 在字符串后追加字面值
//     println!("{}", s); // 将打印 `hello, world!`
// }

// // //作用域例子的一个使用 `String` 而不是字符串字面值的版本
// fn main(){
//     let s = String::from("hello"); // 从此处起，s 是有效的

//     // 使用 s
// }                                  // 此作用域已结束，
//                           // s 不再有效

// //变量与数据交互的方式（一）：移动
// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1;
    
//     println!("{}, world!", s1);
// }

// //变量与数据交互的方式（二）：克隆
// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
    
//     println!("{}, world!", s1);
// }

// //带有所有权和作用域注释的函数
// fn main(){
//     let s1 = String::from("hello");  // s1 进入作用域
//     takes_ownership(s1);  // s1 的值移动到函数里 ...
//                              // ... 所以到这里不再有效
//     let x = 5;             // x 进入作用域
//     makes_copy(x);           // x 应该移动函数里，
//     // 但 i32 是 Copy 的，所以在后面可继续使用 x
    
// }        // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String){ // some_string 进入作用域

//     //hello

//     println!("{}", some_string); // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
// }

// fn makes_copy(some_integer: i32){ // some_integer 进入作用域

//     //5
//     println!("{}", some_integer); // 这里，some_integer 移出作用域。不会有特殊操作
// }



// //返回值也可以转移所有权（转移返回值的所有权）
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership 将返回值
//                                         // 移给 s1

//     let s2 = String::from("hello");     // s2 进入作用域

//     let s3 = takes_and_gives_back(s2);  // s2 被移动到
//                                         // takes_and_gives_back 中, 
//                                         // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 移出作用域并被丢弃

// fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
//                                              // 调用它的函数

//     let some_string = String::from("hello"); // some_string 进入作用域.

//     some_string                              // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

//     a_string  // 返回 a_string 并移出给调用的函数
// }


// //无注释版
// fn main(){
//     let s1 = gives_ownership();
//     let s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String{
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string:String) -> String{
//     a_string
// }

// //使用元组来返回多个值（返回参数的所有权）
// fn main(){
//     let s1 = String::from("hello");
//     let (s2,len) = calculate_length(s1);

//     //The length of 'hello' is 5.
//     println!("The length of '{}' is {}.",s2,len);
// }

// fn calculate_length(s:String) -> (String,usize){
//     let length = s.len();
//     (s,length)
// }

// //以一个对象的引用作为参数而不是获取值的所有权
// fn main(){
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);

//     //The length of 'hello' is 5.
//     println!("The length of '{}' is {}.", s1, len);
// }
// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

// //尝试修改借用的变量（Wrong!）？
// fn main(){
//     let s1 = String::from("hello");
//     change(&s1);
// }
// fn change(some_string: &String){
//     some_string.push_str(", world");
// }

// // //可变引用
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// //尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免
// fn main(){
//     // let reference_to_nothing = dangle();
//     let reference_to_nothing = no_dangle();

// }

// // fn dangle() -> &String{
// //     let s =String::from("hello");

// //     &s
// // }

// // //直接返回 String
// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }


// //**字符串 slice**（*string slice*）是 `String` 中一部分值的引用
// fn main(){
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
// }

// //对于 Rust 的 `..` range 语法，如果想要从第一个索引（0）开始，可以不写两个点号之前的值。
// fn main(){
//     let s = String::from("hello");

//     let slice = &s[0..2];
//     let slice = &s[..2];
// }

// //如果 slice 包含 `String` 的最后一个字节，也可以舍弃尾部的数字
// fn main(){
//     let s = String::from("hello");

//     let len = s.len();
    
//     let slice = &s[3..len];
//     let slice = &s[3..];
// }

// //也可以同时舍弃这两个值来获取整个字符串的 slice
// fn main(){
//     let s = String::from("hello");

//     let len = s.len();

//     let slice = &s[0..len];
//     let slice = &s[..];
// }

// //重写 `first_word` 来返回一个 slice，“字符串 slice” 的类型声明写作 `&str`
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// //通过将 `s` 参数的类型改为字符串 slice 来改进 `first_word` 函数
// fn main() {
//     let my_string = String::from("hello world");

//     // first_word 中传入 `String` 的 slice
//     let word = first_word(&my_string[..]);

//     let my_string_literal = "hello world";

//     // first_word 中传入字符串字面值的 slice
//     let word = first_word(&my_string_literal[..]);

//     // 因为字符串字面值 **就是** 字符串 slice，
//     // 这样写也可以，即不使用 slice 语法！
//     let word = first_word(my_string_literal);
// }

// //其他类型的 slice
// fn main(){
//     let a = [1, 2, 3, 4, 5];

//     let slice = &a[1..3];
// }
