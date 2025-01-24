use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub file: File,
    pub preview: Preview,
    pub sample: Sample,
    pub score: Score,
    pub tags: Tags,
    pub locked_tags: Vec<String>,
    pub change_seq: u64,
    pub flags: Flags,
    pub rating: String,
    pub fav_count: u64,
    pub sources: Vec<String>,
    pub pools: Vec<u64>,
    pub relationships: Relationships,
    pub approver_id: Option<u64>,
    pub uploader_id: u64,
    pub description: String,
    pub comment_count: u64,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub width: u64,
    pub height: u64,
    pub ext: String,
    pub size: u64,
    pub md5: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preview {
    pub width: u64,
    pub height: u64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub has: bool,
    pub height: u64,
    pub width: u64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub up: u64,
    pub down: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub general: Vec<String>,
    pub artist: Vec<String>,
    pub contributor: Vec<String>,
    pub copyright: Vec<String>,
    pub character: Vec<String>,
    pub species: Vec<String>,
    pub invalid: Vec<String>,
    pub meta: Vec<String>,
    pub lore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationships {
    pub parent_id: Option<u64>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Vec<u64>,
}
