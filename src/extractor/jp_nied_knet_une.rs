use crate::{
    analysis_config::TextOrBinary,
    error::{AppError, DataExtractionErr, ProcessErr},
    model::ir::{ProcessableFile, SeismicIr},
    regex_pattern::RE_SCALE_FACTOR,
};

use super::extractor::{Acceleration, Extractor, mock_seismic_ir_data};

pub struct JpNiedKnetUneExtractor {
    pub unextracted: ProcessableFile,
}

impl Extractor for JpNiedKnetUneExtractor {
    fn extract(&self) -> Result<SeismicIr, AppError> {
        let sf = self.extract_ad_scale_factor()?;

        println!("{}", sf);

        Ok(mock_seismic_ir_data())
    }

    fn extract_latitude(&self) -> Result<f64, ProcessErr> {
        todo!()
    }

    fn extract_longitude(&self) -> Result<f64, ProcessErr> {
        todo!()
    }

    fn extract_unit_type(&self) -> Result<String, ProcessErr> {
        todo!()
    }

    fn extract_acc_values(&self) -> Result<Acceleration, ProcessErr> {
        todo!()
    }

    fn extract_initial_time(&self) -> Result<String, ProcessErr> {
        todo!()
    }
}

impl JpNiedKnetUneExtractor {
    pub fn new(unextracted: ProcessableFile) -> Self {
        Self { unextracted }
    }

    fn extract_ad_scale_factor(&self) -> Result<f64, ProcessErr> {
        todo!()
    }
}

// A/D値とスケールファクタとの積を計算し，加速度値を求める
/// K-net, Kik-net, ASCII type only.
fn _to_acceleration_using_scale_factor(scale_factor: f64, ad_values: Vec<f64>) -> Vec<f64> {
    let accs = ad_values.iter().map(|acc| acc * scale_factor).collect();
    accs
}

// 計算量削減のため，先にscale factorの分数を計算する
fn calculate_scale_factor(numerator: u64, denominator: u64) -> f64 {
    (numerator as f64 / denominator as f64)
}
