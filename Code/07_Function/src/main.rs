fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function(){
    println!("another");
}

//函数参数
fn main() {
    another_function(5);
}

fn another_function(x: i32){

    //x的值为：5
    println!("x的值为：{}",x);
}

//当一个函数有多个参数时，使用逗号分隔
fn main() {
    another_function(5,6);
}

fn another_function(x: i32,y: i32){

    //x的值为：5
// y的值为：6
    println!("x的值为：{}",x);
    println!("y的值为：{}",y);

}

// //包含语句和表达式的函数体
// //使用 let 关键字创建变量并绑定一个值是一个语句
fn main() {
    let y = 6;
}

//错误
fn main() {
    let x = (let y = 6);
}


//表达式（大括号（代码块））
fn main() {
    let x = 5;

     let y = {
        let x = 3;
        x + 1
     };

    //y的值为：4
     println!("y的值为：{}",y)
}

//具有返回值的函数
fn main(){
    let x = five();

    //x的值为：5
    println!("x的值为：{}",x);
}

fn five() -> i32 {
    5
}

//“mismatched types”（类型不匹配）
fn main(){
    let x = plus_five(5);

    //x的值为：6
    println!("x的值为：{}",x);
}

fn plus_five(x: i32) -> i32 {
    x + 1;
}
