pub struct CreateUserOutput1 {
    pub id: String,
    pub name: String,
}

#[non_exhaustive]
pub struct CreateUserOutput2 {
    pub id: String,
    pub name: String,
}

impl CreateUserOutput2 {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
