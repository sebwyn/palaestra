use std::error::Error;

use crate::{models::note::Note, repository::notes_repository::NotesRepository};

pub struct NotesService {
    notes_repository: NotesRepository,
}

impl NotesService {
    pub fn new(notes_repository: NotesRepository) -> Self {
        Self { notes_repository }
    }

    pub async fn create_note(&self, title: String, content: String) -> Result<i64, Box<dyn Error>> {
        let created_note = self.notes_repository.insert_note(title, content)
            .await?;

        Ok(created_note.db_id.unwrap().0)
    }

    pub async fn list_notes(&self) -> Result<Vec<Note>, Box<dyn Error>> {
        Ok(self.notes_repository
            .get_all_notes()
            .await?)
    }
}