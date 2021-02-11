pub mod types;
pub mod operation;
pub mod control;
pub mod struct_type;
pub mod impl_trait;
pub mod generics;
pub mod vec;
pub mod string;
pub mod hashmap;
pub mod errors_handle;
pub mod crate_mod;
pub mod inherit;
pub mod interface;
pub mod config;
pub mod read_config;
pub mod config2;
pub mod config3;
pub mod person;
pub mod clone;
pub mod loopx;
pub mod foobar;
pub mod defer;
pub mod dynx;

// ================== 集成测试 ===================
/*
集成测试用例和源码位于不同的目录，因而源码中的模块和函数，对于集成测试来说完全是外部的，因此，只能调用一部分库暴露的公共API。Cargo 约定集成测试的代码位于项目根路径下的 tests 目录中，Cargo 会将 tests 中的每一个文件当做一个 crate 来编译。
 */
pub fn plus(x: i32, y: i32) -> i32 {
  x + y
}

