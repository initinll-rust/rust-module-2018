// Absolute path
use crate::models::user_model::UserModel;
// Relative path
//use super::super::super::models::user_model::UserM;

// Absolute path
//use crate::services::repository::user_repository;
// Relative path
use super::super::repository::user_repository;

pub fn get_user() -> UserModel {
    let user_detail = user_repository::get_user();

    let user:UserModel = UserModel {        
        username: user_detail.username,
        email: user_detail.email,
        sign_in_count: user_detail.sign_in_count,
        active: user_detail.active
    };

    return user;
}