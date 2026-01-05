use crate::api::auth::repository::traits::UserRepository;
use crate::api::auth::repository::UserEntity;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg(test)]
pub mod signup;
#[cfg(test)]
pub mod login;

/// Mock Repository 정의
/// 실제 DB 연결 없이 서비스 로직을 테스트하기 위해 사용됨
pub struct MockUserRepository {
    pub should_exist: bool,          // exists 호출 시 반환값 (true: 이미 존재함)
    pub save_should_fail: bool,      // save 호출 시 에러 발생 여부
    pub find_result: Option<UserEntity>, // find_by_username 호출 시 반환값 (Some: 사용자 있음)
}

impl Default for MockUserRepository {
    fn default() -> Self {
        Self {
            should_exist: false,
            save_should_fail: false,
            find_result: None,
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn exists(&self, _username: &str) -> Result<bool, sqlx::Error> {
        Ok(self.should_exist)
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        Ok(self.find_result.clone())
    }

    async fn save(&self, _username: &str) -> Result<Uuid, sqlx::Error> {
        if self.save_should_fail {
            Err(sqlx::Error::RowNotFound)
        } else {
            Ok(Uuid::new_v4())
        }
    }
}
