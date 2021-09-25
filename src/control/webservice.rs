use crate::hourglass::{HourglassState, ThreadSafeHourglassState};
use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder, rt::System};
use actix_web::dev::Server;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::SystemTime;
use actix_files::Files;


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
                .route("/get_target_time_ms", web::get().to(get_target_time_ms))
                .route("/set_target_time_ms/{unix_time}", web::get().to(set_target_time_ms))
                .route("/get_duration_ms", web::get().to(get_duration_ms))
                .route("/set_duration_ms/{duration_ms}", web::get().to(set_duration_ms))
                .route("/end_service", web::get().to(end_service))
                .service(Files::new("/", "./html/"))
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
    let mut data_unlocked_rw = data.write().unwrap();
    data_unlocked_rw.target_time_ms
        = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
          + data_unlocked_rw.duration_ms;
          data_unlocked_rw.ticking = true;
    format!("Started.")
}

async fn stop(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    let mut data_unlocked_rw = data.write().unwrap();
    data_unlocked_rw.ticking = false;
    format!("Stopped.")
}

async fn plus_minute(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    let mut data_unlocked_rw = data.write().unwrap();
    data_unlocked_rw.duration_ms += 60000;
    if data_unlocked_rw.ticking {
        data_unlocked_rw.target_time_ms += 60000;
    }
    format!("Minute added.")
}

async fn minus_minute(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    let mut data_unlocked_rw = data.write().unwrap();
    data_unlocked_rw.duration_ms -= 60000; // TODO avoid underflow
    if data_unlocked_rw.ticking {
        data_unlocked_rw.target_time_ms -= 60000;
    }
    format!("Minute subtracted.")
}

async fn get_ticking(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", if data.read().unwrap().ticking { "true" } else { "false"})
}

async fn get_target_time_ms(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", data.read().unwrap().target_time_ms)
}

async fn set_target_time_ms(req: HttpRequest, data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    let unix_time = req.match_info().get("unix_time");
    if unix_time.is_some() {
        let mut data_unlocked_rw = data.write().unwrap();
        data_unlocked_rw.target_time_ms = u128::from_str_radix(unix_time.unwrap(), 10).unwrap();
        format!("Setting target time to {}ms", unix_time.unwrap())
    } else {
        format!("Error: No ms since epoch unix time was given.")
    }
}

async fn get_duration_ms(data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    format!("{}", data.read().unwrap().duration_ms)
}

async fn set_duration_ms(req: HttpRequest, data: web::Data<ThreadSafeHourglassState>) -> impl Responder {
    let duration_ms = req.match_info().get("duration_ms");
    if duration_ms.is_some() {
        let mut data_unlocked_rw = data.write().unwrap();
        data_unlocked_rw.duration_ms = u128::from_str_radix(duration_ms.unwrap(), 10).unwrap();
        format!("Setting duration to {}ms.", duration_ms.unwrap())
    } else {
        format!("Error: No duration in ms was given.")
    }
}

async fn end_service(data: web::Data<ThreadSafeHourglassState> )-> impl Responder {
    data.write().unwrap().finalize = true;
    format!("Webservice teared down.")
}
