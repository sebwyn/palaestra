use crate::{models::note::Note, repository::notes_repository::NotesRepository};

pub struct NotesService {
    notes_repository: NotesRepository,
}

impl NotesService {
    pub fn new(notes_repository: NotesRepository) -> Self {
        Self { notes_repository }
    }

    pub async fn create_note(&mut self, title: String, content: String) -> i64 {
        let created_note = self.notes_repository.insert_note(title, content)
            .expect("Failed to insert note");

        created_note.db_id.unwrap().0
    }
}