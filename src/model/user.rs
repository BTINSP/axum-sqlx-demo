use sqlx::{Pool, Postgres, Row};
use serde::{Serialize, Deserialize};

use crate::common::Errors;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: i64,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "password")]
    password: String,

    role: i32
}

impl User {

    pub async fn get_user_by_username(pool: Pool<Postgres>, usernmae: String) -> Result<User, Errors> {
        let result = sqlx::query("select * from demo_user where username = $1")
                    .bind(usernmae)
                    .fetch_one(&pool)
                    .await
                    .map(|user| -> User {
                        let id = user.get("id");
                        let username = user.get("username");
                        let password = user.get("password");
                        let role = user.get("role");
                            User {
                                id,
                                username,
                                password,
                                role
                            }
                        }
                    );
        match result {
            Ok(data) => {Ok(data)},
            Err(err) => {Err(err.into())},
        }
            
    }


    pub async fn get_all_user(pool: Pool<Postgres>) -> Result<Vec<User>, Errors> {
        
        let result = sqlx::query("select * from demo_user")
                .fetch_all(&pool)
                .await
                .map(|users| -> Vec<User> {
                    let mut users_list: Vec<User> = Vec::new();
                    for user in users {
                        let id = user.get("id");
                        let username = user.get("username");
                        let password = user.get("password");
                        let role = user.get("role");
                        let user = User{
                            id,
                            username,
                            password,
                            role
                        };
                        users_list.push(user)
                    }
                    users_list
                });
        match result {
            Ok(data) => {Ok(data)},
            Err(err) => {Err(err.into())},
        }
    }
    
}