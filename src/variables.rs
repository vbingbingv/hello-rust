pub mod variables_test {
    fn mut_define() {
        let mut x = 5;
        if x == 5 {
            x = 6
        }
        println!("{}", x)
    }

    struct Position {
        x: i8,
        y: i8,
    }

    fn destruct() {
        let (name, age) = ("Leng", 24);
        let [one, two, three] = [1,2,3];
        let Position { x, y } = Position {
            x: 5,
            y: 6,
        };
        // ..为省略，类似JS中的...
        let Position { x: _x, .. } = Position {
            x: 5,
            y: 6,
        };
        println!("{} is {}", name, age);
        println!("count: {} {} {}", one, two, three);
        println!("Position is {}, {}", x, y);
    }

    fn const1() {
        const NAME: &str = "Leng FangBing";
        const MAX_COUNTS: i32 = 100_000;
    }
    fn shadow() {
        // 当前block中，在内存中生成一份age，内存引用的值是24
        let age = 24;
        age.is_positive();
        // 在内存中再生成一份age，内存引用的值为24.5
        // 因为内存中已经有同样的命名，所以顶掉了之前的age的内存地址和对应的值
        // 两个age除了名称一样，其余都不一样，内存、值都不同
        // 因为要涉及内存重新分配，性能要差于mut，mut只是修改内存引用的值，不涉及新的内存分配
        let age = 24.5;
        age.abs();
        {
            // 当前block中age是25
            let age = 25;
            println!("The age is {}", age);
        }
        println!("The age is {}", age);
    }
    pub fn run() {
        let a = "hello world";
    }
}