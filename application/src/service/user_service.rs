
use common::error::InfraError;
use domain::model::user::User;
use domain::repositories::user_repository::UserRepository;
use infrastructure::persistence::user_repository_impl::UserRepositoryImpl;
use anyhow::Error;
pub struct UserService {
    repository: UserRepositoryImpl,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repository: UserRepositoryImpl {},
        }
    }

    pub async fn find_all_users(&self) -> Result<Vec<User>, Error> {
        self.repository.find_all().await
    }

    pub async fn find_user_by_id(&self,id: i32) -> Result<Option<User>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self,user: User) -> Result<User,InfraError> {
        self.repository.create(user).await
    }

    pub async fn update_user(&self,user: User) -> Result<User, Error> {
        self.repository.update(user).await
    }

    pub async fn delete_user(&self,id: i32) -> Result<bool, Error> {
        Self::new().repository.delete(id).await
    }
}