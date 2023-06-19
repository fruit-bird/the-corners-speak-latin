use hollow::Hollow;
use serde::Deserialize;
use tide::{Request, Response};

#[derive(Debug, Deserialize)]
struct HollowInput {
    first: String,
    second: String,
    language: String,
}

async fn seek_truth(mut req: Request<()>) -> tide::Result {
    let HollowInput {
        first,
        second,
        language,
    } = req.body_json().await?;

    let hollow = Hollow::new(&first, &second, &language);
    let body = hollow.run().await?;

    let mut response = Response::new(200);
    response.set_body(body);

    Ok(response)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").serve_file("hollow-api/index.html")?;
    app.at("/hollow")
        .post(seek_truth)
        .get(|_| async move { Ok("This API only accepts POST requests") });

    println!("SeEk TruTh... http://127.0.0.1:8088");
    app.listen("0.0.0.0:8088").await?;

    Ok(())
}
