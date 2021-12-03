use std::sync::Arc;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use enigo::{Enigo, Key, KeyboardControllable};
use rust_embed::RustEmbed;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply, filters::BoxedFilter, http::header::HeaderValue, path::Tail, reply::Response};

#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct Asset;

#[tokio::main]
async fn main() {
    let ip = local_ipaddress::get();
    println!("Listening on http://{}", ip.unwrap());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80);
    let routes = get_routes();

    warp::serve(routes).run(addr).await
}

fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let enigo_filter = {
        let enigo = Arc::new(Mutex::new(Enigo::new()));
        warp::any().map(move || enigo.clone())
    };

    let log_wrapper = warp::log::custom(|info| {
        println!("{} {}", info.method(), info.path());
    });

    let handle_index = warp::path::end()
        .and_then(serve_index);

    let handle_dist = warp::path::tail()
        .and_then(serve_dist);

    let handle_press = warp::post()
        .and(enigo_filter.clone())
        .and(warp::path!("api" / "press" / String))
        .and_then(press_key);

    handle_index
        .or(handle_dist)
        .or(handle_press)
        .with(log_wrapper)
        .boxed()
}

async fn serve_index() -> Result<impl Reply, Rejection> {
    serve_file("index.html")
}

async fn serve_dist(path: Tail) -> Result<impl Reply, Rejection> {
    serve_file(path.as_str())
}

async fn press_key(enigo: Arc<Mutex<Enigo>>, key_code: String) -> Result<impl Reply, Rejection> {
    let key = match key_code.as_ref() {
        "down" => Key::DownArrow,
        "left" => Key::LeftArrow,
        "page_down" => Key::PageDown,
        "page_up" => Key::PageUp,
        "right" => Key::RightArrow,
        "space" => Key::Space,
        "up" => Key::UpArrow,
        _ => {
            return Err(warp::reject());
        }
    };

    enigo.lock().await.key_click(key);
    Ok(warp::reply())
}

fn serve_file(path: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut response = Response::new(asset.into());
    response.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
    Ok(response)
}
