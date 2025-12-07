#[macro_export]
macro_rules! launch_service {
    (
        service: $service:expr,
        routes: [ $( $route:expr ),* $(,)? ]
    ) => {{
        use actix_web::{App, HttpServer, web};
        use $crate::{config::Config, app_data::AppData};

        let config = Config::load($service);
        let app_data = AppData::new(config.clone()).await?;
        let addr = format!("{}:{}", $service, config.port);

        println!("Now listening: {addr}");
        HttpServer::new(move || {
            let mut app = App::new().app_data(web::Data::new(app_data.clone()));
            $( app = app.service($route); )*
            app
        })
        .bind(&addr)?
        .run()
        .await?;
    }};
}
