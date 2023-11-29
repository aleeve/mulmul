use atrium_api::app::bsky::feed::post::Record;
use atrium_api::com::atproto::sync::subscribe_repos::Message;
use atrium_xrpc_server::stream::frames::Frame;
use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut stream, _) =
        connect_async("wss://bsky.network/xrpc/com.atproto.sync.subscribeRepos").await?;

    while let Some(Ok(tungstenite::Message::Binary(message))) = stream.next().await {
        tokio::spawn( async move {
            process_message(&message).await.unwrap();
        });
    }
    Ok(())
}

async fn process_message(message: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match Frame::try_from(message)? {
        Frame::Message(message) => {
            match message.body {
                Message::Commit(commit) => {
                    for op in commit.ops {
                        let collection = op.path.split('/').next().expect("op.path is empty");
                        if op.action != "create" || collection != "app.bsky.feed.post" {
                            continue;
                        }
                        let (items, _) =
                            rs_car::car_read_all(&mut commit.blocks.as_slice(), true).await?;
                        if let Some((_, item)) = items.iter().find(|(cid, _)| Some(*cid) == op.cid)
                        {
                            if let Ok(value) =
                                ciborium::de::from_reader::<Record, _>(&mut item.as_slice())
                            {
                                println!("{}, {}: {}", value.created_at, commit.repo, value.text);
                            } else {
                                // TODO
                            }
                        }
                    }
                }
                _ => println!("{:?}", message.body),
            }
        }
        Frame::Error(err) => println!("{err:?}"),
    }
    Ok(())
}
