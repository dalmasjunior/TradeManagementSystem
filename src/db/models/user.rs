use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::services::jwt::create_jwt;

use super::super::schema::{*, self};
use super::super::schema::users::dsl::users as users_dsl;
use super::wallet::Wallet;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub wallet_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        users_dsl
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .find(id)
            .get_result::<User>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn find_by_email(conn: &mut SqliteConnection, email: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .filter(users::email.eq(email))
            .get_result::<User>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn create(conn: &mut SqliteConnection, name: String, email: String, wallet_id: String, password: String) -> Option<Self> {
        let new_id = Uuid::new_v4().as_hyphenated().to_string();

        if email.is_empty() || password.is_empty() || name.is_empty() || wallet_id.is_empty() {
            return None;
        }
        
        
        if Self::find_by_email(conn, email.clone()).is_some() {
            return None;
        }
        
        
        if Wallet::find_by_id(conn, wallet_id.clone()).is_none() {
            return None;
        }
        
        //criptograff password
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();


        let new_user = Self::new_user_struct(new_id, name, email, wallet_id, hashed_password);

        diesel::insert_into(users_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");
        
        Self::find_by_id(conn, new_user.id)
    }

    fn new_user_struct(id: String, name: String, email: String, wallet_id: String, password: String) -> Self {
        Self {
            id: id,
            name: name,
            email: email,
            password: password,
            wallet_id: wallet_id,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn update(conn: &mut SqliteConnection, id: String, name: String, email: String, wallet: String, password: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .find(id)
            .get_result::<User>(conn) {
            let updated_user = Self::update_user_struct(record, name, email, wallet, password);
            diesel::update(users_dsl.find(updated_user.id.clone()))
                .set((schema::users::name.eq(updated_user.name.clone()),
                    schema::users::email.eq(updated_user.email.clone()),
                    schema::users::wallet_id.eq(updated_user.wallet_id.clone()),                    
                    schema::users::password.eq(bcrypt::hash(updated_user.password.clone(), bcrypt::DEFAULT_COST).unwrap()),
                    schema::users::updated_at.eq(chrono::Local::now().naive_local())))
                .execute(conn)
                .expect("Error updating user");
            Some(updated_user)
            } else {
                None
            }
    }

    fn update_user_struct(mut user: Self, name: String, email: String, wallet: String, password: String) -> Self {
        user.name = name;
        user.email = email;
        user.wallet_id = wallet;
        user.password = password;
        user.updated_at = chrono::Local::now().naive_local();
        user
    }

    pub fn delete(conn: &mut SqliteConnection, id: String) -> bool {
        if let Ok(_record) = users_dsl
            .find(id.clone())
            .get_result::<User>(conn) {
            diesel::delete(users_dsl.find(id))
                .execute(conn)
                .expect("Error deleting user");
            true
            } else {
                false
            }
    }

    pub fn login(conn: &mut SqliteConnection, email: String, password: String) -> Option<String> {
        if let Ok(record) = users_dsl
            .filter(users::email.eq(email))
            .get_result::<User>(conn) {
                if bcrypt::verify(password, &record.password).unwrap() {
                    Some(create_jwt(record.id).unwrap())
                } else {
                    None
                }
            } else {
                None
            }
    }

}

