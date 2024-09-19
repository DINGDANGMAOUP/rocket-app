use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;
use rbs::Value;
///逻辑删除插件（待完善）
#[derive(Debug)]
pub struct LogicDelPlugin;
#[async_trait]
impl Intercept for LogicDelPlugin {
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        //当前sql为delete，则修改为update 语句 del_flag = 1,select时 添加del_flag=0条件  并且解决与offset ? limit ? 条件顺序语法错误
        
        if sql.to_lowercase().starts_with("delete") {
            *sql = sql.replace("delete", "update").replace(" set ", " set del_flag = 1 ");
        } else if sql.to_lowercase().contains("select") && !sql.to_lowercase().contains("del_flag = 0") {
            if sql.contains("where") {
                *sql = format!("{} and del_flag = 0", sql);
            } else {
                *sql = format!("{} where del_flag = 0", sql);
            }
        }

        
        println!("LogicDelPlugin => sql: {}",sql);
        Ok(Some(true))
    }
    // async fn after(
    //     &self,
    //     _task_id: i64,
    //     _rb: &dyn Executor,
    //     _sql: &mut String,
    //     _args: &mut Vec<Value>,
    //     _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    // ) -> Result<Option<bool>, Error> {
    //     Ok(Some(true))
    // }
}
