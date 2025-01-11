pub mod number_test {
    use num::{Complex};
    use num::traits::real::Real;

    fn int_test() {
        // 整数默认类型是i32，因为i32支持的位数够多，且性能是最好的
        let i = 1;
        // isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32 位的
        // isize 或 usize 主要作为某些集合的索引
        let index = "1".parse::<usize>().unwrap();
        let mut a: u8 = 255;
        // 如果用release模式运行时，不会检测溢出，会遵循补码循环溢出的规则处理
        // 非release模式会无法通过编译
        a += 1;
        // release模式下此时a为0，因为超精度，会循环补码
        println!("{}", a);
        // 可以通过as进行类型转换，与TS类似
        let b = a as i8;
    }
    fn float_test() {
        // 浮点数默认类型是f64，在现代的 CPU 中它的速度与 f32 几乎相同，但精度更高
        // f32 类型是单精度浮点型，f64 为双精度
        let i = 3.2;
        // 任何程序语言处理浮点数都会有点问题
        // 1. 浮点数往往是想要数字的近似表达。浮点数是基于二进制实现的，0.1在十进制是确定稳定的表达式，但是在二进制并不是
        //    如果想要表达完全精准的真实数字，只有使用无限精度的浮点数才行
        // 2. 浮点数在某些特性上是反直觉的。浮点数是一定不要直接进行比较的！！因为浮点数在二进制并没有准确稳定的表达式，所以比较结果是不准的
        //    浮点是比较运算实现的是 std::cmp::PartialEq 特征(类似其他语言的接口)，并没有实现 std::cmp::Eq 特征
        // assert_eq!(0.1 + 0.2, 0.3);
        // left: 0.30000000000000004
        // right: 0.3
        // 这就是浮点数的直观表达
        // 正确做法如下
        let left1 = 0.1_f64;
        let left2 = 0.2_f64;
        let result = 0.3_f64;
        assert_eq!((left1 + left2 - result).abs() < 0.001, true);
    }
    fn nan() {
        // 数值类型有时会是NaN，这个在JS中常见，用法类似，不过多描述
        let a = -42.4_f32.sqrt();
        if a.is_nan() {
            // 此时a为NaN
        }
    }
    fn number_operator() {
        // 可以通过后缀方式指定类型
        let age = 25i8;
        let age = 25_i8;
        // 可以通过下划线增加可读性
        let rich = 1_000_000_u64;
        let array = [42.0, 42.0f32, 42.0_f32];
        // 打印数组中第二个值，并控制小数位为2位
        println!("{:.2}", array[1])
    }
    fn l_operator() {
        // 位运算符，非常用，但是要了解
        // & 位表示与	  相同位置均为1时则为1，否则为0
        // | 位表示或	  相同位置只要有1时则为1，否则为0
        // ^ 异表示或	  相同位置不相同则为1，相同则为0
        // ! 位表示非   把位中的0和1相互取反，即0置为1，1置为0
        // << 表示左移	所有位向左移动指定位数，右位补0
        // >> 表示右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）
        let a = 2u8; // 也可以表示为0b_000_010
        let b = 3u8; // 也可以表示为0b_000_011
        // {:08b}：左高右低输出二进制01，不足8位则高位补0
        println!("a value is {:08b}", a);
        println!("b value is {:08b}", b);
        println!("(a & b) value is  {:08b}", a & b);
        println!("(a | b) value is  {:08b}", a | b);
        println!("(a ^ b) value is  {:08b}", a ^ b);
        println!("(!b) value is     {:08b}", !b);
        println!("(a << b) value is {:08b}", a << b);
        println!("(a >> b) value is {:08b}", a >> b);
        let mut a = a;
        // 除了!运算符，都以用=来赋值，因为!=用来做相等判断
        a <<= b;
        print!("(a << b) value is  {:08b}", a);
    }
    fn range() {
        // 序列只能用于数字或字符类型，因为他们是连续的，非连续的无法用序列表示
        // 如下表示左闭右开[1,5)
        for i in 1..5{
            println!("{}", i);
        }
        // 如下表示闭区间[1,5]
        for i in 1..=5{
            println!("{}", i);
        }
        for i in 'a'..'p' {
            println!("{}", i);
        }
    }
    fn complex() {
        let a = Complex { re: 2.1, im: -1.2 };
        let b = Complex::new(11.1, 25.7);
        let result = a + b;
        println!("{} + {}", result.re, result.im);
        println!("{}", 0.1 + 0.2);
    }
    pub fn run() {
        range();
    }
}
