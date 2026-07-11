use axum::{Router, routing::get};
use sqlx::PgPool;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL doit être définie dans le fichier .env");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Impossible de se connecter à la base de données");

    println!("DB connectée");

    let app: Router<()> = Router::new().route("/",
        get(|| async { "Bienvenue sur l'API de compta !"})
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await 
        .unwrap()
    ;

    println!("Serveur démarré");

    axum::serve(listener, app).await.unwrap();
}