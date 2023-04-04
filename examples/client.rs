use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse, Kvpair};
use tokio::net::TcpStream;
use tracing::info;
use kv::value::Value::String;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    // 使用 AsyncProstStream 来处理 TCP Frame
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".to_string().into());

    // 发送 HSET 命令
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    let get_cmd = CommandRequest::new_hget("table1","hello");
    client.send(get_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get get_cmd response {:?}",data);
    }

    //get hmget
    let hmget_cmd = CommandRequest::new_hmget("table1", vec!["hello","aa"]);
    client.send(hmget_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get hmget_cmd response {:?}",data);
    }
    //get hmset
    let hmset_cmd = CommandRequest::new_hmset("table1", vec![("name","pwh").into(), ("age", 22_i64).into()]);
    client.send(hmset_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get hmset_cmd response {:?}",data);
    }
    let hmdel_cmd = CommandRequest::new_hmdel("table1", vec!["name","age"]);
    client.send(hmdel_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get hmdel_cmd response {:?}",data);
    }

    let get_all_cmd = CommandRequest::new_hgetall("table1");
    client.send(get_all_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get get_cmd response {:?}",data);
    }
    let emx_cmd = CommandRequest::new_hmexist("table1",vec!["hello","name"]);
    client.send(emx_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get emx_cmd response {:?}",data);
    }

    Ok(())
}