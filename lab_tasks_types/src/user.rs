pub struct User {
    id: u32,
    name: String,
    created_at: String,
}

impl User {
    pub fn new(id: Option<u32>, name: String, created_at: Option<String>) -> Self {
        let curr_date = chrono::offset::Local::now().to_string();
        Self {
            id: id.unwrap_or(0),
            name,
            created_at: created_at.unwrap_or(curr_date),
        }
    }
    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}
