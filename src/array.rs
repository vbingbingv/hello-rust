pub mod array_test {
    use std::str::FromStr;
    // 从给定的arr中获取到对应索引的值
    // 关于生命周期使用后面再说。这里要加生命周期是因为
    // @Doubt ? 下面这段解释不确定对不对，后面学习后再回头查看
    // 该方法本身只是返回该数组的某项引用，并不会创造新的引用，也不会夺取该对象的所有权
    // 如果传入的不是该对象的引用而是该对象，那么该对象的所有权就发生了转移
    // 该对象的所有权转移到此方法内部
    // 此时在 *1 处的代码的返回完成后，该对象就会被释放，调用方也就拿不到此值
    // 所以此处必须只能获取到引用，不修改此对象的所有权
    fn get_array_value<'a, Value>(arr: &'a [Value;3], index: &str) -> Option<&'a Value> {
        let number;
        // 第一种方式，用str的parse，转成设定的数值类型。需要注意对Result的处理
        match index.parse::<usize>() {
            Ok(n) => number = n,
            Err(_) => {
                println!("Not a number");
            },
        }
        // 第二种方式，同样使用str的parse，不过可以更方便采用unwrap来进行处理，注意不能直接使用unwrap
        let number = index.parse::<usize>().unwrap_or_else(|err| {
            println!("parse error: {}", err);
            0
        });
        // 第三种方式，采用str的fromStr转换，同样采用unwrap来进行处理，不能直接使用unwrap
        let number = usize::from_str(index).unwrap_or_else(|err| {
            println!("parse error: {}", err);
            0
        });
        if number > 0 {
            let arr_value = arr.get(number);
            if arr_value.is_some() {
                // @*1
                return arr_value
            }
            println!("arr {} value is Err", number)
        }
        None
    }
    fn test_array_index() {
        let arr: [u32; 3] = [1, 2, 3];
        let value = get_array_value(&arr, "1");
        if value.is_some() {
            println!("result is {:?}", value.unwrap());
        }
    }
    pub fn run() {
        test_array_index()
    }
}
