//! Job scheduler tests for Redis Enterprise

use redis_enterprise::{CreateScheduledJobRequest, EnterpriseClient, JobSchedulerHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn test_scheduled_job() -> serde_json::Value {
    json!({
        "job_id": "job-backup-daily",
        "name": "Daily Database Backup",
        "job_type": "backup",
        "schedule": "0 2 * * *",
        "enabled": true,
        "last_run": "2023-01-01T02:00:00Z",
        "next_run": "2023-01-02T02:00:00Z",
        "params": {
            "bdb_uid": 1,
            "backup_location": "/backups/daily",
            "retention_days": 7
        }
    })
}

fn test_scheduled_job_minimal() -> serde_json::Value {
    json!({
        "job_id": "job-simple",
        "name": "Simple Job",
        "job_type": "maintenance",
        "schedule": "0 0 * * 0"
    })
}

fn test_scheduled_job_disabled() -> serde_json::Value {
    json!({
        "job_id": "job-disabled",
        "name": "Disabled Job",
        "job_type": "cleanup",
        "schedule": "0 3 * * *",
        "enabled": false,
        "params": {
            "cleanup_type": "logs",
            "max_age_days": 30
        }
    })
}

fn test_job_execution_running() -> serde_json::Value {
    json!({
        "execution_id": "exec-123-abc",
        "job_id": "job-backup-daily",
        "start_time": "2023-01-01T02:00:00Z",
        "status": "running"
    })
}

fn test_job_execution_completed() -> serde_json::Value {
    json!({
        "execution_id": "exec-456-def",
        "job_id": "job-backup-daily",
        "start_time": "2022-12-31T02:00:00Z",
        "end_time": "2022-12-31T02:15:00Z",
        "status": "completed"
    })
}

fn test_job_execution_failed() -> serde_json::Value {
    json!({
        "execution_id": "exec-789-ghi",
        "job_id": "job-backup-daily",
        "start_time": "2022-12-30T02:00:00Z",
        "end_time": "2022-12-30T02:05:00Z",
        "status": "failed",
        "error": "Insufficient disk space for backup"
    })
}

#[tokio::test]
async fn test_job_scheduler_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_scheduled_job(),
            test_scheduled_job_minimal(),
            test_scheduled_job_disabled()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let jobs = result.unwrap();
    assert_eq!(jobs.len(), 3);

    // Check first job (full)
    assert_eq!(jobs[0].job_id, "job-backup-daily");
    assert_eq!(jobs[0].name, "Daily Database Backup");
    assert_eq!(jobs[0].job_type, "backup");
    assert_eq!(jobs[0].schedule, "0 2 * * *");
    assert_eq!(jobs[0].enabled, Some(true));
    assert!(jobs[0].last_run.is_some());
    assert!(jobs[0].next_run.is_some());
    assert!(jobs[0].params.is_some());

    // Check second job (minimal)
    assert_eq!(jobs[1].job_id, "job-simple");
    assert_eq!(jobs[1].name, "Simple Job");
    assert_eq!(jobs[1].job_type, "maintenance");
    assert_eq!(jobs[1].schedule, "0 0 * * 0");
    assert!(jobs[1].enabled.is_none());
    assert!(jobs[1].last_run.is_none());
    assert!(jobs[1].next_run.is_none());
    assert!(jobs[1].params.is_none());

    // Check third job (disabled)
    assert_eq!(jobs[2].job_id, "job-disabled");
    assert_eq!(jobs[2].enabled, Some(false));
}

#[tokio::test]
async fn test_job_scheduler_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let jobs = result.unwrap();
    assert_eq!(jobs.len(), 0);
}

#[tokio::test]
async fn test_job_scheduler_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler/job-backup-daily"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_scheduled_job()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.get("job-backup-daily").await;

    assert!(result.is_ok());
    let job = result.unwrap();
    assert_eq!(job.job_id, "job-backup-daily");
    assert_eq!(job.name, "Daily Database Backup");
    assert_eq!(job.job_type, "backup");
    assert_eq!(job.schedule, "0 2 * * *");
    assert_eq!(job.enabled, Some(true));
    assert_eq!(job.last_run, Some("2023-01-01T02:00:00Z".to_string()));
    assert_eq!(job.next_run, Some("2023-01-02T02:00:00Z".to_string()));
    assert!(job.params.is_some());
}

#[tokio::test]
async fn test_job_scheduler_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler/job-simple"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_scheduled_job_minimal()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.get("job-simple").await;

    assert!(result.is_ok());
    let job = result.unwrap();
    assert_eq!(job.job_id, "job-simple");
    assert_eq!(job.name, "Simple Job");
    assert_eq!(job.job_type, "maintenance");
    assert_eq!(job.schedule, "0 0 * * 0");
    assert!(job.enabled.is_none());
    assert!(job.last_run.is_none());
    assert!(job.next_run.is_none());
    assert!(job.params.is_none());
}

#[tokio::test]
async fn test_job_scheduler_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Scheduled job not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_scheduler_create() {
    let mock_server = MockServer::start().await;

    let request = CreateScheduledJobRequest {
        name: "Weekly Report".to_string(),
        job_type: "report".to_string(),
        schedule: "0 9 * * 1".to_string(),
        enabled: Some(true),
        params: Some(json!({
            "report_type": "usage",
            "email_recipients": ["admin@company.com"]
        })),
    };

    Mock::given(method("POST"))
        .and(path("/v1/job_scheduler"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "job_id": "job-weekly-report",
            "name": "Weekly Report",
            "job_type": "report",
            "schedule": "0 9 * * 1",
            "enabled": true,
            "next_run": "2023-01-02T09:00:00Z",
            "params": {
                "report_type": "usage",
                "email_recipients": ["admin@company.com"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let job = result.unwrap();
    assert_eq!(job.job_id, "job-weekly-report");
    assert_eq!(job.name, "Weekly Report");
    assert_eq!(job.job_type, "report");
    assert_eq!(job.schedule, "0 9 * * 1");
    assert_eq!(job.enabled, Some(true));
    assert!(job.params.is_some());
}

#[tokio::test]
async fn test_job_scheduler_create_minimal() {
    let mock_server = MockServer::start().await;

    let request = CreateScheduledJobRequest {
        name: "Simple Task".to_string(),
        job_type: "cleanup".to_string(),
        schedule: "0 0 * * *".to_string(),
        enabled: None,
        params: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/job_scheduler"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "job_id": "job-simple-task",
            "name": "Simple Task",
            "job_type": "cleanup",
            "schedule": "0 0 * * *"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let job = result.unwrap();
    assert_eq!(job.job_id, "job-simple-task");
    assert_eq!(job.name, "Simple Task");
    assert_eq!(job.job_type, "cleanup");
    assert_eq!(job.schedule, "0 0 * * *");
    assert!(job.enabled.is_none());
    assert!(job.params.is_none());
}

#[tokio::test]
async fn test_job_scheduler_create_invalid() {
    let mock_server = MockServer::start().await;

    let request = CreateScheduledJobRequest {
        name: "Invalid Job".to_string(),
        job_type: "unknown".to_string(),
        schedule: "invalid cron".to_string(),
        enabled: Some(true),
        params: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/job_scheduler"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(400, "Invalid cron schedule"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_scheduler_update() {
    let mock_server = MockServer::start().await;

    let request = CreateScheduledJobRequest {
        name: "Updated Daily Backup".to_string(),
        job_type: "backup".to_string(),
        schedule: "0 3 * * *".to_string(),
        enabled: Some(false),
        params: Some(json!({
            "bdb_uid": 1,
            "backup_location": "/backups/updated",
            "retention_days": 14
        })),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/job_scheduler/job-backup-daily"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(success_response(json!({
            "job_id": "job-backup-daily",
            "name": "Updated Daily Backup",
            "job_type": "backup",
            "schedule": "0 3 * * *",
            "enabled": false,
            "last_run": "2023-01-01T02:00:00Z",
            "next_run": null,
            "params": {
                "bdb_uid": 1,
                "backup_location": "/backups/updated",
                "retention_days": 14
            }
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.update("job-backup-daily", request).await;

    assert!(result.is_ok());
    let job = result.unwrap();
    assert_eq!(job.job_id, "job-backup-daily");
    assert_eq!(job.name, "Updated Daily Backup");
    assert_eq!(job.schedule, "0 3 * * *");
    assert_eq!(job.enabled, Some(false));
    assert!(job.params.is_some());
}

#[tokio::test]
async fn test_job_scheduler_update_nonexistent() {
    let mock_server = MockServer::start().await;

    let request = CreateScheduledJobRequest {
        name: "Nonexistent".to_string(),
        job_type: "backup".to_string(),
        schedule: "0 0 * * *".to_string(),
        enabled: None,
        params: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/job_scheduler/nonexistent"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(404, "Scheduled job not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.update("nonexistent", request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_scheduler_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/job_scheduler/job-backup-daily"))
        .and(basic_auth("admin", "password"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.delete("job-backup-daily").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_job_scheduler_delete_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/job_scheduler/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Scheduled job not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.delete("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_scheduler_trigger() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/job_scheduler/job-backup-daily/trigger"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(created_response(test_job_execution_running()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.trigger("job-backup-daily").await;

    assert!(result.is_ok());
    let execution = result.unwrap();
    assert_eq!(execution.execution_id, "exec-123-abc");
    assert_eq!(execution.job_id, "job-backup-daily");
    assert_eq!(execution.start_time, "2023-01-01T02:00:00Z");
    assert!(execution.end_time.is_none());
    assert_eq!(execution.status, "running");
    assert!(execution.error.is_none());
}

#[tokio::test]
async fn test_job_scheduler_trigger_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/job_scheduler/nonexistent/trigger"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(error_response(404, "Scheduled job not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.trigger("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_scheduler_history() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler/job-backup-daily/history"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_job_execution_running(),
            test_job_execution_completed(),
            test_job_execution_failed()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.history("job-backup-daily").await;

    assert!(result.is_ok());
    let executions = result.unwrap();
    assert_eq!(executions.len(), 3);

    // Check running execution
    assert_eq!(executions[0].execution_id, "exec-123-abc");
    assert_eq!(executions[0].status, "running");
    assert!(executions[0].end_time.is_none());
    assert!(executions[0].error.is_none());

    // Check completed execution
    assert_eq!(executions[1].execution_id, "exec-456-def");
    assert_eq!(executions[1].status, "completed");
    assert!(executions[1].end_time.is_some());
    assert!(executions[1].error.is_none());

    // Check failed execution
    assert_eq!(executions[2].execution_id, "exec-789-ghi");
    assert_eq!(executions[2].status, "failed");
    assert!(executions[2].end_time.is_some());
    assert_eq!(
        executions[2].error,
        Some("Insufficient disk space for backup".to_string())
    );
}

#[tokio::test]
async fn test_job_scheduler_history_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler/job-simple/history"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.history("job-simple").await;

    assert!(result.is_ok());
    let executions = result.unwrap();
    assert_eq!(executions.len(), 0);
}

#[tokio::test]
async fn test_job_scheduler_history_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/job_scheduler/nonexistent/history"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Scheduled job not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JobSchedulerHandler::new(client);
    let result = handler.history("nonexistent").await;

    assert!(result.is_err());
}
