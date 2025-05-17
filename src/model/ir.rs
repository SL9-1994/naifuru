use std::path::PathBuf;

use crate::{
    analysis_config::{AccAxis, FromType, TextOrBinary, ToType},
    extractor::extractor::Acceleration,
};

// 共通中間構造体（IR）（Extractor→Processor→Converterまで渡す）
#[derive(Debug)]
pub struct SeismicIr {
    pub num_of_elements: u32,
    pub timestamp: String,
    pub acc_values: Acceleration,
    pub source_metadata: FormatMetadata,
}

// 非共通のメタデータ
#[derive(Debug)]
pub struct FormatMetadata {
    pub unit_type: String,
    pub nvhdr: Option<Nvhdr>, // .sac only
    pub delta_t: Option<f32>,
    pub sampling_rate: Option<u32>,
    pub site_code: String,
    pub lat: f64,
    pub lon: f64,
    pub ad_coefficients: f64, // スケールファクタもこのフィールドに入ります．
}

#[derive(Debug)]
pub enum Nvhdr {
    Ver6,
    Ver7,
}

// AnalysisConfigからExtractor迄の処理用構造体
#[derive(Debug)]
pub struct ProcessableFile {
    pub conv_name: String,
    pub from: FromType,
    pub to: ToType,
    pub group_index: usize,
    pub file_index: usize,
    pub acc_axis: Option<AccAxis>,
    pub path: PathBuf,
    pub data: Option<TextOrBinary>,
}
