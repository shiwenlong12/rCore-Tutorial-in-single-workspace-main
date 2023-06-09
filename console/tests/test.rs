
use rcore_console::{Console,init_console, test_log, set_log_level};

struct Console1;

/// 为 `Console` 实现 `console::Console` trait。
impl Console for Console1 {

    fn put_char(&self, _c: u8) {
        //将作为参数的ASCII码放入数组中
        let buffer = [_c];
        //调用标准库中std::str::from_utf8函数将含有ASCII码的数组转化成含有一个字符的字符串
        let s = std::str::from_utf8(&buffer[..]).expect("invalid utf-8 sequence");
        print!("{s}");
    }

    /// 向控制台放置一个字符串。
    ///
    /// 如果使用了锁，覆盖这个实现以免反复获取和释放锁。
    #[inline]
    fn put_str(&self, s: &str) {
        for c in s.bytes() {
            self.put_char(c);
        }
    }
}

#[test]
fn test_println() {
    init_console(&Console1);

    (&Console1).put_char(70);
    (&Console1).put_char(10);
    print!("_");
    let buffer = [95];
    let s = std::str::from_utf8(&buffer[..]).expect("invalid utf-8 sequence");
    print!("{s}");

    (&Console1).put_char(96);
    (&Console1).put_str("abc\n");

    // 设置日志级别
    set_log_level(option_env!("LOG"));
    //测试各种打印
    test_log();
    rcore_console::print!("hell0 ");
    rcore_console::println!("world");

}