use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

mod config;
mod ipdb;
mod overlay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url_para = config::UrlPara::from_file("config.toml")?;

    let mut url = format!(
        "https://api.mapbox.com/styles/v1/{username}/{style_id}/static/{overlay}/auto/{width}x{height}",
        username = url_para.username,
        style_id = url_para.style_id,
        overlay = "",
        width = 512,
        height = 512,
    ).parse::<hyper::Uri>()?;

    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(443);

    let address = format!("{}:{}", host, port);

    let stream = TcpStream::connect(address).await?;

    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().expect("uri has no authority").clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    println!("response status: {}", res.status());

    Ok(())
}
