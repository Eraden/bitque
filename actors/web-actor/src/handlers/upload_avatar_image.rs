#[cfg(feature = "local-storage")]
use filesystem_actor::FileSystemExecutor;
use {
    actix::Addr,
    actix_multipart::Field,
    actix_web::{http::header::ContentDisposition, web::Data, Error},
    futures::{StreamExt, TryStreamExt},
    jirs_data::UserId,
    rusoto_core::ByteStream,
    tokio::sync::broadcast::{Receiver, Sender},
};
#[cfg(feature = "aws-s3")]
use {
    jirs_config::web::AmazonS3Storage,
    rusoto_s3::{PutObjectRequest, S3Client, S3},
};

#[cfg(all(feature = "local-storage", feature = "aws-s3"))]
pub(crate) async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    fs: Data<Addr<FileSystemExecutor>>,
) -> Result<String, Error> {
    let filename = disposition.get_filename().unwrap();
    let system_file_name = format!("{}-{}", user_id, filename);

    let (sender, receiver) = tokio::sync::broadcast::channel(4);

    let fs_fut = tokio::task::spawn(local_storage_write(
        system_file_name.clone(),
        fs.clone(),
        user_id,
        sender.subscribe(),
    ));

    // Upload to AWS S3
    let aws_fut = tokio::task::spawn(aws_s3(system_file_name, receiver));

    read_form_data(&mut field, sender).await;

    let mut new_link = None;

    if let Ok(url) = fs_fut.await {
        new_link = url;
    }

    if let Ok(url) = aws_fut.await {
        new_link = url;
    }

    Ok(new_link.unwrap_or_default())
}

#[cfg(all(not(feature = "local-storage"), feature = "aws-s3"))]
pub(crate) async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    fs: Data<Addr<FileSystemExecutor>>,
) -> Result<String, Error> {
    let filename = disposition.get_filename().unwrap();
    let system_file_name = format!("{}-{}", user_id, filename);

    let (sender, receiver) = tokio::sync::broadcast::channel(4);

    // Upload to AWS S3
    let aws_fut = aws_s3(system_file_name, receiver);

    read_form_data(&mut field, sender).await;

    let new_link = tokio::select! {
        b = aws_fut => b,
    };

    {
        use filesystem_actor::RemoveTmpFile;
        let _ = fs
          .send(RemoveTmpFile {
              file_name: format!("{}-{}", user_id, filename),
          })
          .await
          .ok();
    };
    Ok(new_link.unwrap_or_default())
}

#[cfg(all(feature = "local-storage", not(feature = "aws-s3")))]
pub(crate) async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    fs: Data<Addr<FileSystemExecutor>>,
) -> Result<String, Error> {
    let filename = disposition.get_filename().unwrap();
    let system_file_name = format!("{}-{}", user_id, filename);

    let (sender, receiver) = tokio::sync::broadcast::channel(4);

    let fs_fut = local_storage_write(
        system_file_name.clone(),
        fs.clone(),
        user_id,
        sender.subscribe(),
    );

    read_form_data(&mut field, sender).await;

    let new_link = tokio::select! {
        a = fs_fut => a,
    };

    {
        use filesystem_actor::RemoveTmpFile;
        let _ = fs
          .send(RemoveTmpFile {
              file_name: format!("{}-{}", user_id, filename),
          })
          .await
          .ok();
    };
    Ok(new_link.unwrap_or_default())
}

/// Read file from client
async fn read_form_data(field: &mut Field, sender: Sender<bytes::Bytes>) {
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        if let Err(err) = sender.send(data) {
            log::error!("{:?}", err);
        }
    }
}

/// Stream bytes directly to AWS S3 Service
#[cfg(feature = "aws-s3")]
async fn aws_s3(system_file_name: String, mut receiver: Receiver<bytes::Bytes>) -> Option<String> {
    let web_config = jirs_config::web::Configuration::read();
    let s3 = &web_config.s3;
    if !s3.active {
        return None;
    }
    s3.set_variables();
    log::debug!("{:?}", s3);

    let mut v: Vec<u8> = vec![];
    use bytes::Buf;

    while let Ok(b) = receiver.recv().await {
        v.extend_from_slice(b.bytes())
    }
    // let stream = receiver.into_stream();
    // let stream = stream.map_err(|_e| std::io::Error::from_raw_os_error(1));

    let client = S3Client::new(s3.region());
    let put_object = PutObjectRequest {
        bucket: s3.bucket.clone(),
        key: system_file_name.clone(),
        // body: Some(ByteStream::new(stream)),
        body: Some(v.into()),
        ..Default::default()
    };
    let id = match client.put_object(put_object).await {
        Ok(obj) => obj,
        Err(e) => {
            log::error!("{}", e);
            return None;
        }
    };
    log::debug!("{:?}", id);
    Some(aws_s3_url(system_file_name.as_str(), s3))
}

///
#[cfg(feature = "local-storage")]
async fn local_storage_write(
    system_file_name: String,
    fs: Data<Addr<filesystem_actor::FileSystemExecutor>>,
    user_id: jirs_data::UserId,
    receiver: Receiver<bytes::Bytes>,
) -> Option<String> {
    let web_config = jirs_config::web::Configuration::read();
    let fs_config = jirs_config::fs::Configuration::read();

    let _ = fs
      .send(filesystem_actor::CreateFile {
          source: receiver,
          file_name: system_file_name.clone(),
      })
      .await;

    Some(format!(
        "{proto}://{bind}{port}{client_path}/{user_id}-{filename}",
        proto = if web_config.ssl { "https" } else { "http" },
        bind = web_config.bind,
        port = match web_config.port.as_str() {
            "80" | "443" => "".to_string(),
            p => format!(":{}", p),
        },
        client_path = fs_config.client_path,
        user_id = user_id,
        filename = system_file_name
    ))
}

#[cfg(feature = "aws-s3")]
fn aws_s3_url(key: &str, config: &AmazonS3Storage) -> String {
    format!(
        "https://{bucket}.s3.{region}.amazonaws.com/{key}",
        bucket = config.bucket,
        region = config.region_name,
        key = key
    )
}
