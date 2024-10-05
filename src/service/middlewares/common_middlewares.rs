use crate::service::app::responder::ApiResponse;

// 404 error handler
pub async fn not_found_handler() -> ApiResponse<String> {
    ApiResponse {
        success: false,
        message: "404 not found".to_owned(),
        data: None,
        status: Some(404)
    }
}
