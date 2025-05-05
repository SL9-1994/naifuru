// このモジュールは，Extractorで共通の処理やTrait等が置かれています
use crate::{
    analysis_config::{ConversionConfig, FromType},
    error::{AppError, ProcessErr},
};

use super::tw_paleart_sac::TwPalertSacExtractor;

pub trait Extractor {
    fn extract(&self) -> Result<ExtractedData, Vec<AppError>>;

    fn extract_latitude(&self) -> Result<f64, ProcessErr>;

    fn extract_longitude(&self) -> Result<f64, ProcessErr>;

    fn extract_unit_type(&self) -> Result<String, ProcessErr>;

    fn extract_acc_values(&self) -> Result<Acceleration, ProcessErr>;

    fn extract_initial_time(&self) -> Result<String, ProcessErr>;
}

pub fn create_extractor<'a>(conversion: &'a ConversionConfig) -> Box<dyn Extractor + 'a> {
    // fromに対応するextractorを呼び出す
    match conversion.from {
        FromType::JpNiedKnet => todo!(),
        FromType::UsScsnV2 => todo!(),
        FromType::NzGeonetV1a => todo!(),
        FromType::NzGeonetV2a => todo!(),
        FromType::TwPalertSac => Box::new(TwPalertSacExtractor::new(conversion)),
        FromType::TkAfadAsc => todo!(),
    }
}

pub enum ExtractedData {
    JpStera3dTxt(JpStera3dTxtData),
    JpJmaCsv(JpJmaCsvData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct JpStera3dTxtData {
    num_of_elements: u32,
    common: CommonValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JpJmaCsvData {
    site_code: String,
    lat: f64,
    lon: f64,
    unit_type: String,
    initial_time: String,
    common: CommonValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommonValue {
    sampling_rate: Option<u32>,
    delta_t: Option<f32>,
    acc_values: Acceleration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Acceleration {
    ns: Vec<f64>,
    ew: Vec<f64>,
    ud: Vec<f64>,
}
