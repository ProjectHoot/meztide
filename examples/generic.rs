use meztide::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new("https://c.tide.tk/api/unstable");
    let info = client.instance_info().await.unwrap();
    println!("Instance info: {:#?}\n", info);

    let communities = client
        .communities(&ReqCommunities {
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("Communities: {:#?}\n", communities);

    let community_id = CommunityId(1);
    let community = community_id.to_community(&client).await.unwrap();
    println!("Community info: {:#?}", community);

    let posts = client
        .posts(&ReqPosts {
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("Posts: {:#?}\n", posts);
}
