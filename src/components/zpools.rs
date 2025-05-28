use async_trait::async_trait;
use indexmap::IndexMap;
use std::process::Stdio;
use thiserror::Error;
use tokio::process::Command;
use termion::{color, style};

use crate::component::{Component, Constraints, PrepareReturn};
use crate::config::global_config::GlobalConfig;
use crate::default_prepare;

#[derive(Clone)]
pub struct Zpools;

#[async_trait]
impl Component for Zpools {
    fn prepare(self: Box<Self>, _global_config: &GlobalConfig) -> PrepareReturn {
        (Box::new(self.clone()), Some(Constraints { min_width: None }))
    }

    async fn print(self: Box<Self>, _global_config: &GlobalConfig, _width: Option<usize>) {
        match get_zpool_status().await {
            Ok(output) => {
                println!("ZFS Pools Status:");
                println!("{}", output);
            }
            Err(err) => println!("{}ZFS Error: {}{}", color::Fg(color::Red), err, style::Reset),
        }
        println!();
    }
}

default_prepare!();

#[derive(Debug, Error)]
pub enum ZpoolError {
    #[error("Failed to execute zpool command: {0}")]
    CommandError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Calls `zpool status` and returns the raw output
async fn get_zpool_status() -> Result<String, ZpoolError> {
    let output = Command::new("zpool")
        .arg("status")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Err(ZpoolError::CommandError(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
