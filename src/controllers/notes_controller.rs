use axum::{extract::{self, FromRef, State}, routing::{get, post}, Json, Router};
use serde::Serialize;

use crate::{controllers::Response, models::note::{CreateNoteRequest, Note}, repository::notes_repository::NotesRepository, services::notes_service::NotesService, ApiState};

#[derive(Serialize)]
pub struct NoteResponse {
    id: Option<i64>,
    title: String,
    content: String
}

impl From<Note> for NoteResponse {
    fn from(value: Note) -> Self {
        Self {
            id: value.db_id.map(|id| { id.0 }),
            title: value.title,
            content: value.content
        }
    }
}

pub struct NotesController {
    notes_service: NotesService
}

impl FromRef<ApiState> for NotesController {
    fn from_ref(input: &ApiState) -> Self {
        Self {
            notes_service: NotesService::new(NotesRepository::new(input.db.clone()))
        }
    }
}

pub fn routes() -> Router<ApiState> {
    Router::new()
        .route("/create", post(create_note))
        .route("/list", get(get_all_notes))
}

async fn create_note( 
    State(controller): State<NotesController>,
    extract::Json(request): extract::Json<CreateNoteRequest>,
) -> Json<Response<i64>> {
    Json(Response::from_result(controller.notes_service.create_note(request.title, request.content).await))
}

async fn get_all_notes(
    State(controller): State<NotesController>
) -> Json<Response<Vec<NoteResponse>>> {
    Json(Response::from_result(
        controller.notes_service
            .list_notes()
            .await
            .map(|notes| notes.into_iter().map(Into::into).collect())
        )
    )
}
