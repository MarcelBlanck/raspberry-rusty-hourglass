use crate::hourglass::{HourglassState, ThreadSafeHourglassState, MAXIMUM_DURATION_MS};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{rt::System, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::SystemTime;

pub fn start_webservice(state: ThreadSafeHourglassState) {
    thread::spawn(move || {
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/", web::get().to(index))
                .route("/start", web::get().to(start))
                .route("/stop", web::get().to(stop))
                .route("/minus_minute", web::get().to(minus_minute))
                .route("/plus_minute", web::get().to(plus_minute))
                .route("/get_ticking", web::get().to(get_ticking))
                .route("/get_duration_ms", web::get().to(get_duration_ms))
                .route(
                    "/set_duration_ms/{duration_ms}",
                    web::get().to(set_duration_ms),
                )
                .route("/get_target_time_ms", web::get().to(get_target_time_ms))
                .route("/end_service", web::get().to(end_service))
                .service(Files::new("/", "./html/"))
        })
        .bind((r#"127.0.0.1"#, 8080))
        .unwrap()
        .run();

        let _ = System::new().block_on(server);
    });
}

fn start_hourglass_timer(data: &web::Data<ThreadSafeHourglassState>) {
    let mut data_unlocked_rw = data.write().unwrap();
    let current_time_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    data_unlocked_rw.target_time_ms = current_time_ms + data_unlocked_rw.duration_ms;
    data_unlocked_rw.ticking = true;
}

fn stop_hourglass_timer(data: &web::Data<ThreadSafeHourglassState>) {
    let mut data_unlocked_rw = data.write().unwrap();
    data_unlocked_rw.target_time_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    data_unlocked_rw.duration_ms = 0;
    data_unlocked_rw.ticking = false;
}

async fn index(_data: web::Data<ThreadSafeHourglassState>) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("LOCATION", "/index.html"))
        .finish()
}

async fn start(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    start_hourglass_timer(&data);
    "Started.".to_string()
}

async fn stop(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    stop_hourglass_timer(&data);
    "Stopped.".to_string()
}

async fn plus_minute(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    {
        let mut data_unlocked_rw = data.write().unwrap();
        data_unlocked_rw.duration_ms = data_unlocked_rw
            .duration_ms
            .checked_add(60000)
            .unwrap_or(MAXIMUM_DURATION_MS)
            .clamp(0, MAXIMUM_DURATION_MS);
    }
    start_hourglass_timer(&data);
    "Minute added.".to_string()
}

async fn minus_minute(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    let mut data_unlocked_rw = data.write().unwrap();
    let decremented_duration_ms = data_unlocked_rw.duration_ms.saturating_sub(60000);
    if data_unlocked_rw.ticking {
        let subtracted_ms = data_unlocked_rw.duration_ms - decremented_duration_ms;
        data_unlocked_rw.target_time_ms -= subtracted_ms;
    }
    data_unlocked_rw.duration_ms = decremented_duration_ms;
    "Minute subtracted.".to_string()
}

async fn get_ticking(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{:?}", data.read().unwrap().ticking)
}

async fn get_target_time_ms(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", data.read().unwrap().target_time_ms)
}

async fn get_duration_ms(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", data.read().unwrap().duration_ms)
}

async fn set_duration_ms(
    req: HttpRequest,
    data: web::Data<ThreadSafeHourglassState>,
) -> impl Responder {
    let duration_ms = req.match_info().get("duration_ms");
    if duration_ms.is_some() {
        match duration_ms.unwrap().parse::<u128>() {
            Ok(duration_ms) => {
                let mut data_unlocked_rw = data.write().unwrap();
                data_unlocked_rw.duration_ms = duration_ms.clamp(0, MAXIMUM_DURATION_MS);
            }
            Err(error) => {
                return format!("Error: Unable to parse duration in ms. {:?}", error);
            }
        };
        format!("Setting duration to {}ms.", duration_ms.unwrap())
    } else {
        "Error: No duration in ms was given.".to_string()
    }
}

async fn end_service(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    data.write().unwrap().finalize = true;
    "Webservice teared down.".to_string()
}
