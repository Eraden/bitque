use std::io::Write;
use std::path::PathBuf;

use actix::SyncContext;
use actix_files::{self, Files};
use jirs_config::fs::Configuration;

#[derive(Debug)]
pub enum FsError {
    CopyFailed,
    UnableToRemove,
    CreateFile,
    WriteFile,
}

pub struct FileSystemExecutor {
    config: Configuration,
}

impl FileSystemExecutor {
    pub fn client_path(&self) -> &str {
        self.config.client_path.as_str()
    }

    pub fn tmp_path(&self) -> &str {
        self.config.tmp_path.as_str()
    }
}

impl Default for FileSystemExecutor {
    fn default() -> Self {
        Self {
            config: Configuration::read(),
        }
    }
}

impl actix::Actor for FileSystemExecutor {
    type Context = SyncContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "Result<usize, FsError>")]
pub struct CreateFile {
    pub source: tokio::sync::broadcast::Receiver<common::bytes::Bytes>,
    pub file_name: String,
}

impl actix::Handler<CreateFile> for FileSystemExecutor {
    type Result = Result<usize, FsError>;

    fn handle(&mut self, msg: CreateFile, _ctx: &mut Self::Context) -> Self::Result {
        let Configuration { store_path, .. } = &self.config;
        let CreateFile {
            mut source,
            file_name,
        } = msg;

        let target = PathBuf::new().join(store_path).join(file_name);
        let _ = std::fs::remove_file(&target);
        let mut f = std::fs::File::create(target).map_err(|_| FsError::CreateFile)?;

        let count = futures::executor::block_on(async move {
            let mut mem = 0;
            while let Ok(b) = source.recv().await {
                mem += f.write(&b).unwrap_or_default();
            }
            mem
        });
        Ok(count)
    }
}

pub fn service() -> Files {
    let Configuration {
        store_path,
        client_path,
        ..
    } = Configuration::read();
    Files::new(client_path.as_str(), store_path.as_str())
}
