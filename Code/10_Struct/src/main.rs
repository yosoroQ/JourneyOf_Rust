fn main() {
    println!("Hello, world!");
}

//存储用户账号信息的结构体
struct User{
    Username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//示例：创建 `User` 结构体的实例
let user1 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

//示例：改变一个可变的 `User` 实例 `email` 字段的值
let mut user1 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("XXX666@outlook.com");


//示例：`build_user` 函数获取 email 和用户名并返回 `User` 实例
fn build_user(email: String,username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//变量与字段同名时的字段初始化简写语法
fn build_user(email: String,username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//使用结构体更新语法从其他实例创建实例
//创建 User 新实例，其使用了一些来自 user1 的值
let user2 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

//使用结构体更新语法为一个 User 实例设置新的 email 和 username 值，不过其余值来自 user1 变量中实例的字段
let user2 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
 
    //.. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    ..user1
};

//使用没有命名字段的元组结构体来创建不同的类型
//Color 和 Point 元组结构体的定义和用法：
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

//结构体数据的所有权
//如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

//一个使用结构体的示例程序
//示例：通过分别指定长方形的宽和高的变量来计算长方形面积
fn main() {
    let width1 = 30;
    let height1 = 50;

//长方形面积是1500 square pixels.
    println!(
        "长方形面积是{} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//## 使用元组重构
//### 示例：使用元组来指定长方形的宽高
fn main() {
    let rect1 = (30,50);

//长方形面积是1500 square pixels.
    println!(
        "长方形面积是{} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//使用结构体重构：赋予更多意义
//将我们正在使用的元组转换成一个有整体名称而且每个部分也有对应名字的数据类型
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

//长方形面积是1500 square pixels.
    println!(
        "长方形面积是{} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//通过派生 trait 增加实用功能
//尝试打印出 `Rectangle` 实例？
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}


//在结构体定义之前加上 `#[derive(Debug)]` 注解
//增加注解来派生 `Debug` trait，并使用调试格式打印 `Rectangle` 实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}

//方法语法
//定义方法
//在 `Rectangle` 结构体上定义 `area` 方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}

//带有更多参数的方法
//使用还未实现的 can_hold 方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

//Can rect1 hold rect2? true
// Can rect1 hold rect3? false
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


//关联函数

#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
}

//使用多个 `impl` 块重写

#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

