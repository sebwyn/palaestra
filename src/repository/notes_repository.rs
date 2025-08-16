use std::sync::Arc;

use agdb::QueryBuilder;
use tokio::sync::RwLock;

use crate::models::note::Note;

pub struct NotesRepository {
    db: Arc<RwLock<agdb::Db>>,
}

impl NotesRepository {
    pub fn new(db: Arc<RwLock<agdb::Db>>) -> Self {
        Self { db }
    }

    pub async fn insert_note(&self, title: String, content: String) -> Result<Note, agdb::QueryError> {
        let mut note = Note {
            db_id: None,
            title,
            content,
        };
        
        let insert_note_result = self.db
            .write().await
            .exec_mut(QueryBuilder::insert()
                .nodes()
                .values([note.clone()])
                .query()
        )?;

        let note_id = insert_note_result.ids().into_iter().next().unwrap(); 
        
        self.db
            .write().await
            .exec_mut(QueryBuilder::insert()
                .edges()
                .from("notes")
                .to(note_id)
                .query()
            )?;

        note.db_id.replace(note_id);
        Ok(note)
    }

    pub async fn get_all_notes(&self) -> Result<Vec<Note>, agdb::QueryError> {
        Ok(self.db
            .read().await
            .exec(QueryBuilder::select()
                .elements::<Note>()
                .search()
                .from("notes")
                .where_()
                .keys("title")
                .query())?
                .try_into()?
        )
    }

}