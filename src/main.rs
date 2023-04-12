use lambda_http::{
    aws_lambda_events::serde_json::json, run, service_fn, Body, Error, IntoResponse, Request,
    Response,
};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

fn get_random_chess960_board() -> &'static Vec<String> {
    lazy_static! {
        static ref ALL_BOARDS: Vec<Vec<String>> = project4::generate_all_chess960_boards();
    }
    let num_board = ALL_BOARDS.len(); // equals to 960 here
    let board_index = thread_rng().gen_range(0..num_board);
    &ALL_BOARDS[board_index]
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_func = |_event: Request| async move {
        let board = get_random_chess960_board();
        let response = build_success_response(board).await;
        Result::<Response<Body>, Error>::Ok(response)
    };
    run(service_fn(handler_func)).await?;
    Ok(())
}

async fn build_success_response(board: &'static Vec<String>) -> Response<Body> {
    json!({ "Start Board": board }).into_response().await
}

async fn build_failure_response(error_message: &str) -> Response<Body> {
    Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "error": error_message }).to_string()))
        .expect("could not build the error response")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_boards() {
        assert_eq!(project4::generate_all_chess960_boards().len(), 960);
    }

    #[tokio::test]
    async fn build_success_response_test() {
        let test_board = get_random_chess960_board();
        let result = build_success_response(&test_board).await;
        let (parts, body) = result.into_parts();
        assert_eq!(200, parts.status.as_u16());
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        let board_string = format!("{:?}", test_board);
        let board_string_trimmed = board_string.replace(", ", ",");
        assert_eq!(
            format!("{{\"start board\":{}}}", board_string_trimmed),
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        );
    }

    #[tokio::test]
    async fn build_failure_response_test() {
        let result = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();
        assert_eq!(400, parts.status.as_u16());
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"error\":\"test error message\"}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        );
    }
}
