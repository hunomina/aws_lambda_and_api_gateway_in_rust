mod io;
mod aws;

use lambda_runtime::{self, handler_fn, Context, Error};
use aws::gateway;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(my_handler)).await?;
    Ok(())
}

async fn my_handler(request: gateway::Request, ctx: Context) -> Result<gateway::Response, Error> {
    println!("{:?}", request);
    println!("{:?}", ctx);

    let response = io::Response {
        req_id: ctx.request_id,
        msg: format!(
            "Hello {}!",
            request
                .body::<io::UserInfo>()
                .unwrap()
                .firstName
                .unwrap_or("World".to_string())
        ),
    };

    Ok(gateway::Response::from(response))
}
