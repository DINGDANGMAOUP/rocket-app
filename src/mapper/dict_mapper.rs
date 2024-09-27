use crate::common::pojo::vo::dict_data::DictVo;
use rbatis::executor::Executor;

htmlsql!(select_dict_data(rb: &dyn Executor)->Result<Vec<DictVo>, rbatis::Error> =>"sql/html/dict.html");
