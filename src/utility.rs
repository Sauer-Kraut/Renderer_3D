use crate::calc_structs::*;
use crate::render_lib::*;
use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn generate_wave_source(config: WaveSourceConfig) -> Result<DynObject, Box<dyn std::error::Error>> {

    let mut entrys: Vec<(
        Model<'static>, 
        Vector3D, 
        Arc<Mutex<dyn Fn(Model<'static>, Vector3D, f32) -> (Model<'static>, Vector3D) + 'static>>
    )> = Vec::new();

    let mut source = Model::import_obj("static/assets/models/Sphere/Sphere.obj")?;
    source.scale(config.source_size);
    let wave = Model::import_obj("static/assets/models/Donut/Donut.obj")?;

    let occurences = (1.0 as f32 * config.frequency) as u32;

    let source_entry = (
        source.clone(),
        config.source_start,
        config.src_mov_fn.clone()
    );

    let obs_entry = (
        source.clone(),
        config.observer_start,
        config.obs_mov_fn.clone()
    );

    entrys.push(source_entry);
    entrys.push(obs_entry);

    for wavefront_index in 1..(occurences +1) {

        let start_duration = wavefront_index as f32 / config.frequency;

        let mov = config.src_mov_fn.clone();
        let (_new_model, start_position) = mov.lock().await(source.clone(), config.source_start, start_duration);

        let dev_fn: Arc<Mutex<dyn Fn(Model<'static>, Vector3D, f32) -> (Model<'static>, Vector3D) + 'static>> = 
            Arc::new(Mutex::new(move |mut wave_model: Model<'static>, starting_position: Vector3D, time_stamp: f32| {
                let start_duration = wavefront_index as f32 / config.frequency;
                if start_duration < time_stamp {
                    wave_model.scale((start_duration - time_stamp) * config.wave_speead);
                    return (
                        wave_model, 
                        starting_position,
                    );
                } else {
                    return (
                        Model::new(vec![], "none".to_owned()), 
                        Vector3D::origin(),
                    );
                }
            }));

        let wave_entry = (
            wave.clone(),
            start_position,
            dev_fn.clone(),
        );

        entrys.push(wave_entry);
    }

    Ok(DynObject::new(entrys, Vector3D::origin()))
}


#[derive(Clone)]
pub struct WaveSourceConfig {
    pub wave_speead: f32, 
    pub frequency: f32,
    pub source_start: Vector3D, 
    pub observer_start: Vector3D, 
    pub src_mov_fn: Arc<Mutex<dyn Fn(Model<'static>, Vector3D, f32) -> (Model<'static>, Vector3D)>>,
    pub obs_mov_fn: Arc<Mutex<dyn Fn(Model<'static>, Vector3D, f32) -> (Model<'static>, Vector3D)>>,
    pub source_size: f32
}