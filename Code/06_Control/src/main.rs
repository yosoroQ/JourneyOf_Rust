//if 表达式
fn main() {
    let number = 3;

    if number < 5 {
        println!("yes");
    } else {
        println!("no");
    }
}

//代码中的条件**必须**是 `bool` 值
fn main() {
    let number = 3;

    if number {
        println!("3?");
    }
}

//修改if表达式：
//如果想要 `if` 代码块只在一个数字不等于 `0` 时执行
fn main() {
    let number = 3;

    if number != 0 {

        //不等于 `0` 时执行
        println!("不等于 `0` 时执行");
    }
}

//使用 else if 处理多重条件
fn main() {
    let number = 6;

    //3
    if number % 4 == 0 {
        println!("4");
    } else if number % 3 == 0 {
        println!("3");
    } else if number % 2 == 0 {
        println!("2");
    } else {
        println!("4, 3, or 2");
    }
}

//在 let 语句中使用 if
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    //number值: 5
    println!("number值: {}", number);
}

//如果它们的类型不匹配，则会出现一个错误
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("number值: {}", number);
}

//使用 `loop` 重复执行代码
fn main() {
    loop {
        println!("loop!");
    }
}

//从循环返回
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    //result值是 20
    println!("result值是 {}", result);
}

//示例：while条件循环
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    //3!
    // 2!
    // 1!
    // LIFTOFF!!!
    println!("LIFTOFF!!!");
}

//如果使用 `while` 结构来遍历集合中的元素，比如数组，会有什么缺点？
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    //the value is: 10
// the value is: 20
// the value is: 30
// the value is: 40
// the value is: 50
}

//更简洁的替代方案，使用 `for` 循环来对一个集合执行每个元素
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

//     the value is: 10
// the value is: 20
// the value is: 30
// the value is: 40
// the value is: 50
}

//示例：使用 `for` 循环来倒计时，`rev`，用来反转 range
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

//     3!
// 2!
// 1!
// LIFTOFF!!!
}