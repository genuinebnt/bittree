use crate::domain::repository::AuthRepository;

#[derive(Debug)]
pub struct RegisterServiceImpl {
    auth_repo: Box<dyn AuthRepository>,
    jwt_service: Box<dyn JwtService>,
}
