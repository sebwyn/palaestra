use agdb::QueryBuilder;

use crate::models::note::Note;

pub struct NotesRepository {
    db: agdb::Db,
}

impl NotesRepository {
    pub fn new(db: agdb::Db) -> Self {
        Self { db }
    }

    pub fn insert_note(&mut self, title: String, content: String) -> Result<Note, agdb::QueryError> {
        let mut note = Note {
            db_id: None,
            title,
            content,
        };
        
        let value_query = QueryBuilder::insert()
            .nodes()
            .values([note.clone()])
            .query();
        
        let query_result = self.db.exec_mut(value_query)?;
        let note_id = query_result.ids().first().unwrap().clone(); 
        
        let edge_query = QueryBuilder::insert()
            .edges()
            .from("users")
            .to(note_id)
            .query();
        self.db.exec_mut(edge_query)?;

        note.db_id.replace(note_id);
        Ok(note)
    }

    pub fn get_all_notes(&self) -> Result<Note, agdb::QueryError> {
        let query = QueryBuilder::select()
            .elements::<Note>()
            .search()
            .from("users")
            .query();

        Ok(self.db.exec(query)?.try_into()?)
    }

}