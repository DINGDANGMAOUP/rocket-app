use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::{async_trait, Error};
use rbs::Value;

/// Postgres insert sql returning id Intercept
#[derive(Debug)]
pub struct ReturningIdPlugin {}

#[async_trait]
impl Intercept for ReturningIdPlugin {
    async fn before(
        &self,
        _task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if sql.contains("insert into") {
            let new_sql = format!("{} {}", sql, "returning id");
            let new_args = args.clone();
            match result {
                ResultType::Exec(exec_r) => {
                    let id = rb.query(&new_sql, new_args).await?;
                    let id: String = rbatis::decode(id)?;
                    let mut exec = ExecResult::default();
                    exec.last_insert_id = id.into();
                    *exec_r = Ok(exec);
                    Ok(None)
                }
                ResultType::Query(_) => Ok(Some(true)),
            }
        } else {
            Ok(Some(true))
        }
    }
}
