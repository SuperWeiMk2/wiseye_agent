use std::io::Error;
use std::str::FromStr;

use sqlx::{FromRow, MySqlConnection};
use sqlx::mysql::MySqlDatabaseError;

struct MysqlInfo {
    max_connections: u32,           // mysql 的最大连接数
    wait_timeout: u32,              // 客户端闲置多少秒后断开连接。
    interactive_timeout: u32,       // 对于交互式客户端（如终端）的等待超时时间。
    innodb_buffer_pool_size: u32,   // InnoDB存储引擎的缓冲池大小
    character_set_server: String,   // 服务器默认的字符集。
    collation_server: String,       // 服务器默认的排序规则。
    innodb_flush_log_at_trx_commit: String, // 控制事务日志刷写策略。
    slow_query_log: String, // 是否启用慢查询日志。
    long_query_time: u32,   // 定义一个查询被认为是“慢”的时间阈值（秒）。
    tmp_table_size: u32,    // 内存中临时表的最大大小。
    max_heap_table_size: u32,   // 用户创建的HEAP表的最大大小。
}

impl MysqlInfo {
    pub async fn fetch_info(mut conn: MySqlConnection) -> Result<Self, Error> {
        let max_connections = Self::get_variable_value(&mut conn, "max_connections").await?;
        let wait_timeout = Self::get_variable_value(&mut conn, "wait_timeout").await?;
        let interactive_timeout = Self::get_variable_value(&mut conn, "interactive_timeout").await?;

        Ok(Self {
            max_connections,
            wait_timeout,
            interactive_timeout,
            innodb_buffer_pool_size: 0,
            character_set_server: "".to_string(),
            collation_server: "".to_string(),
            innodb_flush_log_at_trx_commit: "".to_string(),
            slow_query_log: "".to_string(),
            long_query_time: 0,
            tmp_table_size: 0,
            max_heap_table_size: 0,
        })
    }

    async fn get_variable_value<T>(conn: &mut MySqlConnection, variable_name: &str) -> Result<T, Error>
        where
            T: FromStr,
            <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        // 定义一个结构体来匹配查询结果的每一行
        #[derive(Debug, FromRow)]
        struct Variable {
            variable_name: String,
            value: String,
        }

        // 执行查询以获取指定变量的值
        let result: Vec<Variable> = sqlx::query_as("SHOW VARIABLES LIKE $1")
            .bind(variable_name)
            .fetch_all(conn)
            .await?;

        // 解析结果，获取 Value 列的值并转换为指定类型
        let value = result.into_iter()
            .find(|variable| variable.variable_name == variable_name)
            .and_then(|variable| variable.value.parse::<T>().ok())
            .ok_or_else(|| MySqlDatabaseError::new(format!("{:?} variable not found", variable_name)))?;

        Ok(value)
    }
}