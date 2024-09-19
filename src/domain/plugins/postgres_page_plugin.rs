use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;
use rbs::Value;
use regex::Regex;

//postgres 分页插件
#[derive(Debug)]
pub struct PostgresPagePlugin;
#[async_trait]
impl Intercept for PostgresPagePlugin {
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        println!("PostgresPagePlugin => sql: {}", sql);
        let new_sql = convert_limit_to_offset(sql);
        if new_sql.is_some() {
            *sql = new_sql.unwrap();
            println!("PostgresPagePlugin => sql: {}", sql);
        }
        Ok(Some(true))
    }
}

fn convert_limit_to_offset(query: &str) -> Option<String> {
    // 不区分大小写的正则表达式，匹配LIMIT及其后的数字
    let re = Regex::new(r"(?i)\blimit\s+(\d+)(?:,\s*(\d+))?").unwrap();

    // 查找匹配项
    if let Some(caps) = re.captures(query) {
        let offset_num = caps.get(1).map_or("", |m| m.as_str());
        let limit_num = caps.get(2).map_or("0", |m| m.as_str());

        // 构建新的查询语句，使用正确的OFFSET和LIMIT格式
        let new_query = re
            .replace(query, |_caps: &regex::Captures| {
                format!("OFFSET {} LIMIT {}", offset_num, limit_num)
            })
            .into_owned();

        Some(new_query)
    } else {
        // 如果没有找到合法的LIMIT子句，则不进行任何修改并返回None
        None
    }
}
