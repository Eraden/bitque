use actix;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

#[derive(Debug)]
pub enum AmazonError {
    UploadFailed,
}

pub struct AmazonExecutor;

impl Default for AmazonExecutor {
    fn default() -> Self {
        Self {}
    }
}

impl actix::Actor for AmazonExecutor {
    type Context = actix::SyncContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "Result<String, AmazonError>")]
pub struct S3PutObject {
    pub source: tokio::sync::broadcast::Receiver<bytes::Bytes>,
    pub file_name: String,
}

impl actix::Handler<S3PutObject> for AmazonExecutor {
    type Result = Result<String, AmazonError>;

    fn handle(&mut self, msg: S3PutObject, _ctx: &mut Self::Context) -> Self::Result {
        let S3PutObject {
            mut source,
            file_name,
        } = msg;
        jirs_config::amazon::config().set_variables();

        tokio::runtime::Runtime::new()
            .expect("Failed to start amazon agent")
            .block_on(async {
                let s3 = jirs_config::amazon::config();
                log::debug!("{:?}", s3);

                // TODO: Unable to upload as stream because there is no size_hint
                // use futures::stream::*;
                // let stream = source
                //     .into_stream()
                //     .map_err(|_e| std::io::Error::from_raw_os_error(1));

                let mut v: Vec<u8> = vec![];
                use bytes::Buf;
                while let Ok(b) = source.recv().await {
                    v.extend_from_slice(b.bytes())
                }

                let client = S3Client::new(s3.region());
                let put_object = PutObjectRequest {
                    bucket: s3.bucket.clone(),
                    key: file_name.clone(),
                    // body: Some(rusoto_signature::ByteStream::new(stream)),
                    body: Some(v.into()),
                    ..Default::default()
                };
                let id = match client.put_object(put_object).await {
                    Ok(obj) => obj,
                    Err(e) => {
                        log::error!("{}", e);
                        return Err(AmazonError::UploadFailed);
                    }
                };
                log::debug!("{:?}", id);
                Ok(aws_s3_url(file_name.as_str()))
            })
    }
}

fn aws_s3_url(key: &str) -> String {
    let config = jirs_config::amazon::config();
    format!(
        "https://{bucket}.s3.{region}.amazonaws.com/{key}",
        bucket = config.bucket,
        region = config.region_name,
        key = key
    )
}
