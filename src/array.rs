pub mod array_test {
    use std::str::FromStr;
    // 从给定的arr中获取到对应索引的值
    // 关于生命周期使用后面再说。这里要加生命周期是因为
    // @Doubt ? 下面这段解释不确定对不对，后面学习后再回头查看
    // 结论
    // 数组实现了Copy trait，所以这里理论上是可以传入arr本身的，那么内部使用的就会是Copy后的arr
    // 但是返回中返回的是Copy后的arr的某项引用，此Copy后的arr在方法结束后就在栈中移除
    // 此时在 *1 处的代码的返回完成后，该Copy后的arr就会被释放
    // 所以此处必须只能获取到引用
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
