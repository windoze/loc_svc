use once_cell::sync::OnceCell;
use poem::{
    get, handler,
    listener::TcpListener,
    web::{Json, Path},
    Route, Server, error::NotFoundError,
};
use serde::{Deserialize, Serialize};

const LOCATION_DATA: &'static str = include_str!("data/locations.txt");

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: usize,
    name: String,
}

#[handler]
fn get_location(Path(id): Path<usize>) -> Result<Json<Response>, NotFoundError> {
    let locations = LOCATIONS.get_or_init(init_locations);
    if id >= locations.len() {
        return Err(NotFoundError);
    }
    let location = locations.get(id).unwrap();
    Ok(Json(Response {
        id,
        name: location.to_string(),
    }))
}

static LOCATIONS: OnceCell<Vec<String>> = OnceCell::new();

fn init_locations() -> Vec<String> {
    LOCATION_DATA.lines().map(|line| line.to_string()).collect()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/locations/:id", get(get_location));
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}
