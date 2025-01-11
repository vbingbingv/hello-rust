pub mod hello_world_test {
    pub fn run() {
        let hello_world_germany = "Grüß Gott!";
        let hello_world_china = "你好，世界！";
        let hello_world_english = "hello world!";
        let regions = [hello_world_china, hello_world_english, hello_world_germany];
        for region in regions.iter() {
            // !表示调用的是宏，不是函数，函数与宏还是有一定不同的意义的
            println!("{}", region);
        }
    }
}