use std::io;
use std::process::{Command, ExitStatus};
use sqlx::{Connection, MySqlConnection};

fn check_mysql_version() -> Result<ExitStatus, io::Error> {
    let mut cmd = Command::new("mysql");
    cmd.arg("--version");
    let mut output = cmd.output()?;

    Ok(output.status)

}

async fn mysql_connection() -> MySqlConnection {
    MySqlConnection::connect("mysql://wiseye:wiseye@localhost/database")
        .await?
}