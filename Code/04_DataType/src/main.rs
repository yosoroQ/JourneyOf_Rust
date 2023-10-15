//展示浮点数的实例
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// //数值运算
// fn main() {
//     // 加法
//     let sum = 5 + 10;
//     println!("The value of sum is: {}", sum);

//     // 减法
//     let difference = 95.5 - 4.3;
//     println!("The value of difference is: {}", difference);

//     // 乘法
//     let product = 4 * 30;
//     println!("The value of product is: {}", product);

//     // 除法
//     let quotient = 56.7 / 32.2;
//     println!("The value of quotient is: {}", quotient);

//     // 取余
//     let remainder = 43 % 5;
//     println!("The value of remainder is: {}", remainder);

// }

// //布尔型
// fn main() {
//     let t = true;

//     let f: bool = false; // 显式指定类型注解
// }

// //字符类型
// fn main() {
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
// }

// //复合类型 - 元组类型 - 可选的类型注解
// fn main(){
//     let tup:(i32,f64,u8) = (500,6.4,1);
// }

// //为了从元组中获取单个值
// //，可以使用模式匹配（pattern matching）来解构（destructure）元组值。
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}", y);
// }

// //使用点号（.）后跟值的索引来直接访问
// fm main(){
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let fh = x.0;
//     let spf = x.1;
//     let one = x.2;
// }

// //数组类型
// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// //####想要使用数组而不是 vector 的例子
// // * 当程序需要知道一年中月份的名字时，程序不大可能会去增加或减少月份。
// // * 这时你可以使用数组，因为我们知道它总是包含 12 个元素
// fn main(){
//     let months = ["January", "February", "March", "April",
//      "May", "June", "July","August",
//       "September", "October", "November", "December"];
// }

// //#### 另一种写法
// // * 在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
// fn main(){
//     let a: [i32;5] = [1, 2, 3, 4, 5];
// }

// //访问数组元素
// // 数组是一整块分配在栈上的内存。可以使用索引来访问数组的元素
// // 叫做 first 的变量的值是 1，因为它是数组索引 [0] 的值。
// // 变量 second 将会是数组索引 [1] 的值 2。
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }

//无效的数组元素访问
//如果我们访问数组结尾之后的元素会发生什么呢？
//比如你将上面的例子改成下面这样，这可以编译不过在运行时会因错误而退出：
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
