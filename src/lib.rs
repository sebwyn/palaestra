mod controllers;
mod services;
mod models;
mod repository;

pub mod entrypoint {
    use std::{error::Error, path::Path};

    use agdb::{Db, DbImpl, FileStorageMemoryMapped, QueryBuilder};
    use axum::{routing::{get, post}, Router};
    use tokio::net::TcpListener;
    use crate::{controllers::notes_controller::{self, NotesController}, repository::notes_repository::NotesRepository, services::notes_service::NotesService};

    pub async fn start_server() {
        tracing_subscriber::fmt::init();
        
        let db = initialize_db().expect("Failed to initialize database");
        let notes_repository = NotesRepository::new(db);
        let notes_service = NotesService::new(notes_repository);
        let notes_controller = NotesController::new(notes_service);

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/notes", post(notes_controller.create_note));


        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    pub fn initialize_db() -> Result<DbImpl<FileStorageMemoryMapped>, Box<dyn Error>> {
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

