// Service 레이어에서 사용할 에러 타입
// 비즈니스 로직 에러만 표현 (메시지는 Handler에서 결정)
#[derive(Debug)]
pub enum AuthServiceError {
    UserAlreadyExists,
    DatabaseError,
    // 필요시 추가: UserNotFound, InvalidCredentials 등
}
