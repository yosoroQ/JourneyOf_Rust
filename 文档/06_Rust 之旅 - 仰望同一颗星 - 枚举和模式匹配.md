# Rust 之旅 - 仰望同一颗星 - 枚举和模式匹配

- **枚举**（*enumerations*），也被称作 *enums*。
- 枚举允许你通过列举可能的 **成员**（*variants*） 来定义一个类型。
- 首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。
- 接下来，我们会探索一个特别有用的枚举，叫做 `Option`，它代表一个值要么是某个值要么什么都不是。
- 然后会讲到在 `match` 表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。
- 最后会介绍 `if let`，另一个简洁方便处理代码中枚举的结构。
- 枚举是一个很多语言都有的功能，不过不同语言中其功能各不相同。
- Rust 的枚举与 F#、OCaml 和 Haskell 这样的函数式编程语言中的 **代数数据类型**（*algebraic data types*）最为相似。

# 定义枚举

* 假设我们要处理 IP 地址。
* 目前被广泛使用的两个主要 IP 标准：IPv4（version four）和 IPv6（version six），这是我们的程序可能会遇到的所有可能的 IP 地址类型
* 所以可以 **枚举** 出所有可能的值，这也正是此枚举名字的由来。
* 任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是。
* IP 地址的这个特性使得枚举数据结构非常适合这个场景，因为枚举值只可能是其中一个成员。
* IPv4 和 IPv6 从根本上讲仍是 IP 地址，所以当代码在处理适用于任何类型的 IP 地址的场景时应该把它们当作相同的类型。
* 可以通过在代码中定义一个 `IpAddrKind` 枚举来表现这个概念并列出可能的 IP 地址类型，`V4` 和 `V6`。
* 这被称为枚举的 **成员**（*variants*）：

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

* 现在 `IpAddrKind` 就是一个可以在代码中使用的自定义数据类型了。

## 枚举值

* 可以像这样创建 `IpAddrKind` 两个不同成员的实例：

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

* 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。
* 这么设计的益处是现在 `IpAddrKind::V4` 和 `IpAddrKind::V6` 都是 `IpAddrKind` 类型的。
* 例如，接着可以定义一个函数来获取任何 `IpAddrKind`：

```rust
fn route(ip_type: IpAddrKind) { }
```

* 现在可以使用任一成员来调用这个函数：

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

* 使用枚举甚至还有更多优势。
* 进一步考虑一下我们的 IP 地址类型，目前没有一个存储实际 IP 地址 **数据** 的方法；只知道它是什么 **类型** 的。

### 示例：将 IP 地址的数据和 `IpAddrKind` 成员存储在一个 `struct` 中

```rust
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
```

* 这里我们定义了一个有两个字段的结构体 `IpAddr`：`IpAddrKind`（之前定义的枚举）类型的 `kind` 字段和 `String` 类型 `address` 字段。
* 我们有这个结构体的两个实例。
* 第一个，`home`，它的 `kind` 的值是 `IpAddrKind::V4` 与之相关联的地址数据是 `127.0.0.1`。
* 第二个实例，`loopback`，`kind` 的值是 `IpAddrKind` 的另一个成员，`V6`，关联的地址是 `::1`。
* 我们使用了一个结构体来将 `kind` 和 `address` 打包在一起，现在枚举成员就与值相关联了。

### 改进：仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。

* `IpAddr` 枚举的新定义表明了 `V4` 和 `V6` 成员都关联了 `String` 值：

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

* 我们直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
* 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。
* IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分。
* 如果我们想要将 `V4` 地址存储为四个 `u8` 值而 `V6` 地址仍然表现为一个 `String`，这就不能使用结构体了。
* 枚举则可以轻易处理的这个情况：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

* 这些代码展示了使用枚举来存储两种不同 IP 地址的几种可能的选择。
* 然而，事实证明存储和编码 IP 地址实在是太常见了[以致标准库提供了一个开箱即用的定义！](https://doc.rust-lang.org/std/net/enum.IpAddr.html)
* 让我们看看标准库是如何定义 `IpAddr` 的：
* 它正有着跟我们定义和使用的一样的枚举和成员，不过它将成员中的地址数据嵌入到了两个不同形式的结构体中，它们对不同的成员的定义是不同的：

```rust
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
```

* 这些代码展示了可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。
* 甚至可以包含另一个枚举！另外，标准库中的类型通常并不比你设想出来的要复杂多少。
* 注意虽然标准库中包含一个 `IpAddr` 的定义，仍然可以创建和使用我们自己的定义而不会有冲突，因为我们并没有将标准库中的定义引入作用域。

### 枚举 - 示例 - 它的成员中内嵌了多种多样的类型

* 一个 `Message` 枚举，其每个成员都存储了不同数量和类型的值。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

* 这个枚举有四个含有不同类型的成员：

    - `Quit` 没有关联任何数据。
    - `Move` 包含一个匿名结构体。
    - `Write` 包含单独一个 `String`。
    - `ChangeColor` 包含三个 `i32`。

* 定义一个如示例 6-2 中所示那样的有关联值的枚举的方式和定义多个不同类型的结构体的方式很相像，除了枚举不使用 `struct` 关键字以及其所有成员都被组合在一起位于 `Message` 类型下。

### 如下这些结构体可以包含与之前枚举成员中相同的数据

```rust
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```

* 不过，如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像使用（<u>**枚举 - 示例 - 它的成员中内嵌了多种多样的类型**</u>）中定义的 `Message` 枚举那样，轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型。
* 结构体和枚举还有另一个相似点：就像可以使用 `impl` 来为结构体定义方法那样，也可以在枚举上定义方法。

### 一个定义于我们 `Message` 枚举上的叫做 `call` 的方法

* 方法体使用了 `self` 来获取调用方法的值。
* 这个例子中，创建了一个值为 `Message::Write(String::from("hello"))` 的变量 `m`，而且这就是当 `m.call()` 运行时 `call` 方法中的 `self` 的值。

```rust
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## Option 枚举和其相对于空值的优势

* 在之前的部分，我们看到了 `IpAddr` 枚举如何利用 Rust 的类型系统在程序中编码更多信息而不单单是数据。
* 接下来我们分析一个 `Option` 的案例，`Option` 是标准库定义的另一个枚举。
* `Option` 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。
* 从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的 bug。
* 编程语言的设计经常要考虑包含哪些功能，但考虑排除哪些功能也很重要。
* Rust 并没有很多其他语言中有的空值功能。
* **空值**（*Null* ）是一个值，它代表没有值。
* 在有空值的语言中，变量总是这两种状态之一：空值和非空值。
* 空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。
* 因为空和非空的属性无处不在，非常容易出现这类错误。
* 然而，空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。
* 问题不在于概念而在于具体的实现。
* 为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。
* 这个枚举是 `Option<T>`，而且它[定义于标准库中](https://doc.rust-lang.org/std/option/enum.Option.html)，如下:

### 枚举 `Option<T>`

* `Option<T>` 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。
* 另外，它的成员也是如此，可以不需要 `Option::` 前缀来直接使用 `Some` 和 `None`。
* 即便如此 `Option<T>` 也仍是常规的枚举，`Some(T)` 和 `None` 仍是 `Option<T>` 的成员。
* `<T>` 语法是一个我们还未讲到的 Rust 功能。
* 它是一个泛型类型参数。
* 目前，需要知道的就是 `<T>` 意味着 `Option` 枚举的 `Some` 成员可以包含任意类型的数据。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### 包含数字类型和字符串类型 `Option` 值的例子

* 如果使用 `None` 而不是 `Some`，需要告诉 Rust `Option<T>` 是什么类型的，因为编译器只通过 `None` 值无法推断出 `Some` 成员保存的值的类型。

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

* 当有一个 `Some` 值时，我们就知道存在一个值，而这个值保存在 `Some` 中。
* 当有个 `None` 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。

### `Option<T>` 为什么就比空值要好呢？

* 因为 `Option<T>` 和 `T`（这里 `T` 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 `Option<T>`。
* 例如，这段代码不能编译，因为它尝试将 `Option<i8>` 与 `i8` 相加：

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

如果运行这些代码，将得到类似这样的错误信息：

```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

* 事实上，错误信息意味着 Rust 不知道该如何将 `Option<i8>` 与 `i8` 相加，因为它们的类型不同。

* 当在 Rust 中拥有一个像 `i8` 这样类型的值时，编译器确保它总是有一个有效的值。

* 我们可以自信使用而无需做空值检查。

* 只有当使用 `Option<i8>`（或者任何用到的类型）的时候需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。

* 换句话说，在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`。

* 通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。

* 不再担心会错误的假设一个非空值，会让你对代码更加有信心。

* 为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 `Option<T>` 中。

* 接着，当使用这个值时，必须明确的处理值为空的情况。

* 只要一个值不是 `Option<T>` 类型，你就 **可以** 安全的认定它的值不为空。

* 这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。

### 当有一个 `Option<T>` 的值时，如何从 `Some` 成员中取出 `T` 的值来使用它呢？

*  `Option<T>` 枚举拥有大量用于各种情况的方法：你可以查看[它的文档](https://doc.rust-lang.org/std/option/enum.Option.html)。
* 熟悉 `Option<T>` 的方法将对你的 Rust 之旅非常有用。
* 总的来说，为了使用 `Option<T>` 值，需要编写处理每个成员的代码。
* 你想要一些代码只当拥有 `Some(T)` 值时运行，允许这些代码使用其中的 `T`。
* 也希望一些代码在值为 `None` 时运行，这些代码并没有一个可用的 `T` 值。
* `match` 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。

# match控制流运算符

* Rust 有一个叫做 `match` 的极为强大的控制流运算符，它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。
* 模式可由字面值、变量、通配符和许多其他内容构成。
* `match`的力量来源于模式的表现力以及编译器检查，它确保了所有可能的情况都得到处理。
* 可以把 `match` 表达式想象成某种硬币分类器：硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。
* 同样地，值也会通过 `match` 的每一个模式，并且在遇到第一个 “符合” 的模式时，值会进入相关联的代码块并在执行中被使用。

## 使用 `match` 的例子 —— 一个枚举和一个以枚举成员作为模式的 `match` 表达式

```rust
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
```

* 拆开 `value_in_cents` 函数中的 `match` 来看。
* 首先，我们列出 `match` 关键字后跟一个表达式，在这个例子中是 `coin` 的值。
* 这看起来非常像 `if` 使用的表达式，不过这里有一个非常大的区别：对于 `if`，表达式必须返回一个布尔值，而这里它可以是任何类型的。
* 接下来是 `match` 的分支。
* 一个分支有两个部分：一个模式和一些代码。
* 第一个分支的模式是值 `Coin::Penny` 而之后的 `=>` 运算符将模式和将要运行的代码分开。
* 这里的代码就仅仅是值 `1`，每一个分支之间使用逗号分隔。
* 当 `match` 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。
* 如果模式匹配了这个值，这个模式相关联的代码将被执行。
* 如果模式并不匹配这个值，将继续执行下一个分支，非常类似一个硬币分类器。
* 可以拥有任意多的分支：示例中的 `match` 有四个分支。
* 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 `match` 表达式的返回值。
* 如果分支代码较短的话通常不使用大括号，正如上例的每个分支都只是返回一个值。

## 想要在分支中运行多行代码，可以使用大括号

* 例如，如下代码在每次使用`Coin::Penny` 调用时都会打印出 “Lucky penny!”，同时仍然返回代码块最后的值，`1`：

```rust
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
```

## 绑定值的模式

* 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。
* 这也就是如何从枚举成员中提取值的。

* 作为一个例子，让我们修改枚举的一个成员来存放数据。1999 年到 2008 年间，美帝在 25 美分的硬币的一侧为 50 个州的每一个都印刷了不同的设计。其他的硬币都没有这种区分州的设计，所以只有这些 25 美分硬币有特殊的价值。

### 例子：修改枚举的一个成员来存放数据 - `Quarter` 成员也存放了一个 `UsState` 值的 `Coin` 枚举
* 可以将这些信息加入我们的 `enum`，通过改变 `Quarter` 成员来包含一个 `State` 值。

```rust
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
```

* 想象一下我们的一个朋友尝试收集所有 50 个州的 25 美分硬币。
* 在根据硬币类型分类零钱的同时，也可以报告出每个 25 美分硬币所对应的州名称，这样如果我们的朋友没有的话，他可以将其加入收藏。
* 在这些代码的匹配表达式中，我们在匹配 `Coin::Quarter` 成员的分支的模式中增加了一个叫做 `state` 的变量。
* 当匹配到 `Coin::Quarter` 时，变量 `state` 将会绑定 25 美分硬币所对应州的值。

### 在那个分支的代码中使用 `state`

* 如果调用 `value_in_cents(Coin::Quarter(UsState::Alaska))`，`coin` 将是 `Coin::Quarter(UsState::Alaska)`。
* 当将值与每个分支相比较时，没有分支会匹配，直到遇到 `Coin::Quarter(state)`。
* 这时，`state` 绑定的将会是值 `UsState::Alaska`。
* 接着就可以在 `println!` 表达式中使用这个绑定了，像这样就可以获取 `Coin` 枚举的 `Quarter` 成员中内部的州的值。

```rust
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
```

## 匹配 Option<T>

* 我们在之前的部分中使用 `Option<T>` 时，是为了从 `Some` 中取出其内部的 `T` 值。
* 我们还可以像处理 `Coin` 枚举那样使用 `match` 处理 `Option<T>`！
* 与其直接比较硬币，我们将比较 `Option<T>` 的成员，不过 `match` 表达式的工作方式保持不变。
* 比如我们想要编写一个函数，它获取一个 `Option<i32>` 并且如果其中有一个值，将其加一。
* 如果其中没有值，函数应该返回 `None` 值并不尝试执行任何操作。
* 得益于 `match`，编写这个函数非常简单。

### 示例：一个在 `Option<i32>` 上使用 `match` 表达式的函数

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## 匹配 Some(T)

* 让我们更仔细地检查 `plus_one` 的第一行操作。
* 当调用 `plus_one(five)` 时，`plus_one` 函数体中的 `x` 将会是值 `Some(5)`。
* 接着将其与每个分支比较。

```rust
None => None,
```

* 值 `Some(5)` 并不匹配模式 `None`，所以继续进行下一个分支。

```rust
Some(i) => Some(i + 1),
```

* `Some(5)` 与 `Some(i)` 匹配吗？当然匹配！它们是相同的成员。
* `i` 绑定了 `Some` 中包含的值，所以 `i` 的值是 `5`。
* 接着匹配分支的代码被执行，所以我们将 `i` 的值加一并返回一个含有值 `6` 的新 `Some`。
* 接着考虑下上例中 `plus_one` 的第二个调用，这里 `x` 是 `None`。
* 我们进入 `match` 并与第一个分支相比较。

```rust
None => None,
```

* 匹配上了！这里没有值来加一，所以程序结束并返回 `=>` 右侧的值 `None`，因为第一个分支就匹配到了，其他的分支将不再比较。
* 将 `match` 与枚举相结合在很多场景中都是有用的。
* 你会在 Rust 代码中看到很多这样的模式：**`match` 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。**

## 匹配是穷尽的

* `match` 还有另一方面需要讨论。考虑一下 `plus_one` 函数的这个版本，它有一个 bug 并不能编译：

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

* 我们没有处理 `None` 的情况，所以这些代码会造成一个 bug。
* 幸运的是，这是一个 Rust 知道如何处理的 bug。如果尝试编译这段代码，会得到这个错误：
* Rust 知道我们没有覆盖所有可能的情况甚至知道哪些模式被忘记了！
* **Rust 中的匹配是 穷尽的（*exhaustive*）**：必须穷举到最后的可能性来使代码有效。
* 特别的在这个 `Option<T>` 的例子中，Rust 防止我们忘记明确的处理 `None` 的情况，这使我们免于假设拥有一个实际上为空的值，这造成了之前提到过的价值亿万的错误。

```text
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

## _ 通配符

* Rust 也提供了一个模式用于不想列举出所有可能值的场景。
* 例如，`u8` 可以拥有 0 到 255 的有效的值，如果我们只关心 1、3、5 和 7 这几个值，就并不想必须列出 0、2、4、6、8、9 一直到 255 的值。
* 所幸我们不必这么做：可以使用特殊的模式 `_` 替代：

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

* `_` 模式会匹配所有的值。
* 通过将其放置于其他分支之后，`_` 将会匹配所有之前没有指定的可能的值。
* `()` 就是 unit 值，所以 `_` 的情况什么也不会发生。
* 因此，可以说我们想要对 `_` 通配符之前没有列出的所有可能的值不做任何处理。
* 然而，`match` 在只关心 **一个** 情况的场景中可能就有点啰嗦了。
* 为此 Rust 提供了`if let`。

# if let简洁控制流

* `if let` 语法让我们以一种不那么冗长的方式结合 `if` 和 `let`，来处理只匹配一个模式的值而忽略其他模式的情况。

## `match` 只关心当值为 `Some(3)` 时执行代码

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

* 想要对 `Some(3)` 匹配进行操作但是不想处理任何其他 `Some<u8>` 值或 `None` 值。
* 为了满足 `match` 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 `_ => ()`，这样也要增加很多样板代码。
* 不过我们可以使用 `if let` 这种更短的方式编写。
* 如下代码与上例中的 `match` 行为一致：

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

* `if let` 获取通过等号分隔的一个模式和一个表达式。
* 它的工作方式与 `match` 相同，这里的表达式对应 `match` 而模式则对应第一个分支。
* 使用 `if let` 意味着编写更少代码，更少的缩进和更少的样板代码。
* 然而，这样会失去 `match` 强制要求的穷尽性检查。
* `match` 和 `if let` 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
* 换句话说，可以认为 `if let` 是 `match` 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
* 可以在 `if let` 中包含一个 `else`。
* `else` 块中的代码与 `match` 表达式中的 `_` 分支块中的代码相同，这样的 `match` 表达式就等同于 `if let` 和 `else`。

## 如果想要计数所有不是 25 美分的硬币的同时也报告 25 美分硬币所属的州，可以使用这样一个 `match` 表达式

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

### 或者可以使用这样的 `if let` 和 `else` 表达式

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

* 如果你的程序遇到一个使用 `match` 表达起来过于啰嗦的逻辑，记住 `if let` 也在你的 Rust 工具箱中。

# [总结](https://rust.bootcss.com/ch06-03-if-let.html#总结)

* 现在我们涉及到了如何使用枚举来创建有一系列可列举值的自定义类型。
* 我们也展示了标准库的 `Option<T>` 类型是如何帮助你利用类型系统来避免出错的。
* 当枚举值包含数据时，你可以根据需要处理多少情况来选择使用 `match` 或 `if let` 来获取并使用这些值。
* 你的 Rust 程序现在能够使用结构体和枚举在自己的作用域内表现其内容了。
* 在你的 API 中使用自定义类型保证了类型安全：编译器会确保你的函数只会得到它期望的类型的值。
* 为了向你的用户提供一个组织良好的 API，它使用起来很直观并且只向用户暴露他们确实需要的部分，那么现在就让我们转向 Rust 的模块系统吧。