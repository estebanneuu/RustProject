use serde::{Deserialize, Serialize};

use crate::schema::agents;
use crate::schema::commands;
use crate::schema::results;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Agents {
  pub id: i32,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "agents"]
pub struct NewAgent<> {
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Commands {
  pub id: i32,
  pub command: String,
  pub agent_id: i32,
  pub created_at: chrono::NaiveDateTime,
  pub done: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "commands"]
pub struct NewCommand<'a,'b> {
  pub command: &'a str,
  pub agent_id: &'b i32,
  pub created_at: chrono::NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandPayload {
  pub command: String,
  pub agent_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Results {
  pub id: i32,
  pub command_id: i32,
  pub agent_id: i32,
  pub result_content: String,
  pub done_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "results"]
pub struct NewResult<'a,'b,'c> {
  pub command_id: &'a i32,
  pub agent_id: &'b i32,
  pub result_content: &'c str,
  pub done_at: chrono::NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPayload {
  pub command_id: i32,
  pub agent_id: i32,
  pub result_content: String,
}