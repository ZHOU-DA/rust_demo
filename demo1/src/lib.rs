use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text_content: String = fs::read_to_string(config.file_name)?;

    println!("Hello, world! {}", text_content);
    Ok(())
}

pub struct Config {
    _query: String,
    file_name: String,
}

impl Config {
    pub fn new<'a>(args: &'a [String]) -> Result<Config, &'a str> {
        if args.len() < 3 {
            return Err("参数不够");
        }
        let _query = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config { _query, file_name })
    }
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

// start 标识 init() 在 WASM 加载时自动执行
#[wasm_bindgen(start)]
pub fn init() -> Result<(), JsValue> {
    // 使用 web_sys 的 window 全局对象
    let window = web_sys::window().expect("不存在全局 window 对象");
    let document = window.document().expect("需要在 window 上存在 document");
    let body = document.body().expect("document 中需要存在一个 body");

    // 生成 dom 元素
    let input = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    let btn = document.create_element("button")?;
    btn.set_text_content(Some(&"点击计算斐波那契数"));
    let out = document.create_element("h3")?;
    let input = Rc::new(input); // 为了不违背“一个变量只能有一个所有者”的规则，需要使用 Rc 包裹 input 元素，方便在闭包中拿到并使用它的值
    let out = Rc::new(RefCell::new(out)); // 因为需要改变 out 元素的 textContent，需要使用 RefCell 包裹方便去在闭包中把它当做可变变量来改变它
    {
        let out = out.clone(); // 复制一份智能指针
        let input = input.clone();
        // 使用到 wasm_bindgen::closure::Closure，它的作用是打通 Rust 中的闭包和 JS 中的闭包
        let closure = Closure::<dyn Fn()>::new(move || {
            let val = input.value();
            let num = val.parse::<u32>().unwrap();
            
            let res = fib(num);
            out.borrow_mut()
                .set_text_content(Some(res.to_string().as_str())); // 在这里使用 borrow_mut 把 out 当做可变变量获取出来，并设置 textContent
        });

        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?; // 挂载事件监听器，将闭包函数先转换为 JS 值，再跳过类型判断，再作为回调函数传给 btn
        closure.forget(); // 释放 Rust 对这片堆内存的管理，交给 JS 的 GC 去回收
    }

    body.append_child(&input)?;
    body.append_child(&btn)?;
    body.append_child(&out.borrow())?; // 挂载 DOM 元素节点
    Ok(())
}
