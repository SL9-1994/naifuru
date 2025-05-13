// このモジュールは，Extractorで共通の処理やTrait等が置かれています
use crate::{
    analysis_config::FromType,
    error::{AppError, ProcessErr},
    model::ir::{FormatMetadata, Nvhdr, ProcessableFile, SeismicIr},
};

use super::tw_paleart_sac::TwPalertSacExtractor;

pub trait Extractor {
    fn extract(&self) -> Result<SeismicIr, AppError>;

    fn extract_latitude(&self) -> Result<f64, ProcessErr>;

    fn extract_longitude(&self) -> Result<f64, ProcessErr>;

    fn extract_unit_type(&self) -> Result<String, ProcessErr>;

    fn extract_acc_values(&self) -> Result<Acceleration, ProcessErr>;

    fn extract_initial_time(&self) -> Result<String, ProcessErr>;
}

pub fn create_extractor(conversion: ProcessableFile) -> Box<dyn Extractor> {
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

// pub enum ExtractedData {
//     JpStera3dTxt(JpStera3dTxtData),
//     JpJmaCsv(JpJmaCsvData),
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct JpStera3dTxtData {
//     num_of_elements: u32,
//     common: CommonValue,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct JpJmaCsvData {
//     site_code: String,
//     lat: f64,
//     lon: f64,
//     unit_type: String,
//     initial_time: String,
//     common: CommonValue,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct CommonValue {
//     sampling_rate: Option<u32>,
//     delta_t: Option<f32>,
//     acc_values: Acceleration,
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Acceleration {
    ns: Vec<f64>,
    ew: Vec<f64>,
    ud: Vec<f64>,
}

// モックデータ生成用
pub fn mock_seismic_ir_data() -> SeismicIr {
    SeismicIr {
        num_of_elements: 3,
        timestamp: "2025-05-06T12:34:56Z".to_string(),
        acc_values: Acceleration {
            ns: vec![0.1, 0.2, 0.3],
            ew: vec![0.0, 0.1, 0.2],
            ud: vec![-0.1, -0.2, -0.3],
        },
        source_metadata: FormatMetadata {
            unit_type: "cm/s^2".to_string(),
            nvhdr: Some(Nvhdr::Ver6),
            delta_t: Some(0.01),
            sampling_rate: Some(100),
            site_code: "W339".to_string(),
            lat: 24.123456,
            lon: 121.654321,
            ad_coefficients: 1.0,
        },
    }
}
