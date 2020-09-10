
& 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝 例如 &mut guess
! 表示宏引用
* 取消引用
mut 可变的：例如 let mut name = String::new();

prientln!("{}") prientln!("{:?}") prientln!("{:#?}") :#?结构体格式化输出

impl 块（implementation 的缩写）
关联函数：允许在 impl 块中定义 不 以 self 作为参数的函数,
使用结构体名和 :: 语法来调用这个关联函数.例如 String::from

vec 向量
enum 枚举
str 字符在，String 是 str 的标准库
HashMap<K, V> 键值对结果（对象）
String::new，new 是 String 类型的一个 关联函数（associated function），一些语言中把它称为 静态方法（static method）
use std::io 表示从标准库中引入了io功能，也可以直接使用 std::io::stdin
