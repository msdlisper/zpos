#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

fn clear_bss() {
    extern "C" {
        // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
        // 想到用FFI的方式来引入，根据官方文档，在extern "C"块中似乎只能引用ABI接口，也就是一个函数签名，需要有函数名、参数列表和返回值。好像不能像C语言那样extern int c;这样做。引入之后sbss和ebss都变成函数了，所以有as usize将其转换成函数入口地址也就是符号自身的地址。
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}
