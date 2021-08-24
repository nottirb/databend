//  Copyright 2020 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;

use async_compat::Compat;
use async_compat::CompatExt;
use common_exception::ErrorCode;
use common_exception::Result;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::blob_accessor::Bytes;
use crate::blob_accessor::DataAccessor;

pub struct Local {
    root: PathBuf,
}

impl Local {
    pub fn new(root: &str) -> Local {
        Local {
            root: PathBuf::from(root),
        }
    }
}

impl Local {
    fn prefix_with_root(&self, path: &str) -> Result<PathBuf> {
        let path = self.root.join(path).canonicalize()?;
        if path.starts_with(&self.root) {
            Ok(path)
        } else {
            // TODO customize error code
            Err(ErrorCode::from(Error::new(
                ErrorKind::Other,
                format!("please dont play with me, malicious path {:?}", path),
            )))
        }
    }
}

#[async_trait::async_trait]
impl DataAccessor for Local {
    type InputStream = Compat<File>;

    async fn get_input_stream(
        &self,
        path: &str,
        _stream_len: Option<u64>,
    ) -> Result<Self::InputStream> {
        let path = self.prefix_with_root(path)?;
        Ok(tokio::fs::File::open(path).await?.compat())
    }

    async fn get(&self, path: &str) -> Result<Bytes> {
        let path = self.prefix_with_root(path)?;
        let mut file = tokio::fs::File::open(path).await?;
        let mut contents = vec![];
        let _ = file.read_to_end(&mut contents).await?;
        Ok(contents)
    }

    async fn put(&self, path: &str, content: Vec<u8>) -> common_exception::Result<()> {
        let path = self.prefix_with_root(path)?;
        let parent = path
            .parent()
            .ok_or_else(|| ErrorCode::UnknownException(""))?; // TODO customized error code
        tokio::fs::create_dir_all(parent).await?;
        let mut new_file = tokio::fs::File::create(path).await?;
        new_file.write_all(&content).await?;
        Ok(())
    }
}