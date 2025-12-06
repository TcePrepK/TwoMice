#[macro_export]
macro_rules! launch_service {
    (
        routes: [ $( $route:expr ),* $(,)? ]
    ) => {{
        use actix_web::{App, HttpServer, web};
        use $crate::{config::Config, app_data::AppData};

        #[cfg(debug_assertions)]
        Config::load_local_env();

        let config = Config::load($service);
        let app_data = AppData::new(config.clone()).await?;
        let addr = format!("0.0.0.0:{}", config.port);

        HttpServer::new(move || {
            let mut app = App::new().app_data(web::Data::new(app_data.clone()));
            $( app = app.service($route); )*
            app
        })
        .bind(&addr)?
        .run()
        .await?;
    }};

    (
        service: $service:expr,
        routes: [ $( $route:expr ),* $(,)? ]
    ) => {{
        use actix_web::{App, HttpServer, web};
        use $crate::{config::Config, app_data::AppData};

        #[cfg(debug_assertions)]
        Config::load_local_env();

        let config = Config::load($service);
        let app_data = AppData::new(config.clone()).await?;
        let addr = format!("0.0.0.0:{}", config.port);

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
