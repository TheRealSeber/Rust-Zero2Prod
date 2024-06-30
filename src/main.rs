use zero2prod::configurations::load_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = load_configuration().expect("Failed to read configuration file.");
    let application = Application::build_server(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
