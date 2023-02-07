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



use std::sync::Once;
use actix_web::rt::Runtime;

static mut SUMMARIZATION_MODEL: Option<SummarizationModel> = None;

static INIT_MODEL: Once = Once::new();

pub fn init_summarization_model() -> SummarizationModel {
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

    }).join().expect("Thread panicked");
    return summarization_model
}

