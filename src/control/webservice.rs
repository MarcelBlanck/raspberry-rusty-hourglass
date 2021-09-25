use crate::hourglass::{HourglassState};
use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder, rt::System};
use actix_web::dev::Server;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::sync::{Arc, RwLock};
use std::thread;
use actix_files::Files;

type ThreadSafeHourglassState = Arc<RwLock<HourglassState<bool, isize, bool>>>;

pub fn start_webservice(state: ThreadSafeHourglassState) -> Server {
    let (control_extraction_tx, control_extraction_rx) = unbounded::<Server>();

    thread::spawn(move || {
        let sys = System::new("http-server");
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/", web::get().to(index))
                .route("/start", web::get().to(start))
                .route("/stop", web::get().to(stop))
                .route("/minus_minute", web::get().to(minus_minute))
                .route("/plus_minute", web::get().to(plus_minute))
                .route("/get_ticking", web::get().to(get_ticking))
                .route("/get_remaining_seconds", web::get().to(get_remaining_seconds))
                .route("/set_remaining_seconds/{seconds}", web::get().to(set_remaining_seconds))
                .route("/end_service", web::get().to(end_service))
                .service(Files::new("/", "html/"))
        })
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run();

        control_extraction_tx.send(server).unwrap();
        sys.run()
    });

    control_extraction_rx.recv().unwrap()
}

async fn index(_data: web::Data<ThreadSafeHourglassState>) -> HttpResponse {
    HttpResponse::Found()
        .header("LOCATION", "/index.html")
        .finish()
}

async fn start(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    data.write().unwrap().ticking = true;
    format!("Started.")
}

async fn stop(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    data.write().unwrap().ticking = false;
    format!("Stopped.")
}

async fn plus_minute(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    data.write().unwrap().remaining_seconds += 60;
    format!("Minute added.")
}

async fn minus_minute(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    data.write().unwrap().remaining_seconds -= 60;
    format!("Minute subtracted.")
}

async fn get_ticking(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", if data.read().unwrap().ticking { "true" } else { "false"})
}

async fn get_remaining_seconds(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", data.read().unwrap().remaining_seconds)
}

async fn set_remaining_seconds(req: HttpRequest, data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("called set rem");
    let seconds = req.match_info().get("seconds");
    if seconds.is_some() {
        match seconds.unwrap().parse::<isize>() {
            Ok(value) => {
                data.write().unwrap().remaining_seconds = value;
                format!("Set remaining seconds to {}", value)
            },
            Err(_) => {
                format!("Remaining seconds not changed, wrong argument. Try a positive number.")
            }
        }
    } else {
        format!("Remaining seconds not changed, missing argument.")
    }
}

async fn end_service(data: web::Data<ThreadSafeHourglassState> )-> impl Responder {
    data.write().unwrap().finalize = true;
    format!("Webservice teared down.")
}
