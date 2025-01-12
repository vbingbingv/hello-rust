pub mod function_test {
    fn test() {
        println!("function test");
        trigger_behind();
        // 将方法传入作为回调函数，方法不涉及所有权的概念，没有&表示地址引用这一说
        callback(println_name);
        // 另一种方式。通过闭包的形式传入
        let temp_println_name = |name: &str| {
            println!("name in closure: {}", name);
            true
        };
        // @Doubt ? 后面回来再细看，为什么temp_println_name的所有权被移到callback内部使用完毕后没有被drop
        // 看来所有权与Copy有很大的关系
        callback(temp_println_name);
    }
    fn trigger_behind() {
        println!("trigger behind");
    }
    fn println_name(name: &str) -> bool {
        println!("name: {}", name);
        true
    }
    // 接收方法作为参数，与JS的回调函数一个用法和含义
    fn callback(cb: fn(&str) -> bool) {
        let name = "function call in callback";
        let trigger_result = cb(name);
        println!("trigger_result: {}", trigger_result);
    }
    pub fn run() {
        test()
    }
}