use crate::service::app::responder::ApiResponse;

pub async fn user_list() -> ApiResponse<String> {
    ApiResponse {
        success: false,
        message: "list of users".to_string(),
        data: None,
        status: Some(200),
    }
}
