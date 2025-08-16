use axum::{extract, handler::Handler, Json};

use crate::{models::note::CreateNoteRequest, services::notes_service::NotesService};


#[derive(serde::Serialize)]
pub struct NoteCreationResponse {
    status: String,
}

pub struct NotesController {
    notes_service: NotesService
}

impl Handler<T, S> for NotesController {
    type Future = ;

    async fn call(
        self, 
        req: extract::Json(request): extract::Json<CreateNoteRequest>, 
        state: S) -> Json<NoteCreationResponse> {
        todo!()
    }
    
}


impl NotesController {
    pub fn new(notes_service: NotesService) -> Self {
        Self { notes_service }
    }

    pub async fn create_note(
        &mut self, 
        extract::Json(request): extract::Json<CreateNoteRequest>
    ) -> Json<NoteCreationResponse> {
        
        self.notes_service.create_note(request.title, request.content).await;
        Json(NoteCreationResponse { status: "Ok".to_string(), })
    }

}