use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;
use rbs::Value;
#[derive(Debug)]
pub struct LogicDelIntercept;
#[async_trait]
impl Intercept for LogicDelIntercept {
    async fn before(
        &self,
        task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        println!("before task_id:{}", task_id);
        println!("before sql:{}", sql);
        println!("before args:{:?}", args);
        println!("before result:{:?}", result);
        //当前sql为delete，则修改为update 语句 del_flag = 1
        if sql.contains("delete") {
            *sql = sql.replace("delete", "update");
            sql.push_str(" set del_flag = 1");
        } else if sql.contains("select") {
            //当前sql为select，添加过滤条件 del_flag = 0
            if sql.contains("where") {
                sql.push_str(" and del_flag = 0");
            } else {
                sql.push_str(" where del_flag = 0");
            }
        }
        Ok(Some(true))
    }
    async fn after(
        &self,
        task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        println!("after task_id:{}", task_id);
        println!("after sql:{}", sql);
        println!("after args:{:?}", args);
        println!("after result:{:?}", result);
        Ok(Some(true))
    }
}
