use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;
use rbs::Value;
#[derive(Debug)]
pub struct LogicDelIntercept;
#[async_trait]
impl Intercept  for LogicDelIntercept {
    async fn before(
        &self,
        task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        println!("before task_id:{}",task_id);
        println!("before sql:{}",sql);
        println!("before args:{:?}",args);
        println!("before result:{:?}",result);
        Ok(Some(true))
    }
    async fn after(&self, task_id: i64, _rb: &dyn Executor, sql: &mut String, args: &mut Vec<Value>, result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>) -> Result<Option<bool>, Error> {
        println!("after task_id:{}",task_id);
        println!("after sql:{}",sql);
        println!("after args:{:?}",args);
        println!("after result:{:?}",result);
        Ok(Some(true))
    }
}