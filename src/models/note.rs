use agdb::{DbId, UserValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
}


#[derive(Debug, UserValue, Clone)]
pub struct Note {
    pub db_id: Option<DbId>,    
    pub title: String,
    pub content: String,
}