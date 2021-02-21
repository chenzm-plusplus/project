//! 内存模块
//! 
//! 
//! 

// 因为模块内包含许多基础设施类别，实现了许多以后可能会用到的函数，
// 所以在模块范围内不提示「未使用的函数」等警告
#![allow(dead_code)]

pub mod config;
mod heap;

/// 初始化memory相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    heap::init();
    println!("mod memory initialized");
}