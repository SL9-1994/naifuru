use crate::{
    error::{AppError, ProcessErr},
    model::ir::{ProcessableFile, SeismicIr},
};

use super::extractor::Extractor;

pub struct JpNiedKnetUneExtractor {
    pub unextracted: ProcessableFile,
}

impl Extractor for JpNiedKnetUneExtractor {
    fn extract(&self) -> Result<SeismicIr, AppError> {
        todo!()
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

    fn extract_acc_values(&self) -> Result<super::extractor::Acceleration, ProcessErr> {
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
}

// 計算量削減のため，先にscale factorの分数を計算する
fn _calculate_scale_factor(numerator: u64, denominator: u64) -> f64 {
    (numerator / denominator) as f64
}
