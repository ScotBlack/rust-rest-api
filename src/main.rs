use tide::{Request, Response, StatusCode};

// #[async_std::main]
// async fn main() -> Result<(), std::io::Error> {
//     let mut app = tide::new();
//     app.at("/").get(|_| async move {
//         let mut resp = Response::new(StatusCode::Ok);
//         resp.set_body("Hello, world!");
//         Ok(resp)
//     });

//     // app.at("/hello").get(getArticles);
//     app.listen("127.0.0.1:8080").await?;
//     Ok(())
// }
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/articles").get(get_articles);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_articles(req: Request<()>) -> Result<Response,tide::Error> {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body("List of articles");
    Ok(resp)
}
