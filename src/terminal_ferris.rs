pub mod terminal_ferris_test {
    use std::io::{stdout, BufWriter};
    use ferris_says::say;
    const HELLO_WORLD: &'static str = "hello man";

    pub fn run() {
        let stdout = stdout();
        let message = String::from(HELLO_WORLD);
        let width = message.chars().count();
        let mut writter = BufWriter::new(stdout.lock());
        say(&message, width, &mut writter).unwrap();
    }
}
