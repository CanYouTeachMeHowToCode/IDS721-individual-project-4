use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let board = get_random_chess960_board();
    // let response = build_success_response(board).await;
    let response = Response {
        req_id: event.context.request_id,
        msg: board.join(", "),
    };
    Result::<Response, Error>::Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

fn get_random_chess960_board() -> &'static Vec<String> {
    lazy_static! {
        static ref ALL_BOARDS: Vec<Vec<String>> = project4::generate_all_chess960_boards();
    }
    let num_board = ALL_BOARDS.len(); // equals to 960 here
    let board_index = thread_rng().gen_range(0..num_board);
    &ALL_BOARDS[board_index]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_num_boards() {
        assert_eq!(project4::generate_all_chess960_boards().len(), 960);
    }
}
