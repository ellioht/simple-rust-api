use crate::users::models::User;
use crate::users::models::UserCreateRequest;

pub async fn get_users() -> Vec<User> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
        },
    ];

    users
}

pub async fn create_user(req: UserCreateRequest) -> Result<User, ()> {
    let user = User {
        id: 3,
        name: req.name,
    };

    Ok(user)
}