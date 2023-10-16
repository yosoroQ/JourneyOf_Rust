# Rust 之旅 - 斐波那契数列 & 转换摄氏度与华氏温度 

# 斐波那契数列

```rust
//生成 n 阶斐波那契数列。
fn main() {
    let n: i32 = 5;
    println!("fib({}) = {}", n, fib(n));
    let n: i32 = 6;
    println!("fib({}) = {}", n, fib(n));
    let n: i32 = 40;
    println!("fib({}) = {}", n, fib(n));
}

fn fib(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

# 转换摄氏度与华氏温度 

```rust
// 相互转换摄氏与华氏温度
fn main() {
	let celsius : f64 = 10.0; //摄氏温度
	let fahrenheit : f64 = (celsius * 1.8) + 32.0; // 华氏温度
	println!("{} 摄氏温度转为华氏温度为 {}", celsius, fahrenheit)
}
```



