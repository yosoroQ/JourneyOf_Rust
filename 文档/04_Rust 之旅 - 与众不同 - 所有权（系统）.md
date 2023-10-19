# Rust 之旅 - 与众不同 - 所有权（系统）

- 所有权（系统）是 Rust 最为与众不同的特性。
- 它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全。

* 所有权以及相关功能：**借用**、**slice** 以及 **Rust 如何在内存中布局数据**。

# 什么是所有权？

* 所有运行的程序都必须管理其使用计算机内存的方式。
* 一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；
* 在另一些语言中，程序员必须亲自分配和释放内存。
* Rust 则选择了第三种方式：**通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查**。
* 在运行时，所有权系统的任何功能都不会减慢程序。

# 栈（Stack）与堆（Heap）

* 在很多语言中，你并不需要经常考虑到栈与堆。
* 不过在像 Rust 这样的系统编程语言中，值是位于栈上还是堆上在更大程度上影响了语言的行为以及为何必须做出这样的抉择。
* 栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。
* 栈以放入值的顺序存储值并以相反顺序取出值，这也被称作 **后进先出**。
* 增加数据叫做 **进栈**（*pushing onto the stack*），而移出数据叫做 **出栈**（*popping off the stack*）。
* 栈中的所有数据都必须占用已知且固定的大小。
* 在编译时大小未知或大小可能变化的数据，要改为存储在堆上。
* 堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。
* 操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 **指针**（*pointer*）。
* 这个过程称作 **在堆上分配内存**（*allocating on the heap*），有时简称为 “分配”（allocating）。
* 将数据推入栈中并不被认为是分配，因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。
* 入栈比在堆上分配内存要快，因为（入栈时）操作系统无需为存储新数据去搜索内存空间，其位置总是在栈顶。
* 相比之下，在堆上分配内存则需要更多的工作，这是因为操作系统必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。
* 访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。
* 现代处理器在内存中跳转越少就越快（缓存），出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。
* 当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中，当函数结束时，这些值被移出栈。
* **跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间**，这些问题正是**所有权系统要处理**的。
* 一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的存在就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。

# 所有权规则

* Rust 中的每一个值都有一个被称为其 **所有者**（*owner*）的变量。
* 值有且只有一个所有者。
* 当所有者（变量）离开作用域，这个值将被丢弃。

# 变量作用域

* 变量的 **作用域**（*scope*）。
* 作用域是一个项（item）在程序中有效的范围。
* 变量 `s` 绑定到了一个字符串字面值，这个字符串值是硬编码进程序代码中的。
* 这个变量从声明的点开始直到当前 **作用域** 结束时都是有效的。

```rust
fn main() {                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的

    // 使用 s
}                      // 此作用域已结束，s 不再有效
```

- 当 `s` **进入作用域** 时，它就是有效的。
- 这一直持续到它 **离开作用域** 为止。

# String 类型

* 前面介绍的类型都是存储在栈上的并且当离开作用域时被移出栈，不过我们需要寻找**一个存储在堆上的数据来探索 Rust 是如何知道该在何时清理数据**的。
* 这里使用 `String` 作为例子，并专注于 `String` 与所有权相关的部分。
* 我们已经见过字符串字面值，字符串值被硬编码进程序里。
* 字符串字面值是很方便的，不过他们并不适合使用文本的每一种场景，原因之一就是他们是不可变的，另一个原因是并不是所有字符串的值都能在编写代码时就知道。
* 例如，要是想获取用户输入并存储该怎么办呢？
* 为此，Rust 有第二个字符串类型，`String`，这个类型被分配到堆上，所以能够存储在编译时未知大小的文本。
* **可以使用 `from` 函数基于字符串字面值来创建 `String`**。

```rust
let s = String::from("hello");
```

* 这两个冒号（`::`）是运算符，允许将特定的 `from` 函数置于 `String` 类型的命名空间（namespace）下，而不需要使用类似 `string_from` 这样的名字。
* 可以修改此类字符串：

```rust
//**可以使用 `from` 函数基于字符串字面值来创建 `String`**。
fn main(){
    //原
    let s = String::from("hello");

    //可以 修改此类字符串
    let mut s = String::from("hello");
    s.push_str(",world");// push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
}
```

* 那么这里有什么区别呢？
* 为什么 `String` 可变而字面值却不行呢？
* **区别在于两个类型对内存的处理上。**

# 内存与分配

* 就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中，这使得字符串字面值快速且高效，不过这些特性都只得益于字符串字面值的不可变性。
* 不幸的是，我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。
* 对于 `String` 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。
* 这意味着：必须在运行时向操作系统请求内存，需要一个当我们处理完 `String` 时将内存返回给操作系统的方法。
* 第一部分由我们完成：当调用 `String::from` 时，它的实现 (*implementation*) 请求其所需的内存。
* 然而，第二部分实现起来就各有区别了。
* 在有 **垃圾回收**（*garbage collector*，*GC*）的语言中， GC 记录并清除不再使用的内存，而我们并不需要关心它。没有 GC 的话，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。如果重复回收，这也是个 bug。
* 我们需要**精确的为一个 `allocate` 配对一个 `free`**。
* Rust 采取了一个不同的策略：**内存在拥有它的变量离开作用域后就被自动释放**。

## 作用域例子的一个使用 `String` 而不是字符串字面值的版本

```rust
//作用域例子的一个使用 `String` 而不是字符串字面值的版本
fn main(){
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
}                                  // 此作用域已结束，
                          // s 不再有效
```

* 这是一个将 `String` 需要的内存返回给操作系统的很自然的位置：当 `s` 离开作用域的时候。
* 当变量离开作用域，Rust 为我们调用一个特殊的函数，这个函数叫做 `drop`，在这里 `String` 的作者可以放置释放内存的代码。
* Rust 在结尾的 `}` 处自动调用 `drop`。

> 注意：在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 **资源获取即初始化**（*Resource Acquisition Is Initialization (RAII)*）。

## 变量与数据交互的方式（一）：移动

* Rust 中的多个变量可以采用一种独特的方式与同一数据交互。

### 使用整型的例子

```rust
let x = 5;
let y = x;
```

### 将变量 `x` 的整数值赋给 `y`

* “将 `5` 绑定到 `x`，接着生成一个值 `x` 的拷贝并绑定到 `y`”。
* 现在有了两个变量，`x` 和 `y`，都等于 `5`。
* 因为整数是有已知固定大小的简单值，所以这两个 `5` 被放入了栈中。

* 现在看看这个 `String` 版本：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

* 之前我们提到过当变量离开作用域后，Rust 自动调用 `drop` 函数并清理变量的堆内存。
* 不过两个数据指针指向了同一位置，这就有了一个问题：当 `s2` 和 `s1` 离开作用域，他们都会尝试释放相同的内存。
* 这是一个叫做 **二次释放**（*double free*）的错误，也是之前提到过的内存安全性 bug 之一。
* 两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
* 为了确保内存安全，这种场景下 Rust 的处理有另一个细节值得注意。
* 与其尝试拷贝被分配的内存，Rust 则认为 `s1` 不再有效，因此 Rust 不需要在 `s1` 离开作用域后清理任何东西。
* 看看在 `s2` 被创建之后尝试使用 `s1` 会发生什么，**这段代码不能运行**：

```rust
//变量与数据交互的方式（一）：移动
fn main(){
    let s1 = String::from("hello");
    let s2 = s1;
    
    println!("{}, world!", s1);
}             
```

* 你会得到一个类似如下的错误，因为 Rust 禁止你使用无效的引用。

* ![image-20231019213934138](http://qny.expressisland.cn/dian/image-20231019213934138.png)

* 因为 Rust 同时使第一个变量无效了，这个操作被称为 **移动**（*move*），而不是浅拷贝。
* 因为只有 `s2` 是有效的，当其离开作用域，它就释放自己的内存完毕。
* 另外，这里还隐含了一个设计选择：**Rust 永远也不会自动创建数据的 “深拷贝”**。
* 因此，任何 **自动** 的复制可以被认为对运行时性能影响较小。

## 变量与数据交互的方式（二）：克隆

* 如果我们 **确实** 需要深度复制 `String` 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 `clone` 的通用函数。


### 使用 `clone` 方法

```rust
//变量与数据交互的方式（二）：克隆
fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("{}, world!", s1);
}
```

* 这里堆上的数据 **确实** 被复制了。
* 当出现 `clone` 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。

## 只在栈上的数据：拷贝

* 这些代码使用了整型并且是有效的。

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

* 但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 `clone`，不过 `x` 依然有效且没有被移动到 `y` 中。
* 原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
* 这意味着没有理由在创建变量 `y` 后使 `x` 无效。
* 换句话说，这里没有深浅拷贝的区别，所以这里调用 `clone` 并不会与通常的浅拷贝有什么不同，我们可以不用管它。
* Rust 有一个叫做 `Copy` trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。
* 如果一个类型拥有 `Copy` trait，一个旧的变量在将其赋值给其他变量后仍然可用。
* Rust 不允许自身或其任何部分实现了 `Drop` trait 的类型使用 `Copy` trait。
* 如果我们对其值离开作用域时需要特殊处理的类型使用 `Copy` 注解，将会出现一个编译时错误。

### 什么类型是 `Copy`

* 作为一个通用的规则，任何简单标量值的组合可以是 `Copy` 的，不需要分配内存或某种形式资源的类型是 `Copy` 的。
* 如下是一些 `Copy` 的类型：

> - 所有整数类型，比如 `u32`。
> - 布尔类型，`bool`，它的值是 `true` 和 `false`。
> - 所有浮点数类型，比如 `f64`。
> - 字符类型，`char`。
> - 元组，当且仅当其包含的类型也都是 `Copy` 的时候，比如，`(i32, i32)` 是 `Copy` 的，但 `(i32, String)` 就不是。

# 所有权与函数

* 将值传递给函数在语义上与给变量赋值相似。
* 向函数传递值可能会移动或者复制，就像赋值语句一样。

## 带有所有权和作用域注释的函数

* 当尝试在调用 `takes_ownership` 后使用 `s` 时，Rust 会抛出一个编译时错误，这些静态检查使我们免于犯错。

```rust
//带有所有权和作用域注释的函数
fn main(){
    let s1 = String::from("hello");  // s1 进入作用域
    takes_ownership(s1);  // s1 的值移动到函数里 ...
                             // ... 所以到这里不再有效
    let x = 5;             // x 进入作用域
    makes_copy(x);           // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    
}        // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String){ // some_string 进入作用域

    //hello

    println!("{}", some_string); // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
}

fn makes_copy(some_integer: i32){ // some_integer 进入作用域

    //5
    println!("{}", some_integer); // 这里，some_integer 移出作用域。不会有特殊操作
}
```

# 返回值与作用域

## 返回值也可以转移所有权（转移返回值的所有权）

* 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
* 当持有堆中数据值的变量离开作用域时，其值将通过 `drop` 被清理掉，除非数据被移动为另一个变量所有。

### 无注释版

```rust
fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}
```

### 注释版

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中, 
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
```

## 使用元组来返回多个值（返回参数的所有权）

```rust
//使用元组来返回多个值（返回参数的所有权）
fn main(){
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);

    //The length of 'hello' is 5.
    println!("The length of '{}' is {}.",s2,len);
}

fn calculate_length(s:String) -> (String,usize){
    let length = s.len(); // len() 返回字符串的长度
    (s,length)
}
```

* 但是这未免有些形式主义，而且这种场景应该很常见。
* 幸运的是，Rust 对此提供了一个功能，叫做 **引用**（*references*）。

# 引用与借用

* 使用元组来返回多个值（返回参数的所有权）有这样一个问题：我们必须将 `String` 返回给调用函数，以便在调用 `calculate_length` 后仍能使用 `String`，因为 `String` 被移动到了 `calculate_length` 内。
* 下面是如何定义并使用一个（新的）`calculate_length` 函数，它以一个对象的引用作为参数而不是获取值的所有权：

## 以一个对象的引用作为参数而不是获取值的所有权

* 首先，注意变量声明和函数返回值中的所有元组代码都消失了。
* 其次，注意我们传递 `&s1` 给 `calculate_length`，同时在函数定义中，我们获取 `&String` 而不是 `String`。
* 这些 & 符号就是 **引用**，它们**允许你使用值但不获取其所有权**。

```rust
//以一个对象的引用作为参数而不是获取值的所有权
fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    //The length of 'hello' is 5.
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize{
    s.len()
}
```

> 注意：与使用 `&` 引用相反的操作是 **解引用**（*dereferencing*），它使用解引用运算符，`*`。

* `&s1` 语法让我们创建一个 **指向** 值 `s1` 的引用，但是并不拥有它。
* 因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
* 同理，函数签名使用 `&` 来表明参数 `s` 的类型是一个引用。
* 变量 `s` 有效的作用域与函数参数的作用域一样，不过当引用离开作用域后并不丢弃它指向的数据，因为我们没有所有权。
* 当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。
* 我们将获取引用作为函数参数称为 **借用**（*borrowing*）。

## 尝试修改借用的变量（Wrong!）？

* 正如变量默认是不可变的，引用也一样。
* （默认）**不允许修改引用的值**。

```rust
//尝试修改借用的变量（Wrong!）？
fn main(){
    let s1 = String::from("hello");
    change(&s);
}
fn change(some_string: &String){
    some_string.push_str(", world");
}
```

### 错误提示

![image-20231019221659153](http://qny.expressisland.cn/dian/image-20231019221659153.png)

## 可变引用

### 修复上例代码中的错误

* 首先，必须将 `s` 改为 `mut`。
* 然后必须创建一个可变引用 `&mut s` 和接受一个可变引用 `some_string: &mut String`。

```rust
//可变引用
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

* 不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。

### 编译失败的代码

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

* 错误如下：

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```

* 这个限制允许可变性，不过是以一种受限制的方式允许。
* 新 Rustacean 们经常与此作斗争，因为大部分语言中变量任何时候都是可变的。
* 这个限制的好处是 Rust 可以在编译时就避免数据竞争。
* **数据竞争**（*data race*）类似于竞态条件，它可由这三个行为造成：

	* 两个或更多指针同时访问同一数据。
	* 至少有一个指针被用来写入数据。
	* 没有同步数据访问的机制。

* 数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；
* Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

### 使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 **同时** 拥有

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
```

### 类似的规则也存在于同时使用可变与不可变引用中（Wrong）

* 这些代码会导致一个错误：

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);
```

错误如下：

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here
```

* 不能在拥有不可变引用的同时拥有可变引用。
* 然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
* 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
* 例如，因为最后一次使用不可变引用在声明可变引用之前，所以如下代码是可以编译的：

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{} and {}", r1, r2);
// 此位置之后 r1 和 r2 不再使用

let r3 = &mut s; // 没问题
println!("{}", r3);
```

* 不可变引用 `r1` 和 `r2` 的作用域在 `println!` 最后一次使用之后结束，这也是创建可变引用 `r3` 的地方。
* 它们的作用域没有重叠，所以代码是可以编译的。
* 尽管这些错误有时使人沮丧，但请牢记这是 Rust 编译器在提前指出一个潜在的 bug（在编译时而不是在运行时）并精准显示问题所在。

## 悬垂引用（Dangling References）

* 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 **悬垂指针**（*dangling pointer*）。
* 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
* 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：**当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域**。

### 尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免

* 错误信息引用了一个我们还未介绍的功能：**生命周期（lifetimes）**。

```rust
//尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免
fn main(){
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s =String::from("hello");

    &s
}
```

![image-20231019223608209](http://qny.expressisland.cn/dian/image-20231019223608209.png)

* 因为 `s` 是在 `dangle` 函数内创建的，当 `dangle` 的代码执行完毕后，`s` 将被释放。
* 不过我们尝试返回它的引用，这意味着这个引用会指向一个无效的 `String`，这可不对！Rust 不会允许我们这么做。

### 解决方法：直接返回 `String`

* 这样就没有任何错误了。
* 所有权被移动出去，所以没有值被释放。

```rust
fn main(){
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();

}

// fn dangle() -> &String{
//     let s =String::from("hello");

//     &s
// }

//直接返回 String
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

## 引用的规则

- 在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用。
- 引用必须总是有效的。

# 另一种不同类型的引用：slice —— Slice 类型

* 另一个**没有所有权的数据类型**是 *slice*。
* slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。

## 字符串slice

* **字符串 slice**（*string slice*）是 `String` 中一部分值的引用，它看起来像这样：
* 这类似于引用整个 `String` 不过带有额外的 `[0..5]` 部分。
* 它不是对整个 `String` 的引用，而是对部分 `String` 的引用。
* 可以使用一个由中括号中的 `[starting_index..ending_index]` 指定的 range 创建一个 slice，其中 `starting_index` 是 slice 的第一个位置，`ending_index` 则是 slice 最后一个位置的后一个值。
* 在其内部，slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 `ending_index` 减去 `starting_index` 的值。所以对于 `let world = &s[6..11];` 的情况，`world` 将是一个包含指向 `s` 第 7 个字节（从 1 开始）的指针和长度值 5 的 slice。

```rust
//**字符串 slice**（*string slice*）是 `String` 中一部分值的引用
fn main(){
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

* 对于 Rust 的 `..` range 语法，如果想要从第一个索引（0）开始，可以不写两个点号之前的值。
* 换句话说，如下两个语句是相同的：

```rust
//对于 Rust 的 `..` range 语法，如果想要从第一个索引（0）开始，可以不写两个点号之前的值。
fn main(){
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}
```

* 依此类推，如果 slice 包含 `String` 的最后一个字节，也可以舍弃尾部的数字。
* 这意味着如下也是相同的：

```rust
//如果 slice 包含 `String` 的最后一个字节，也可以舍弃尾部的数字
fn main(){
    let s = String::from("hello");

    let len = s.len();
    
    let slice = &s[3..len];
    let slice = &s[3..];
}
```

* 也可以同时舍弃这两个值来获取整个字符串的 slice。
* 所以如下亦是相同的：

```rust
//也可以同时舍弃这两个值来获取整个字符串的 slice
fn main(){
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}
```

> 注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。
>
> 出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；

### 重写 `first_word` 来返回一个 slice，“字符串 slice” 的类型声明写作 `&str`

* 通过寻找第一个出现的空格。
* 当找到一个空格，我们返回一个字符串 slice，它使用字符串的开始和空格的索引作为开始和结束的索引。

```rust
//重写 `first_word` 来返回一个 slice，“字符串 slice” 的类型声明写作 `&str`
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

* 现在当调用 `first_word` 时，会返回与底层数据关联的单个值。
* 这个值由一个 slice 开始位置的引用和 slice 中元素的数量组成。
* `second_word` 函数也可以改为返回一个 slice：

```rust
fn second_word(s: &String) -> &str {
```

* 现在我们有了一个不易混淆且直观的 API 了，因为编译器会确保指向 `String` 的引用持续有效。

### 字符串字面值就是 slice

* 字符串字面值被储存在二进制文件中。
* 现在知道 slice 了，我们就可以正确的理解字符串字面值了：
* 这里 `s` 的类型是 `&str`：它是一个指向二进制程序特定位置的 slice。
* 这也就是为什么字符串字面值是不可变的；
* `&str` 是一个不可变引用。

```rust
let s = "Hello, world!";
```

#### 字符串 slice 作为参数

##### 通过将 `s` 参数的类型改为字符串 slice 来改进 `first_word` 函数

* 如果有一个字符串 slice，可以直接传递它。
* 如果有一个 `String`，则可以传递整个 `String` 的 slice。
* 定义一个获取字符串 slice 而不是 `String` 引用的函数使得我们的 API 更加通用并且不会丢失任何功能：

文件名: src/main.rs

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
```

## 其他类型的 slice

* 字符串 slice是针对字符串的。
* 不过也有更通用的 slice 类型，考虑一下这个数组：

```rust
let a = [1, 2, 3, 4, 5];
```

* 如果要引用数组的一部分，我们可以这样做：
* 这个 slice 的类型是 `&[i32]`。
* 它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。
* 你可以对其他所有集合使用这类 slice。

```rust
//其他类型的 slice
fn main(){
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}
```

# 总结

* 所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。
* Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。

