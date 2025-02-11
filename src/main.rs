use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Construir nuestra aplicación con una ruta
    let app = Router::new()
        .route("/", get(handler))
        .route("/about", get(about_handler));

    // Definir la dirección y el puerto
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor escuchando en http://{}", addr);

    // Ejecutar el servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Manejador para la ruta raíz
async fn handler() -> Html<&'static str> {
    Html("<h1>¡Hola, mundo!</h1> <a href='/about'><button>informacion</button></a>")
}
async fn about_handler() -> Html<&'static str> {
    Html("<h1>Acerca de mi aplicación</h1><p>Esta es una aplicación de ejemplo usando Axum.</p>")
}