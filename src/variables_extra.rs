pub mod variables_extra_test {
    use std::collections::HashMap;

    fn char() {
        // Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符
        // 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
        let a = 'a';
        let b = '😂';
        let c = '√';
        let d = '对';
        println!("字符 {} 占用了{}字节的内存大小", a, size_of_val(&a));
        println!("字符 {} 占用了{}字节的内存大小", b, size_of_val(&b));
        println!("字符 {} 占用了{}字节的内存大小", c, size_of_val(&c));
        println!("字符 {} 占用了{}字节的内存大小", d, size_of_val(&d));
    }
    fn boolean() {
        let a = true;
        // 布尔值占用内存的大小为 1 个字节
        println!("布尔 {} 占用了{}字节的内存大小", a, size_of_val(&a));
    }
    fn unit() {
        // 单元类型也就是()，形似元组但并不是。可以理解为无意义的类型，类似TS的never
        // 比如main函数，返回的就是()
        // 没有返回值的方法被称为 发散函数(diverge function) 。
        let a: HashMap<u32, ()> = HashMap::new();
        // 比如这种定义方式，即为我们只关心这个Map的key，值为单元类型，仅用来占位，不会占用任何内存
    }
    pub fn run() {
        boolean();
    }
}