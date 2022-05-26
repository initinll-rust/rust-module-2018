// Absolute path
//use crate::services::entity::user_entity::UserEntity;

// Relative path
use super::super::entity::user_entity::UserEntity;

pub fn get_user() -> UserEntity {
    let user:UserEntity = UserEntity{        
        username: "hey".to_string(),
        email: "abc@bbc.com".to_string(),
        sign_in_count: 5,
        active:true
    };

    return user;
}