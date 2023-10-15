# Rust 之旅 - Guessing Game

* 经典的新手编程问题：猜猜看游戏。它是这么工作的：程序将会随机生成一个 1 到 100 之间的随机整数。接着它会请玩家猜一个数并输入，然后提示猜测是大了还是小了。如果猜对了，它会打印祝贺信息并退出。
*  涉及概念：`let`、`match`、方法、关联函数、使用外部 `crate` 

# 完整代码

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("猜数字游戏");
    let secret_number = rand::thread_rng().gen_range(1,50);

    loop{

    println!("secret_number随机数为：{}",secret_number);
    println!("猜数字：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取错误");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

        println!("你输入的数字为：{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {println!("对了");
            break;
        }
    }
    }
}
```

# 创建项目

```rust
cargo new GuessNumber
cd GuessNumber
cargo run
```

# 从键盘获取输入值

```rust
use std::io;

fn main(){
    println!("猜数字游戏");
    println!("猜数字：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取错误");

    println!("你输入的数字为：{}",guess);

}
```

## 终端

![image-20231015112050587](http://qny.expressisland.cn/dian/image-20231015112050587.png)

## 代码详解

这些代码包含很多信息，我们一行一行地过一遍。为了获取用户输入并打印结果作为输出，我们需要将 `io`（输入/输出）库引入当前作用域。`io` 库来自于标准库（也被称为 `std`）：

```rust
use std::io;
```

默认情况下，Rust 将 [*prelude*](https://doc.rust-lang.org/std/prelude/index.html) 模块中少量的类型引入到每个程序的作用域中。如果需要的类型不在 prelude 中，你必须使用 `use` 语句显式地将其引入作用域。`std::io` 库提供很多有用的功能，包括接收用户输入的功能。

如第一章所提及，`main` 函数是程序的入口点：

```rust
fn main() {
```

`fn` 语法声明了一个新函数，`()` 表明没有参数，`{` 作为函数体的开始。

第一章也提及了 `println!` 是一个在屏幕上打印字符串的宏：

```rust
println!("Guess the number!");

println!("Please input your guess.");
```

这些代码仅仅打印提示，介绍游戏的内容然后请求用户输入。

### 使用变量储存值

接下来，创建一个储存用户输入的地方，像这样：

```rust
let mut guess = String::new();
```

现在程序开始变得有意思了！这一小行代码发生了很多事。注意这是一个 `let` 语句，用来创建 **变量**（*variable*）。这里是另外一个例子：

```rust
let foo = bar;
```

这行代码新建了一个叫做 `foo` 的变量并把它绑定到值 `bar` 上。在 Rust 中，变量默认是不可变的。我们将会在第三章的 [“变量与可变性”](https://rust.bootcss.com/ch03-01-variables-and-mutability.html#variables-and-mutability) 部分详细讨论这个概念。下面的例子展示了如何在变量名前使用 `mut` 来使一个变量可变：

```rust
let foo = 5; // 不可变
let mut bar = 5; // 可变
```

让我们回到猜猜看程序中。现在我们知道了 `let mut guess` 会引入一个叫做 `guess` 的可变变量。等号（`=`）的右边是 `guess` 所绑定的值，它是 `String::new` 的结果，这个函数会返回一个 `String` 的新实例。[`String`](https://doc.rust-lang.org/std/string/struct.String.html) 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块。

`::new` 那一行的 `::` 语法表明 `new` 是 `String` 类型的一个 **关联函数**（*associated function*）。关联函数是针对类型实现的，在这个例子中是 `String`，而不是 `String` 的某个特定实例。一些语言中把它称为 **静态方法**（*static method*）。

`new` 函数创建了一个新的空字符串，你会发现很多类型上有 `new` 函数，因为它是创建类型实例的惯用函数名。

总结一下，`let mut guess = String::new();` 这一行创建了一个可变变量，当前它绑定到一个新的 `String` 空实例上。

回忆一下，我们在程序的第一行使用 `use std::io;` 从标准库中引入了输入/输出功能。现在调用 `io` 库中的函数 `stdin`：

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

如果程序的开头没有 `use std::io` 这一行，可以把函数调用写成 `std::io::stdin`。`stdin` 函数返回一个 [`std::io::Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html) 的实例，这代表终端标准输入句柄的类型。

代码的下一部分，`.read_line(&mut guess)`，调用 [`read_line`](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line) 方法从标准输入句柄获取用户输入。我们还向 `read_line()` 传递了一个参数：`&mut guess`。

`read_line` 的工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数。这个字符串参数应该是可变的，以便 `read_line` 将用户输入附加上去。

`&` 表示这个参数是一个 **引用**（*reference*），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。完成当前程序并不需要了解如此多细节。现在，我们只需知道它像变量一样，默认是不可变的。因此，需要写成 `&mut guess` 来使其可变，而不是 `&guess`。

### 使用 `Result` 类型来处理潜在的错误

我们还没有完全分析完这行代码。虽然这是单独一行代码，但它是一个逻辑行（虽然换行了但仍是一个语句）的第一部分。第二部分是这个方法：

```rust
.expect("Failed to read line");
```

当使用 `.foo()` 语法调用方法时，通过换行加缩进来把长行拆开是明智的。我们完全可以这样写：

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

不过，过长的行难以阅读，所以最好拆开来写，两个方法调用占两行。现在来看看这行代码干了什么。

之前提到了 `read_line` 将用户输入附加到传递给它的字符串中，不过它也返回一个值——在这个例子中是 [`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html)。Rust 标准库中有很多叫做 `Result` 的类型：一个通用的 [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) 以及在子模块中的特化版本，比如 `io::Result`。

`Result` 类型是 [*枚举*（*enumerations*）](https://rust.bootcss.com/ch06-00-enums.html)，通常也写作 *enums*。枚举类型持有固定集合的值，这些值被称为枚举的 **成员**（*variants*）。

`Result` 的成员是 `Ok` 和 `Err`，`Ok` 成员表示操作成功，内部包含成功时产生的值。`Err` 成员则意味着操作失败，并且包含失败的前因后果。

这些 `Result` 类型的作用是编码错误处理信息。`Result` 类型的值，像其他类型一样，拥有定义于其上的方法。`io::Result` 的实例拥有 [`expect` 方法](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)。如果 `io::Result` 实例的值是 `Err`，`expect` 会导致程序崩溃，并显示当做参数传递给 `expect` 的信息。如果 `read_line` 方法返回 `Err`，则可能是来源于底层操作系统错误的结果。如果 `io::Result` 实例的值是 `Ok`，`expect` 会获取 `Ok` 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。

如果不调用 `expect`，程序也能编译，不过会出现一个警告：

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

Rust 警告我们没有使用 `read_line` 的返回值 `Result`，说明有一个可能的错误没有处理。

消除警告的正确做法是实际编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 `expect`。

### 使用 `println!` 占位符打印值

除了位于结尾的大括号，目前为止就只有这一行代码值得讨论一下了，就是这一行：

```rust
println!("You guessed: {}", guess);
```

这行代码打印存储用户输入的字符串。第一个参数是格式化字符串，里面的 `{}` 是预留在特定位置的占位符。使用 `{}` 也可以打印多个值：第一对 `{}` 使用格式化字符串之后的第一个值，第二对则使用第二个值，依此类推。调用一次 `println!` 打印多个值看起来像这样：

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

这行代码会打印出 `x = 5 and y = 10`。

# 生成一个随机数

* 接下来，需要生成一个随机数，好让用户来猜。

* 秘密数字应该每次都不同，这样重复玩才不会乏味；范围应该在 1 到 100 之间，这样才不会太困难。Rust 标准库中尚未包含随机数功能。然而，Rust 团队还是提供了一个 [`rand` crate](https://crates.io/crates/rand)。

* 首先，我们新增了一行 `use`：`use rand::Rng`。`Rng` 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。第十章会详细介绍 trait。

  接下来，我们在中间还新增加了两行。`rand::thread_rng` 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。接下来，调用随机数生成器的 `gen_range` 方法。这个方法由刚才引入到作用域的 `Rng` trait 定义。`gen_range` 方法获取两个数字作为参数，并生成一个范围在两者之间的随机数。它包含下限但不包含上限，所以需要指定 `1` 和 `50` 来请求一个 1 和 49 之间的数。

```rust
use std::io;
use rand::Rng;

fn main(){
    println!("猜数字游戏");
    let secret_number = rand::thread_rng().gen_range(1,50);
    println!("secret_number随机数为：{}",secret_number);
    println!("猜数字：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取错误");

    println!("你输入的数字为：{}",guess);

}
```

## 输出

![image-20231015114120982](http://qny.expressisland.cn/dian/image-20231015114120982.png)

## [`rand` crate](https://crates.io/crates/rand)

* 使用 crate 来增加更多功能

* 记住，*crate* 是一个 Rust 代码包。我们正在构建的项目是一个 **二进制 crate**，它生成一个可执行文件。 `rand` crate 是一个 **库 crate**，库 crate 可以包含任意能被其他程序使用的代码。

  Cargo 对外部 crate 的运用是其真正闪光的地方。在我们使用 `rand` 编写代码之前，需要修改 *Cargo.toml* 文件，引入一个 `rand` 依赖。现在打开这个文件并在底部的 `[dependencies]` 片段标题之下添加：

## Cargo.toml

```toml
[package]
name = "GuessNumber"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

rand = "0.5.5"
```

## Cargo.lock 文件确保构建是可重现的

* Cargo 有一个机制来确保任何人在任何时候重新构建代码，都会产生相同的结果：Cargo 只会使用你指定的依赖版本，除非你又手动指定了别的。例如，如果下周 `rand` crate 的 `0.5.6` 版本出来了，它修复了一个重要的 bug，同时也含有一个会破坏代码运行的缺陷，这时会发生什么呢？
* 这个问题的答案是 *Cargo.lock* 文件。它在第一次运行 `cargo build` 时创建，并放在 *guessing_game* 目录。当第一次构建项目时，Cargo 计算出所有符合要求的依赖版本并写入 *Cargo.lock* 文件。当将来构建项目时，Cargo 会发现 *Cargo.lock* 已存在并使用其中指定的版本，而不是再次计算所有的版本。这使得你拥有了一个自动化的可重现的构建。换句话说，项目会持续使用 `0.5.5` 直到你显式升级，多亏有了 *Cargo.lock* 文件。

# 比较猜测的数字和秘密数字

```rust
use std::io;
use rand::Rng;  
use std::cmp::Ordering;  //比较猜测的数字和秘密数字

fn main(){
    println!("猜数字游戏");
    let secret_number = rand::thread_rng().gen_range(1,50);
    println!("secret_number随机数为：{}",secret_number);
    println!("猜数字：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取错误");

    //比较猜测的数字和秘密数字
    let guess: u32 = guess.trim().parse().expect("请输入一个数字!");
    println!("你输入的数字为：{}",guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("小了"),
        Ordering::Greater => println!("大了"),
        Ordering::Equal => println!("对了"),
    }

}
```

## 输出

![image-20231015115342705](http://qny.expressisland.cn/dian/image-20231015115342705.png)

## 详解

新代码的第一行是另一个 `use`，从标准库引入了一个叫做 `std::cmp::Ordering` 的类型。同 `Result` 一样， `Ordering` 也是一个枚举，不过它的成员是 `Less`、`Greater` 和 `Equal`。这是比较两个值时可能出现的三种结果。

接着，底部的五行新代码使用了 `Ordering` 类型，`cmp` 方法用来比较两个值并可以在任何可比较的值上调用。它获取一个被比较值的引用：这里是把 `guess` 与 `secret_number` 做比较。 然后它会返回一个刚才通过 `use` 引入作用域的 `Ordering` 枚举的成员。使用一个 [`match`](https://rust.bootcss.com/ch06-02-match.html) 表达式，根据对 `guess` 和 `secret_number` 调用 `cmp` 返回的 `Ordering` 成员来决定接下来做什么。

一个 `match` 表达式由 **分支（arms）** 构成。一个分支包含一个 **模式**（*pattern*）和表达式开头的值与分支模式相匹配时应该执行的代码。Rust 获取提供给 `match` 的值并挨个检查每个分支的模式。`match` 结构和模式是 Rust 中强大的功能，它体现了代码可能遇到的多种情形，并帮助你确保没有遗漏处理。这些功能将分别在第六章和第十八章详细介绍。

让我们看看使用 `match` 表达式的例子。假设用户猜了 50，这时随机生成的秘密数字是 38。比较 50 与 38 时，因为 50 比 38 要大，`cmp` 方法会返回 `Ordering::Greater`。`Ordering::Greater` 是 `match` 表达式得到的值。它检查第一个分支的模式，`Ordering::Less` 与 `Ordering::Greater`并不匹配，所以它忽略了这个分支的代码并来到下一个分支。下一个分支的模式是 `Ordering::Greater`，**正确** 匹配！这个分支关联的代码被执行，在屏幕打印出 `Too big!`。`match` 表达式就此终止，因为该场景下没有检查最后一个分支的必要。

错误的核心表明这里有 **不匹配的类型**（*mismatched types*）。Rust 有一个静态强类型系统，同时也有类型推断。当我们写出 `let guess = String::new()` 时，Rust 推断出 `guess` 应该是 `String` 类型，并不需要我们写出类型。另一方面，`secret_number`，是数字类型。几个数字类型拥有 1 到 100 之间的值：32 位数字 `i32`；32 位无符号数字 `u32`；64 位数字 `i64` 等等。Rust 默认使用 `i32`，所以它是 `secret_number` 的类型，除非增加类型信息，或任何能让 Rust 推断出不同数值类型的信息。这里错误的原因在于 Rust 不会比较字符串类型和数字类型。

所以我们必须把从输入中读取到的 `String` 转换为一个真正的数字类型，才好与秘密数字进行比较。这可以通过在 `main` 函数体中增加如下两行代码来实现：

```rust
    let guess: u32 = guess.trim().parse().expect("请输入一个数字!");
```

这里创建了一个叫做 `guess` 的变量。不过等等，不是已经有了一个叫做 `guess` 的变量了吗？确实如此，不过 Rust 允许用一个新值来 **隐藏** （*shadow*） `guess` 之前的值。这个功能常用在需要转换值类型之类的场景。它允许我们复用 `guess` 变量的名字，而不是被迫创建两个不同变量，诸如 `guess_str` 和 `guess` 之类。

我们将 `guess` 绑定到 `guess.trim().parse()` 表达式上。表达式中的 `guess` 是包含输入的原始 `String` 类型。`String` 实例的 `trim` 方法会去除字符串开头和结尾的空白字符。`u32` 只能由数字字符转换，不过用户必须输入 enter 键才能让 `read_line` 返回，然而用户按下 enter 键时，会在字符串中增加一个换行（newline）符。例如，用户输入 5 并按下 enter，`guess` 看起来像这样：`5\n`。`\n` 代表 “换行”，回车键。`trim` 方法消除 `\n`，只留下 `5`。

[字符串的 `parse` 方法](https://doc.rust-lang.org/std/primitive.str.html#method.parse) 将字符串解析成数字。因为这个方法可以解析多种数字类型，因此需要告诉 Rust 具体的数字类型，这里通过 `let guess: u32` 指定。`guess` 后面的冒号（`:`）告诉 Rust 我们指定了变量的类型。Rust 有一些内建的数字类型；`u32` 是一个无符号的 32 位整型。对于不大的正整数来说，它是不错的类型，第三章还会讲到其他数字类型。另外，程序中的 `u32` 注解以及与 `secret_number` 的比较，意味着 Rust 会推断出 `secret_number` 也是 `u32` 类型。现在可以使用相同类型比较两个值了！

`parse` 调用很容易产生错误。例如，字符串中包含 `A%`，就无法将其转换为一个数字。因此，`parse` 方法返回一个 `Result` 类型。像之前 [“使用 `Result` 类型来处理潜在的错误”](https://rust.bootcss.com/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type) 讨论的 `read_line` 方法那样，再次按部就班的用 `expect` 方法处理即可。如果 `parse` 不能从字符串生成一个数字，返回一个 `Result` 的 `Err` 成员时，`expect` 会使游戏崩溃并打印附带的信息。如果 `parse` 成功地将字符串转换为一个数字，它会返回 `Result` 的 `Ok` 成员，然后 `expect` 会返回 `Ok` 值中的数字。

# 使用循环来允许多次猜测

* `loop` 关键字创建了一个无限循环。将其加入后，用户可以反复猜测。

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("猜数字游戏");
    let secret_number = rand::thread_rng().gen_range(1,50);

    loop{

    println!("secret_number随机数为：{}",secret_number);
    println!("猜数字：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取错误");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

        println!("你输入的数字为：{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {println!("对了");
            break;
        }
    }
    }
}
```

## 输出

![image-20231015120008202](http://qny.expressisland.cn/dian/image-20231015120008202.png)

## 猜测正确后退出

* 增加一个 `break` 语句，在用户猜对时退出游戏
* 通过在 `对了!` 之后增加一行 `break`，用户猜对了神秘数字后会退出循环。退出循环也意味着退出程序，因为循环是 `main` 的最后一部分。

```rust
        match guess.cmp(&secret_number){
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {println!("对了");
            break;
        }
    }
```

## 处理无效输入

* 为了进一步改善游戏性，不要在用户输入非数字时崩溃，需要忽略非数字，让用户可以继续猜测。可以通过修改 `guess` 将 `String` 转化为 `u32` 那部分代码来实现。

```rust
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
```

将 `expect` 调用换成 `match` 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法。须知 `parse` 返回一个 `Result` 类型，而 `Result` 是一个拥有 `Ok` 或 `Err` 成员的枚举。这里使用的 `match` 表达式，和之前处理 `cmp` 方法返回 `Ordering` 时用的一样。

如果 `parse` 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 `Ok`。这个 `Ok` 值与 `match` 第一个分支的模式相匹配，该分支对应的动作返回 `Ok` 值中的数字 `num`，最后如愿变成新创建的 `guess` 变量。

如果 `parse` *不* 能将字符串转换为一个数字，它会返回一个包含更多错误信息的 `Err`。`Err` 值不能匹配第一个 `match` 分支的 `Ok(num)` 模式，但是会匹配第二个分支的 `Err(_)` 模式：`_` 是一个通配符值，本例中用来匹配所有 `Err` 值，不管其中有何种信息。所以程序会执行第二个分支的动作，`continue` 意味着进入 `loop` 的下一次循环，请求另一个猜测。这样程序就有效的忽略了 `parse` 可能遇到的所有错误！