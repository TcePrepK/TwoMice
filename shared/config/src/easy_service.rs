#[macro_export]
macro_rules! launch_service {
    (
        service: $service:expr,
        routes: [ $( $route:expr ),* $(,)? ]
    ) => {{
        use actix_web::{App, HttpServer, web};
        use $crate::{config::Config, app_data::AppData};
        use env_logger::Env;

        let config = Config::load($service);
        let app_data = AppData::new(config.clone()).await?;
        let global_addr = format!("0.0.0.0:{}", config.port);

        // Set up the logger
        let filter = match config.app_env.as_str() {
            "prod" => "warn,axum=info,tower_http=info",
            // "staging" => "info,axum=info,tower_http=debug", << We might need later on
            _ => "debug", // dev
        };
        env_logger::init_from_env(Env::default().default_filter_or(filter));

        let local_addr = format!("http://{}-service:{}/", $service, config.port);
        HttpServer::new(move || {
            let mut app = App::new().app_data(web::Data::new(app_data.clone()));
            $( app = app.service($route); )*
            app
        })
        .bind(&global_addr)?
        .run()
        .await?;
    }};
}
