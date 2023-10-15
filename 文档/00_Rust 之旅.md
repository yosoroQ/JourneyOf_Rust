# Rust 之旅

- 在 Linux、macOS 和 Windows 上安装 Rust
- 编写一个打印 `Hello, world!` 的程序
- 使用 Rust 的包管理器和构建系统 `cargo`

# 在 Windows 上安装 Rust

## 使用 Rustup（推荐）

* https://www.rust-lang.org/zh-CN/tools/install
* ![image-20231015100018461](http://qny.expressisland.cn/dian/image-20231015100018461.png)

* 选择第一项”继续安装“
* ![image-20231015100117731](http://qny.expressisland.cn/dian/image-20231015100117731.png)
* ![image-20231015100134054](http://qny.expressisland.cn/dian/image-20231015100134054.png)

## 测试是否安装完成

* powershell中输入`rustc --version`或输入`rustup doc` 在浏览器中查看本地文档。
* ![image-20231015100425878](http://qny.expressisland.cn/dian/image-20231015100425878.png)
* ![image-20231015100457431](http://qny.expressisland.cn/dian/image-20231015100457431.png)

# `Hello, world!` 

## 使用vscode编译

* 安装rust-analyzer拓展![image-20231015104138188](http://qny.expressisland.cn/dian/image-20231015104138188.png)

```rust
fn main(){
    println!("hello!");
}
```

## 终端输出

```cmake
PS C:\Users\yosoro> cd C:\Users\yosoro\Desktop
PS C:\Users\yosoro\Desktop> rustc hello.rs
PS C:\Users\yosoro\Desktop> ./hello.exe
hello!
```

# 使用 Rust 的包管理器和构建系统 `cargo`

* Cargo 是 Rust 的构建系统和包管理器。大多数 Rustacean 们使用 Cargo 来管理他们的 Rust 项目，因为它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。（我们把代码所需要的库叫做 **依赖**（*dependencies*））。
* 最简单的 Rust 程序，比如我们刚刚编写的，没有任何依赖。所以如果使用 Cargo 来构建 “Hello, world!” 项目，将只会用到 Cargo 构建代码的那部分功能。在编写更复杂的 Rust 程序时，你将添加依赖项，如果使用 Cargo 启动项目，则添加依赖项将更容易。

## 测试是否安装了 Cargo

* `cargo --version`
* ![image-20231015104918055](http://qny.expressisland.cn/dian/image-20231015104918055.png)

## 使用 Cargo 创建项目

### 终端输入

```cmd
cargo new cargo_test
```

* ![image-20231015105225510](http://qny.expressisland.cn/dian/image-20231015105225510.png)

## 构建并运行 Cargo 项目

### 使用`cargo build`命令

```rust
cargo build
.\target\debug\cargo_test.exe
```

### 输出

![image-20231015105706851](http://qny.expressisland.cn/dian/image-20231015105706851.png)

###  `cargo run` 

* 我们刚刚使用 `cargo build` 构建了项目，并使用 `./target/debug/hello_cargo` 运行了程序，也可以使用 `cargo run` 在一个命令中同时编译并运行生成的可执行文件

```rust
cargo run
```

![image-20231015105753383](http://qny.expressisland.cn/dian/image-20231015105753383.png)

### `cargo check` 

* Cargo 还提供了一个叫 `cargo check` 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件。
* 为什么你会不需要可执行文件呢？通常 `cargo check` 要比 `cargo build` 快得多，因为它省略了生成可执行文件的步骤。如果你在编写代码时持续的进行检查，`cargo check` 会加速开发！为此很多 Rustaceans 编写代码时定期运行 `cargo check` 确保它们可以编译。当准备好使用可执行文件时才运行 `cargo build`。
* ![image-20231015105923653](http://qny.expressisland.cn/dian/image-20231015105923653.png)

## 发布（release）构建

* 当项目最终准备好发布时，可以使用 `cargo build --release` 来优化编译项目。这会在 *target/release* 而不是 *target/debug* 下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。如果你在测试代码的运行时间，请确保运行 `cargo build --release` 并使用 *target/release* 下的可执行文件进行测试。

## 把 Cargo 当作习惯

* 对于简单项目， Cargo 并不比 `rustc` 提供了更多的优势，不过随着开发的深入，终将证明其价值。对于拥有多个 crate 的复杂项目，交给 Cargo 来协调构建将简单的多。

* 即便 `hello_cargo` 项目十分简单，它现在也使用了很多在你之后的 Rust 生涯将会用到的实用工具。其实，要在任何已存在的项目上工作时，可以使用如下命令通过 Git 检出代码，移动到该项目目录并构建：

```rust
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

