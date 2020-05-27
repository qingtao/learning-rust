use std::fmt;

fn main() {
    // 2. 原生类型
    // 变量明确声明类型
    let _logical: bool = true;

    let _a_float: f64 = 3.0;
    let _an_integer = 5i32;

    // 默认类型
    let _defualt_float = 3.0;
    let _default_integer = 7;

    // 推断为i64
    let mut _inferred_type = 12;
    _inferred_type = 4294967296i64;

    // 可变变量值可以改变
    let mut _mutable = 12;
    _mutable = 21;

    // 变量类型不可改变
    // _mutable = true;

    let _mutable = true;

    // 2.1. 字面量和运算符

    println!("1+2={}", 1u32 + 2);
    println!("1+2={}", 1i32 + 2);

    // 1u32-2溢出
    // println!("1-2={}", 1u32 - 2);
    println!("1-2={}", 1i32 - 2);

    // 逻辑运算
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101u32);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 下划线提高数字可读性
    println!("One million is written as {}", 1_000_000u32);

    // 2.2 元组
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reverse (1,true) is {:?}", reverse(pair));

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8, -2i16));
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 解构元组
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix.0);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}

// 元组当作函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (i, b) = pair;
    (b, i)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// 转换右上角和左下角的元素
fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
