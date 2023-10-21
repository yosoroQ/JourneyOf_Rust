# Rust 之旅 - E大调在全世界引起共鸣 - 使用结构体组织相关联的数据

- *struct*，或者 *structure*，是一个自定义数据类型，允许你命名和包装多个相关的值，从而形成一个有意义的组合。
- 如果你熟悉一门面向对象语言，*struct* 就像对象中的数据属性。
- 涉及概念：**对比元组与结构体的异同**、**演示结构体的用法**、**讨论如何在结构体上定义方法和关联函数来指定与结构体数据相关的行为**。
- 你可以在程序中基于结构体和枚举（*enum*）创建新类型，以充分利用 Rust 的编译时类型检查。

# 定义并实例化结构体

* 和元组一样，结构体的每一部分可以是不同类型。
* 但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。
* 由于有了这些名字，结构体比元组更灵活：**不需要依赖顺序来指定或访问实例中的值**。
* 定义结构体，需要使用 `struct` 关键字并为整个结构体提供一个名字。
* 结构体的名字需要描述它所组合的数据的意义。
* 接着，在大括号中，定义每一部分数据的名字和类型，我们称为 **字段**（*field*）。

## 示例：存储用户账号信息的结构体

* 一旦定义了结构体后，为了使用它，通过为每个字段指定具体值来创建这个结构体的 **实例**。
* 创建一个实例需要以结构体的名字开头，接着在大括号中使用 `key: value` 键-值对的形式提供字段，其中 key 是字段的名字，value 是需要存储在字段中的数据值。
* 实例中字段的顺序不需要和它们在结构体中声明的顺序一致。
* 换句话说，结构体的定义就像一个类型的通用模板，而实例则会在这个模板中放入特定数据来创建这个类型的值。

```rust
//存储用户账号信息的结构体
struct User{
    Username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

## 示例：创建 `User` 结构体的实例

* 为了从结构体中获取某个特定的值，可以使用点号。
* 如果我们只想要用户的邮箱地址，可以用 `user1.email`。
* 要更改结构体中的值，如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。

```rust
//示例：创建 `User` 结构体的实例
let user1 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
```

## 示例：改变一个可变的 `User` 实例 `email` 字段的值

* 注意整个实例必须是可变的；
* Rust 并不允许只将某个字段标记为可变。
* 另外需要注意同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。

```rust
//示例：改变一个可变的 `User` 实例 `email` 字段的值
let mut user1 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("XXX666@outlook.com");
```

## 示例：`build_user` 函数获取 email 和用户名并返回 `User` 实例

* 显示了一个 `build_user` 函数，它返回一个带有给定的 email 和用户名的 `User` 结构体实例。
* `active` 字段的值为 `true`，并且 `sign_in_count` 的值为 `1`。

```rust
//示例：`build_user` 函数获取 email 和用户名并返回 `User` 实例
fn build_user(email: String,username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

* 为函数参数起与结构体字段相同的名字是可以理解的，但是不得不重复 `email` 和 `username` 字段名称与变量有些啰嗦。
* 如果结构体有更多字段，重复每个名称就更加烦人了。幸运的是，有一个方便的简写语法！

## 变量与字段同名时的字段初始化简写语法

* 如果参数名与字段名都完全相同，我们可以使用 **字段初始化简写语法**（*field init shorthand*）来重写 `build_user`。
* 这样其行为与之前完全相同，不过无需重复 `email` 和 `username` 了。

### 改写`build_user` 函数

* 这里我们创建了一个新的 `User` 结构体实例，它有一个叫做 `email` 的字段。
* 我们想要将 `email` 字段的值设置为 `build_user` 函数 `email` 参数的值。
* 因为 `email` 字段与 `email` 参数有着相同的名称，则只需编写 `email` 而不是 `email: email`。

```rust
//变量与字段同名时的字段初始化简写语法
fn build_user(email: String,username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 使用结构体更新语法从其他实例创建实例

* 使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常是很有帮助的。
* 这可以通过 **结构体更新语法**（*struct update syntax*）实现。

### 实例：创建 User 新实例，其使用了一些来自 user1 的值

```rust
//使用结构体更新语法从其他实例创建实例
//创建 User 新实例，其使用了一些来自 user1 的值
let user2 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

### 实例：使用结构体更新语法为一个 User 实例设置新的 email 和 username 值，不过其余值来自 user1 变量中实例的字段

* 使用结构体更新语法，我们可以通过更少的代码来达到相同的效果。
* `..` 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
* 在 `user2` 中创建了一个新实例，其有不同的 `email` 和 `username` 值不过 `active` 和 `sign_in_count` 字段的值与 `user1` 相同。

```rust
//使用结构体更新语法为一个 User 实例设置新的 email 和 username 值，不过其余值来自 user1 变量中实例的字段
let user2 = User {
    email: String::from("XXX@outlook.com"),
    username: String::from("someusername123"),
    
    //.. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    ..user1
};
```

## 使用没有命名字段的元组结构体来创建不同的类型

* 也可以定义与元组类似的结构体，称为 **元组结构体**（*tuple structs*）。
* 元组结构体有着结构体名称提供的含义，**但没有具体的字段名，只有字段的类型**。
* 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。
* 要定义元组结构体，以 `struct` 关键字和结构体名开头并后跟元组中的类型。

### 示例：`Color` 和 `Point` 元组结构体的定义和用法

* 注意 `black` 和 `origin` 值的类型不同，因为它们是不同的元组结构体的实例。
* 你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型。
* 例如，一个获取 `Color` 类型参数的函数不能接受 `Point` 作为参数，即便这两个类型都由三个 `i32` 值组成。
* 在其他方面，元组结构体实例类似于元组：可以将其解构为单独的部分，也可以使用 `.` 后跟索引来访问单独的值，等等。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## 没有任何字段的类单元结构体

* 我们也可以定义一个没有任何字段的结构体！
* 它们被称为 **类单元结构体**（*unit-like structs*），因为它们类似于 `()`，即 unit 类型。
* 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。

### 结构体数据的所有权

* 在**示例：存储用户账号信息的结构体**中的 `User` 结构体的定义中，我们使用了自身拥有所有权的 `String` 类型而不是 `&str` 字符串 slice 类型。
* 这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
* 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 **生命周期**（*lifetimes*）。
* 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。

#### 示例：如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的

* 编译器会抱怨它需要生命周期标识符

```rust
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
```

![image-20231021221656064](http://qny.expressisland.cn/dian/image-20231021221656064.png)

* 现在我们可以使用像 `String` 这类拥有所有权的类型来替代 `&str` 这样的引用以修正这个错误。

# 一个使用结构体的示例程序

* 为了理解何时会需要使用结构体，让我们编写一个计算长方形面积的程序。
* 我们会从单独的变量开始，接着重构程序直到使用结构体替代他们为止。

## 示例：通过分别指定长方形的宽和高的变量来计算长方形面积

```rust
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
```

* 虽然示例可以运行，并在调用 `area` 函数时传入每个维度来计算出长方形的面积，不过我们可以做的更好。
* 宽度和高度是相关联的，因为他们在一起才能定义一个长方形。
* 这些代码的问题突显在 `area` 的签名上：

```rust
fn area(width: u32, height: u32) -> u32 {
```

* 函数 `area` 本应该计算一个长方形的面积，不过函数却有两个参数。
* 这两个参数是相关联的，不过程序本身却没有表现出这一点。
* 将长度和宽度组合在一起将更易懂也更易处理。
* **使用元组重构**。

## 使用元组重构

### 示例：使用元组来指定长方形的宽高

* 在某种程度上说，这个程序更好一点了。
* 元组帮助我们增加了一些结构性，并且现在只需传一个参数。
* 不过在另一方面，这个版本却有一点不明确了：元组并没有给出元素的名称，所以计算变得更费解了，因为不得不使用索引来获取元组的每一部分。
* 在计算面积时将宽和高弄混倒无关紧要，不过当在屏幕上绘制长方形时就有问题了！
* 我们必须牢记 `width` 的元组索引是 `0`，`height` 的元组索引是 `1`。
* 如果其他人要使用这些代码，他们必须要搞清楚这一点，并也要牢记于心。
* 很容易忘记或者混淆这些值而造成错误，因为我们没有在代码中传达数据的意图。

```rust
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
```

## 使用结构体重构：赋予更多意义

* 我们使用结构体为数据命名来为其赋予意义。
* 我们可以将我们正在使用的元组转换成一个有整体名称而且每个部分也有对应名字的数据类型

### 示例：将我们正在使用的元组转换成一个有整体名称而且每个部分也有对应名字的数据类型

* 这里我们定义了一个结构体并称其为 `Rectangle`。
* 在大括号中定义了字段 `width` 和 `height`，类型都是 `u32`。
* 接着在 `main` 中，我们创建了一个具体的 `Rectangle` 实例，它的宽是 30，高是 50。
* 函数 `area` 现在被定义为接收一个名叫 `rectangle` 的参数，其类型是一个结构体 `Rectangle` 实例的不可变借用。
* 我们希望借用结构体而不是获取它的所有权，这样 `main` 函数就可以保持 `rect1` 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 `&`。
* `area` 函数访问 `Rectangle` 实例的 `width` 和 `height` 字段。
* `area` 的函数签名现在明确的阐述了我们的意图：使用 `Rectangle` 的 `width` 和 `height` 字段，计算 `Rectangle` 的面积。
* 这表明宽高是相互联系的，并为这些值提供了描述性的名称而不是使用元组的索引值 `0` 和 `1` 。
* 结构体胜在更清晰明了。

```rust
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
```

## 通过派生 trait 增加实用功能

* 如果能够在调试程序时打印出 `Rectangle` 实例来查看其所有字段的值就更好了。
* 示例像前面章节那样尝试使用 `println!` 宏，但这并不行。

### 尝试打印出 `Rectangle` 实例？

```rust
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
```

#### Error

* ![image-20231021223328912](http://qny.expressisland.cn/dian/image-20231021223328912.png)

* `println!` 宏能处理很多类型的格式。
* 不过，`{}` 默认告诉 `println!` 使用被称为 `Display` 的格式：意在提供给直接终端用户查看的输出。
* 目前为止见过的基本类型都默认实现了 `Display`，因为它就是向用户展示 `1` 或其他任何基本类型的唯一方式。
* 不过对于结构体，`println!` 应该用来输出的格式是不明确的。
* 因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？
* 由于这种不确定性，Rust 不会尝试猜测我们的意图，所以结构体并没有提供一个 `Display` 实现。

### 在结构体定义之前加上 `#[derive(Debug)]` 注解

* Rust **确实** 包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。

#### 示例：增加注解来派生 `Debug` trait，并使用调试格式打印 `Rectangle` 实例

```rust
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
```

![image-20231021223742254](http://qny.expressisland.cn/dian/image-20231021223742254.png)

* Rust 为我们提供了很多可以通过 `derive` 注解来使用的 trait，他们可以为我们的自定义类型增加实用的行为。
* 我们的 `area` 函数是非常特殊的，它只计算长方形的面积。
* 如果这个行为与 `Rectangle` 结构体再结合得更紧密一些就更好了，因为它不能用于其他类型。
* 现在让我们看看如何继续重构这些代码，来将 `area` 函数协调进 `Rectangle` 类型定义的 `area` **方法** 中。

# 方法语法

* **方法** 与函数类似：它们使用 `fn` 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
* 不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文，将分别在第六章和第十七章讲解），并且它们第一个参数总是 `self`，它代表调用该方法的结构体实例。

## 定义方法

* 让我们把前面实现的获取一个 `Rectangle` 实例作为参数的 `area` 函数，改写成一个定义于 `Rectangle` 结构体上的 `area` 方法。

### 在 `Rectangle` 结构体上定义 `area` 方法

* 为了使函数定义于 `Rectangle` 的上下文中，我们开始了一个 `impl` 块（`impl` 是 *implementation* 的缩写）。
* 接着将 `area` 函数移动到 `impl` 大括号中，并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数改成 `self`。
* 然后在 `main` 中将我们先前调用 `area` 方法并传递 `rect1` 作为参数的地方，改成使用 **方法语法**（*method syntax*）在 `Rectangle` 实例上调用 `area` 方法。
* 方法语法获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。
* 在 `area` 的签名中，使用 `&self` 来替代 `rectangle: &Rectangle`，因为该方法位于 `impl Rectangle` 上下文中所以 Rust 知道 `self` 的类型是 `Rectangle`。
* 注意仍然需要在 `self` 前面加上 `&`，就像 `&Rectangle` 一样。
* 方法可以选择获取 `self` 的所有权，或者像我们这里一样不可变地借用 `self`，或者可变地借用 `self`，就跟其他参数一样。
* 这里选择 `&self` 的理由跟在函数版本中使用 `&Rectangle` 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
* 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 `&mut self`。
* 通过仅仅使用 `self` 作为第一个参数来使方法获取实例的所有权是很少见的；
* 这种技术通常用在当方法将 `self` 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。
* 使用方法替代函数，除了可使用方法语法和不需要在每个函数签名中重复 `self` 的类型之外，其主要好处在于组织性。
* 我们将某个类型实例能做的所有事情都一起放入 `impl` 块中，而不是让将来的用户在我们的库中到处寻找 `Rectangle` 的功能。

```rust
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
```

### 运算符到哪去了？

* 在 C/C++ 语言中，有两个不同的运算符来调用方法：`.` 直接在对象上调用方法，而 `->` 在一个对象的指针上调用方法，这时需要先解引用（dereference）指针。
* 换句话说，如果 `object` 是一个指针，那么 `object->something()` 就像 `(*object).something()` 一样。
* Rust 并没有一个与 `->` 等效的运算符，相反，Rust 有一个叫 **自动引用和解引用**（*automatic referencing and dereferencing*）的功能。
* 方法调用是 Rust 中少数几个拥有这种行为的地方。
* 他是这样工作的：当使用 `object.something()` 调用方法时，Rust 会自动为 `object` 添加 `&`、`&mut` 或 `*` 以便使 `object` 与方法签名匹配。
* 也就是说，这些代码是等价的：

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

* 第一行看起来简洁的多。
* 这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— `self` 的类型。
* 在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（`&self`），做出修改（`&mut self`）或者是获取所有权（`self`）。
* 事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。

## 带有更多参数的方法

* 通过实现 `Rectangle` 结构体上的另一方法来练习使用方法。
* 让一个 `Rectangle` 的实例获取另一个 `Rectangle` 实例，如果 `self` 能完全包含第二个长方形则返回 `true`，否则返回 `false`。
* 一旦定义了 `can_hold` 方法，就可以编写代码。

### 使用还未实现的 `can_hold` 方法

```rust
//带有更多参数的方法
//使用还未实现的 can_hold 方法
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

* 因为我们想定义一个方法，所以它应该位于 `impl Rectangle` 块中。
* 方法名是 `can_hold`，并且它会获取另一个 `Rectangle` 的不可变借用作为参数。
* 通过观察调用方法的代码可以看出参数是什么类型的：`rect1.can_hold(&rect2)` 传入了 `&rect2`，它是一个 `Rectangle` 的实例 `rect2` 的不可变借用。
* 这是可以理解的，因为我们只需要读取 `rect2`（而不是写入，这意味着我们需要一个不可变借用），而且希望 `main` 保持 `rect2` 的所有权，这样就可以在调用这个方法后继续使用它。
* `can_hold` 的返回值是一个布尔值，其实现会分别检查 `self` 的宽高是否都大于另一个 `Rectangle`。
* 让我们在示例 5-13 的 ，如示例 5-15 所示：

### 在 `Rectangle` 上实现 `can_hold` 方法，它获取另一个 `Rectangle` 实例作为参数

* 在方法签名中，可以在 `self` 后增加多个参数，而且这些参数就像函数中的参数一样工作。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### 完整代码

```rust
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
```

## 关联函数

* `impl` 块的另一个有用的功能是：允许在 `impl` 块中定义 **不** 以 `self` 作为参数的函数。
* 这被称为 **关联函数**（*associated functions*），因为它们与结构体相关联。
* 它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例，你已经使用过 `String::from` 关联函数了。
* 关联函数经常被用作返回一个结构体新实例的构造函数。
* 例如我们可以提供一个关联函数，它接受一个维度参数并且同时作为宽和高，这样可以更轻松的创建一个正方形 `Rectangle` 而不必指定两次同样的值。

### 示例

* 使用结构体名和 `::` 语法来调用这个关联函数：比如 `let sq = Rectangle::square(3);`。
* 这个方法位于结构体的命名空间中：`::` 语法用于关联函数和模块创建的命名空间。

```rust
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
```

## 多个impl块

* 每个结构体都允许拥有多个 `impl` 块。

### 使用多个 `impl` 块重写

* 这里没有理由将这些方法分散在多个 `impl` 块中，不过这是有效的语法。

```rust
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
```

# 总结

* 结构体让你可以创建出在你的领域中有意义的自定义类型。
* 通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。
* 方法允许为结构体实例指定行为，而关联函数将特定功能置于结构体的命名空间中并且无需一个实例。
* 但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的**枚举功能**，为你的工具箱再添一个工具。



