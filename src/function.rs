pub mod function_test {
    fn test() {
        println!("function test");
        trigger_behind();
        // 将方法传入作为回调函数，此为函数指针的方式，fn被称为函数指针
        // fn是一个类型而不是一个trait
        // 直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
        // 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce）
        // 所以总是可以在调用期望闭包的函数时传递函数指针作为参数
        callback(println_name);
        // 另一种方式。通过闭包的方式
        // 这种方式为闭包
        // 闭包表现为 trait，这意味着不能直接返回闭包
        let temp_println_name = |name: &str| {
            println!("name in closure: {}", name);
            true
        };
        // @Doubt ? 后面回来再细看，为什么temp_println_name的所有权被移到callback内部使用完毕后没有被drop
        // 看来所有权与Copy有很大的关系
        callback(temp_println_name);
        temp_println_name("111");
        // Copy trait是关键！可以用在类似整型这样的存储在栈上的类型上
        // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
        // 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
        // 也就是实现了Copy trait的类型，被drop时没有任何效果
        // 任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy
        // 比如数值类型（整数&浮点）、bool、char、数组、元组（必须所有属性都实现了Copy此元组才会实现Copy）、fn
        // 官方解释
        // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的
        let a = 5;
        let b = a;
        let c = String::from("123");
        // i32实现了Copy，Drop是没有效果的
        drop(a);
        // String没有实现Copy，所以可以手动被Drop掉
        drop(c);
        // 此时a仍可以使用，因为i32实现了Copy trait。
        println!("a: {}", a);
    }
    fn trigger_behind() {
        println!("trigger behind");
    }
    fn println_name(name: &str) -> bool {
        println!("name: {}", name);
        true
    }
    // 接收方法作为参数，与JS的回调函数一个用法和含义
    // Rust中，在方法调用时，会将方法传参参数压入栈中，执行方法完成后，会将参数出栈。遵循内存管理规则
    // 当你的代码调用一个函数时，传递给函数的参数（包括可能指向堆上数据的指针和函数的局部变量）依次被压入栈中
    // 当函数调用结束时，这些值将被从栈中按照相反的顺序依次移除
    fn callback(cb: fn(&str) -> bool) {
        let name = "function call in callback";
        let trigger_result = cb(name);
        println!("trigger_result: {}", trigger_result);
    }
    pub fn run() {
        test()
    }
}