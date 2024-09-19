// #![allow(unused_variables)] //允许未使用的变量
// #![allow(dead_code)] //允许未使用的代码
#[macro_use]
extern crate rbatis;
extern crate serde_qs as qs;
extern crate serde_with;
#[macro_use]
pub mod common;
pub mod config;
pub mod controller;
pub mod domain;
pub mod error;
pub mod mapper;
pub mod middleware;
pub mod pojo;
pub mod response;
pub mod service;
