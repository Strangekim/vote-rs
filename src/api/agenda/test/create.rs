use crate::api::agenda::service::create_agenda;
use super::MockAgendaRepository;
use uuid::Uuid;

#[tokio::test]
async fn test_create_agenda_success() {
    let mock_repo = MockAgendaRepository::default();
    let creator_id = Uuid::new_v4();
    let title = "New Agenda".to_string();

    let result = create_agenda(&mock_repo, title.clone(), creator_id).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.title, title);
    assert_eq!(response.created_by, creator_id.to_string());
}

#[tokio::test]
async fn test_create_agenda_failure() {
    let mock_repo = MockAgendaRepository { should_fail: true };
    let creator_id = Uuid::new_v4();
    
    let result = create_agenda(&mock_repo, "Fail".to_string(), creator_id).await;
    
    assert!(result.is_err());
}
