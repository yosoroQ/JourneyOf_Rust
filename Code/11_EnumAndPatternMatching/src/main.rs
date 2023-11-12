fn main() {
    println!("Hello, world!");
}


//可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的 IP 地址类型，V4 和 V6。这被称为枚举的 成员（variants）：
//现在 IpAddrKind 就是一个可以在代码中使用的自定义数据类型了。
// enum IpAddrKind {
//     V4,
//     V6,
// }

//示例 6-1：将 IP 地址的数据和 `IpAddrKind` 成员存储在一个 `struct` 中
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

// 改进：仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

//* 如果我们想要将 `V4` 地址存储为四个 `u8` 值而 `V6` 地址仍然表现为一个 `String`，这就不能使用结构体了。
// * 枚举则可以轻易处理的这个情况：
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

//标准库是如何定义 `IpAddr` 的：
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//示例 6-2 中的另一个枚举的例子：它的成员中内嵌了多种多样的类型'
// 一个 `Message` 枚举，其每个成员都存储了不同数量和类型的值
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//如下这些结构体可以包含与之前枚举成员中相同的数据
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

//一个定义于我们 `Message` 枚举上的叫做 `call` 的方法
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();

//枚举 Option<T> (定义于标准库中)
enum Option<T> {
    Some(T),
    None,
}

//包含数字类型和字符串类型 `Option` 值的例子
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

//`Option<T>` 为什么就比空值要好呢？
//因为 `Option<T>` 和 `T`（这里 `T` 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 `Option<T>`。
//例如，这段代码不能编译，因为它尝试将 `Option<i8>` 与 `i8` 相加：
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;

//match 控制流运算符
// 使用 `match` 的例子 —— 一个枚举和一个以枚举成员作为模式的 `match` 表达式
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//想要在分支中运行多行代码，可以使用大括号
//例如，如下代码在每次使用`Coin::Penny` 调用时都会打印出 “Lucky penny!”，同时仍然返回代码块最后的值，`1`：
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//### 例子：修改枚举的一个成员来存放数据
//`Quarter` 成员也存放了一个 `UsState` 值的 `Coin` 枚举
// * 可以将这些信息加入我们的 `enum`，通过改变 `Quarter` 成员来包含一个 `State` 值。
#[derive(Debug)] // 这样可以可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//在那个分支的代码中使用 `state`
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

//示例：一个在 `Option<i32>` 上使用 `match` 表达式的函数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

//if let 简单控制流
// `match` 只关心当值为 `Some(3)` 时执行代码
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

//如果想要计数所有不是 25 美分的硬币的同时也报告 25 美分硬币所属的州，可以使用这样一个 `match` 表达式
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

//或者可以使用这样的 `if let` 和 `else` 表达式
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}