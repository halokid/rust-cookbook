/*
全局变量的方式获取配置
static全局变量只支持有限的类型
 */

pub static mut NUM: i32 = 99;


struct Config2 {
  pub name: String,
  pub age: u16,
}

// 定义一个配置并且初始化
// error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
// pub static mut CFG: Config2 = Config2 { name: "x".to_string(), age: 0 };

/*
impl Config2 {

  // 初始化配置到项目真实需要的值
  pub fn init() {
    unsafe {
      CFG.name = "halokid".to_string();
      CFG.age = 18;
    }
  }
}
 */

// 初始化配置到项目真实需要的值
// pub fn init() {
//   unsafe {
//     CFG.name = "halokid".to_string();
//     CFG.age = 18;
//   }
// }




