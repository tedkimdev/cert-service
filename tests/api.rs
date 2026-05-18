mod helpers;

use helpers::TestApp;

// ============================================================
// No DB required tests
// ============================================================

#[tokio::test]
async fn test_health_live() {
    let app = TestApp::new().await;
    let response = app.get("/health/live").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_certificate_invalid_uuid() {
    let app = TestApp::new().await;
    let response = app.get("/certificates/invalid-uuid").await;
    assert_eq!(response.status(), 400);
}

// ============================================================
// DB integration tests
// TODO: Run these with a test database
// ============================================================
