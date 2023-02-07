use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use serde::Deserialize;


use exitfailure::ExitFailure;
use std::thread;

use rust_bert::bart::{
    BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
};
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::resources::RemoteResource;
use tch::Device;


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Deserialize)]
struct Info {
    context: String,
}


#[get("/api/health")]
async fn api_health_handler() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/")]
async fn api_health_handler2() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


use std::sync::Once;
use actix_web::rt::Runtime;

static mut SUMMARIZATION_MODEL: Option<SummarizationModel> = None;

static INIT_MODEL: Once = Once::new();

fn init_summarization_model() -> SummarizationModel {
    let do_steps = move || -> Result<SummarizationModel, ExitFailure> {
        let config_resource = Box::new(RemoteResource::from_pretrained(
            BartConfigResources::DISTILBART_CNN_6_6,
        ));
        let vocab_resource = Box::new(RemoteResource::from_pretrained(
            BartVocabResources::DISTILBART_CNN_6_6,
        ));
        let merges_resource = Box::new(RemoteResource::from_pretrained(
            BartMergesResources::DISTILBART_CNN_6_6,
        ));
        let model_resource = Box::new(RemoteResource::from_pretrained(
            BartModelResources::DISTILBART_CNN_6_6,
        ));

        let summarization_config = SummarizationConfig {
            model_resource,
            config_resource,
            vocab_resource,
            merges_resource: Some(merges_resource),
            num_beams: 1,
            length_penalty: 1.0,
            min_length: 56,
            max_length: Some(142),
            device: Device::Cpu,
            ..Default::default()
        };

        let summarization_model = SummarizationModel::new(summarization_config)?;
        Ok(summarization_model)
    };

    let summarization_model = thread::spawn(move || {
        do_steps().unwrap()
        // match do_steps() {
        //     Ok(report) => {
        //         report
        //     }
        //     Err(err) => {
        //         // "error".to_string()
        //
        //         // or write a better logic
        //     }
        // }
    }).join().expect("Thread panicked");
    return summarization_model
    // do_steps().unwrap()
}


// #[get("/api/summary")]
// async fn api_summary_handler(info: web::Query<Info>) -> impl Responder {
//     // INIT_MODEL.call_once(|| {
//     //     unsafe {
//     //         SUMMARIZATION_MODEL = Some(init_summarization_model());
//     //     }
//     // });
//     let summarization_model = unsafe { SUMMARIZATION_MODEL.as_ref().unwrap() };
//
//     let mut input = [String::new(); 1];
//     input[0] = info.context.to_owned();
//
//     let _output = summarization_model.summarize(&input);
//     let mut result = String::new();
//     result = _output.join(" ");
//     let response_json = &GenericResponse {
//         status: "success".to_string(),
//         message: result.to_string(),
//     };
//     HttpResponse::Ok().json(response_json)
// }

#[get("/api/summary")]
async fn api_summary_handler(info: web::Query<Info>) -> impl Responder {



    let summarization_model =init_summarization_model();

    let mut input = [String::new(); 1];
    input[0] = info.context.to_owned();

    let _output = summarization_model.summarize(&input);
    let mut result = String::from(_output.join(" "));
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: result.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // let mut rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     init_summarization_model();
    //     drop(rt); // this line causes the error
    // });
    INIT_MODEL.call_once(|| {
        let model = init_summarization_model();
        unsafe { SUMMARIZATION_MODEL = Some(model) }
    });


    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(api_summary_handler)
            .service(api_health_handler)
            .service(api_health_handler2)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;
    Ok(())
}

