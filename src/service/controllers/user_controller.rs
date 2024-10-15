use crate::service::app::{responder::ApiResponse, connection::Database};




    pub async fn user_list () -> ApiResponse<String>{
        // let users = self::Database.init().await?;
        // let users_list = users.find();
        ApiResponse{
            success: true,
            message: "user_list".to_string(),
            data: None,
            status: Some(201)
        }
    }





pub async fn create_user () -> ApiResponse<String>{
    ApiResponse{
        success: true,
        message: "user_list".to_string(),
        data: None,
        status: Some(201)
    }
}