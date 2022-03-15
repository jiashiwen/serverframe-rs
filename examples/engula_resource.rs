use anyhow::Result;
use engula_client::{Any, Blob, List, Map, Universe, I64};

#[tokio::main]
async fn main() -> Result<()> {
    // The address of the server you started above.
    let url = "http://localhost:21716";
    let uv = Universe::connect(url).await?;
    let db = uv.database("db");


    println!("{:?}", db.desc().await.unwrap());
    uv.delete_database("db").await?;
    // let _ = uv.delete_database("db").await?;
    // let db = uv.create_database("db").await?;
    // let c1 = db.create_collection::<Any>("c1").await?;
    // let c2 = db.create_collection::<I64>("c2").await?;
    // let c3 = db.create_collection::<Blob>("c3").await?;
    // let c4 = db.create_collection::<List<Any>>("c4").await?;
    // let c5 = db.create_collection::<List<I64>>("c5").await?;
    // let c6 = db.create_collection::<List<Blob>>("c6").await?;
    // let c7 = db.create_collection::<Map<Any>>("c7").await?;
    // let c8 = db.create_collection::<Map<I64>>("c8").await?;
    // let c9 = db.create_collection::<Map<Blob>>("c9").await?;
    Ok(())
}