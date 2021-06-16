use crate::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

type ConnectionPool = PooledConnection<ConnectionManager<SqliteConnection>>;

#[allow(dead_code)]
pub async fn get_by_username<'a>(conn: &'a ConnectionPool, username_input: String) -> Option<User> {
    let user = users
        .filter(username.eq(username_input))
        .select((id, username, email, password, is_admin))
        .first::<(Option<i32>, String, String, String, bool)>(conn);

    match user {
        Ok(user) => Some(User {
            id: user.0,
            username: user.1,
            email: user.2,
            password: user.3,
            is_admin: user.4,
        }),
        Err(_) => None,
    }
}

pub async fn update_password<'a>(
    conn: &'a ConnectionPool,
    username_input: String,
    new_password: String,
) -> Result<usize, diesel::result::Error> {
    let target = users.filter(username.eq(username_input));
    diesel::update(target)
        .set(password.eq(new_password))
        .execute(conn)
}

pub async fn insert<'a>(conn: &'a ConnectionPool, user: User) -> Result<User, ()> {
    match diesel::insert_into(users).values(&user).execute(conn) {
        Ok(_) => Ok(user),
        Err(_) => Err(()),
    }
}
