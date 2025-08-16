use std::sync::Arc;

use tokio::sync::{RwLock};

mod controllers;
mod services;
mod models;
mod repository;

#[derive(Clone)]
pub struct ApiState {
    db: Arc<RwLock<agdb::Db>>
}

pub mod entrypoint {
    use std::{error::Error, path::Path, sync::Arc};

    use agdb::{Db, QueryBuilder};
    use axum::{routing::get, Router};
    use tokio::{net::TcpListener, sync::RwLock};
    use crate::{controllers::notes_controller::{self}, ApiState};

    pub async fn start_server() {
        tracing_subscriber::fmt::init();
        
        let db = initialize_db().expect("Failed to initialize database");

        let api_state = ApiState {
            db: Arc::new(RwLock::new(db))
        };

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .nest("/notes", notes_controller::routes())
            .with_state(api_state);

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    pub fn initialize_db() -> Result<Db, Box<dyn Error>> {
        if !Path::new("agdb_paleastra.agdb").exists() {
            let mut db = Db::new("agdb_paleastra.agdb")?;
            
            db.exec_mut(QueryBuilder::insert()
                .nodes()
                .aliases("notes")
                .query())?;

        }
        Ok(Db::new("agdb_paleastra.agdb")?)
    }
}

