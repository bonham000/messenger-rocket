use diesel;

use super::schema::messages;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
    author: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub status: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "messages"]
pub struct SavedMessage {
    pub id: i32,
    pub message: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct InsertableMessage {
    pub message: String,
    pub author: String,
}

impl InsertableMessage {
    pub fn from_message(message: Message) -> InsertableMessage {
        InsertableMessage {
            message: (*message.message).to_string(),
            author: (*message.author.name).to_string(),
        }
    }
}