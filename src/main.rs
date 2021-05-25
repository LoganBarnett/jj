// Needed to make future magic work, I guess.
use futures::TryFutureExt;

mod cli;
mod config;
mod jenkins;
mod logging;
mod error;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let config = config::config_load()
        .and_then(cli::cli_validate)?;
    // This gives us something like this:
    // https://jenkins.foo/queue/item/590249/
    //
    // https://support.cloudbees.com/hc/en-us/articles/360028147532-Get-Build-Number-with-REST-API?page=27
    // The above documentation states that the queue item should be around for 5
    // minutes. We can use that to query to see which build it has produced, and
    // then use that to poll/watch the build log.
    jenkins::build_enqueue(&config)
        .and_then(|url| jenkins::build_queue_item_poll(&config, url))
        .and_then(|url| jenkins::build_log_stream(&config, url))
        .await
        .and_then(|()| {
            println!("Done!");
            Ok(())
        })
        // .and_then(|s| {
        //     println!("Done! {:?}", s);
        //     Ok(())
        // })
}
