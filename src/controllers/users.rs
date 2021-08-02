use actix_web::{web, HttpResponse, Responder, http::StatusCode};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::services::sqlite;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    name: String,
    age: String,
}

#[derive(Deserialize)]
pub struct UserInsert {
    name: String,
    age: String,
}

pub async fn detail(web::Path(id): web::Path<String>) -> impl Responder {
    let result_connection = sqlite::connection().await;

    match result_connection {
        Ok(conn) => {
            let result = conn.prepare("SELECT id, name, age FROM users where id = :id");

            match result {
                Ok(mut stmt) => {
                    let mut users: Vec<User> = Vec::<User>::new();
        
                    let users_iter = stmt.query_map(&[(":id", &id)], |row| {
                        Ok(User {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            age: row.get(2)?,
                        })
                    });
                
                    for user in users_iter.unwrap(){
                        users.push(user.unwrap());
                    }

                    if users.len() == 0 {
                        HttpResponse::Ok()
                        .status(StatusCode::NOT_FOUND)
                        .finish()
                    } else {
                        HttpResponse::Ok().json(users)
                    }
                
                }
                Err(err) => {
                    println!("Error: {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        } Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn list() -> impl Responder {
    let result_connection = sqlite::connection().await;

    match result_connection {
        Ok(conn) => {
            let result = conn.prepare("SELECT id, name, age FROM users");

            match result {
                Ok(mut stmt) => {
                    let mut users: Vec<User> = Vec::<User>::new();
        
                    let users_iter = stmt.query_map([], |row| {
                        Ok(User {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            age: row.get(2)?,
                        })
                    });
                
                    for user in users_iter.unwrap(){
                        users.push(user.unwrap());
                    }
                
                    HttpResponse::Ok().json(users)
                }
                Err(err) => {
                    println!("Error: {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        } Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn add(data: web::Json<UserInsert>) -> impl Responder {
    let ulid = Ulid::new();

    let user = User {
        id: ulid.to_string(),
        name: data.name.to_string(),
        age: data.age.to_string(),
    };

    let result_connection = sqlite::connection().await;

    match result_connection {
        Ok(conn) => {
            let result_insert = conn.execute(
                "INSERT INTO users (id, name, age) values (?1, ?2, ?3)",
                &[
                    &user.id,
                    &user.name,
                    &user.age
                ],
            );
            match result_insert {
                Ok(_) => {
                    HttpResponse::Ok().status(StatusCode::CREATED).json(user)
                }
                Err(err) => {
                    println!("Error: {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        } Err(err) => {
            println!("Error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}