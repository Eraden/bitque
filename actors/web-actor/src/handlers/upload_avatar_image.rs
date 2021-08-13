use actix::Addr;
use actix_multipart::Field;
use actix_web::http::header::ContentDisposition;
use actix_web::web::Data;
use actix_web::Error;
use common::*;
use futures::StreamExt;
use jirs_data::UserId;
use tokio::sync::broadcast::{Receiver, Sender};

#[cfg(all(feature = "local-storage", feature = "aws-s3"))]
pub(crate) async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    fs: Data<Addr<filesystem_actor::FileSystemExecutor>>,
    amazon: Data<Addr<amazon_actor::AmazonExecutor>>,
) -> Result<String, Error> {
    let filename = disposition.get_filename().unwrap();
    let system_file_name = format!("{}-{}", user_id, filename);

    let (sender, receiver) = tokio::sync::broadcast::channel(64);

    let fs_fut = local_storage_write(system_file_name.clone(), fs, user_id, sender.subscribe());
    let aws_fut = aws_s3(system_file_name, amazon, receiver);
    let read_fut = read_form_data(&mut field, sender);

    let fs_join = tokio::task::spawn(fs_fut);
    let aws_join = tokio::task::spawn(aws_fut);
    read_fut.await;

    let mut new_link = None;

    if let Ok(url) = fs_join.await {
        new_link = url;
    }

    if let Ok(url) = aws_join.await {
        new_link = url;
    }

    Ok(new_link.unwrap_or_default())
}

#[cfg(all(not(feature = "local-storage"), feature = "aws-s3"))]
pub(crate) async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    amazon: Data<Addr<amazon_actor::AmazonExecutor>>,
) -> Result<String, Error> {
    let filename = disposition.get_filename().unwrap();
    let system_file_name = format!("{}-{}", user_id, filename);

    let (sender, receiver) = tokio::sync::broadcast::channel(64);

    let aws_fut = aws_s3(system_file_name, amazon, receiver);
    let read_fut = read_form_data(&mut field, sender);

    let aws_join = tokio::task::spawn(aws_fut);
    read_fut.await;

    let mut new_link = None;

    if let Ok(url) = aws_join.await {
        new_link = url;
    }

    Ok(new_link.unwrap_or_default())
}

#[cfg(all(feature = "local-storage", not(feature = "aws-s3")))]
pub(crate) async fn handle_image(
    user_id: UserId,
    mut field: Field,
    disposition: ContentDisposition,
    fs: Data<Addr<filesystem_actor::FileSystemExecutor>>,
) -> Result<String, Error> {
    let filename = disposition.get_filename().unwrap();
    let system_file_name = format!("{}-{}", user_id, filename);

    let (sender, receiver) = tokio::sync::broadcast::channel(64);

    let fs_fut = local_storage_write(system_file_name, fs, user_id, sender.subscribe());
    let read_fut = read_form_data(&mut field, sender);

    let fs_join = tokio::task::spawn(fs_fut);
    read_fut.await;

    let mut new_link = None;

    if let Ok(url) = fs_join.await {
        new_link = url;
    }

    Ok(new_link.unwrap_or_default())
}

/// Read file from client
async fn read_form_data(field: &mut Field, sender: Sender<common::bytes::Bytes>) {
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        if let Err(err) = sender.send(data) {
            log::error!("{:?}", err);
        }
    }
}

/// Stream bytes directly to AWS S3 Service
#[cfg(feature = "aws-s3")]
async fn aws_s3(
    system_file_name: String,
    amazon: Data<Addr<amazon_actor::AmazonExecutor>>,
    receiver: Receiver<bytes::Bytes>,
) -> Option<String> {
    let s3 = jirs_config::amazon::config();
    if !s3.active {
        return None;
    }

    match amazon
        .send(amazon_actor::S3PutObject {
            source: receiver,
            file_name: system_file_name.to_string(),
        })
        .await
    {
        Ok(Ok(s)) => Some(s),
        _ => None,
    }
}

#[cfg(feature = "local-storage")]
async fn local_storage_write(
    system_file_name: String,
    fs: Data<Addr<filesystem_actor::FileSystemExecutor>>,
    user_id: jirs_data::UserId,
    receiver: Receiver<bytes::Bytes>,
) -> Option<String> {
    let web_config = jirs_config::web::config();
    let fs_config = jirs_config::fs::config();

    match fs
        .send(filesystem_actor::CreateFile {
            source: receiver,
            file_name: system_file_name.to_string(),
        })
        .await
    {
        Ok(Ok(_)) => Some(format!(
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
        )),
        Ok(_) => None,
        _ => None,
    }
}
