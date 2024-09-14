// #![allow(unused_variables)] //允许未使用的变量
// #![allow(dead_code)] //允许未使用的代码
#[macro_use]
extern crate rbatis;

#[macro_use]
pub mod common;
pub mod config;
pub mod controller;
pub mod domain;
pub mod mapper;
pub mod pojo;
pub mod service;
pub mod  middleware;
pub mod response;
pub mod error;
