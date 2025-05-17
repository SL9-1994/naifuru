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
        // 複数ファイルの抽出関数切り替え(必要のない抽出をスキップ)
        // nameをidとして使用>group_idx>file_idx
        // name(g_idx=n, f_idx=1)の時はheader情報を取得
        if self.unextracted.file_index == 0 {
            // ヘッダー情報＋加速度値を抽出
            let _sf = self.extract_ad_scale_factor()?;
            let _lat = self.extract_latitude()?;
            let _lon = self.extract_longitude()?;
            // headerありでIrを組み立て
        } else {
            // 2つ目以降は加速度値のみ抽出
            // Optionでheaderの無いIrを組み立て
        }

        Ok(mock_seismic_ir_data())
    }

    fn extract_latitude(&self) -> Result<f64, ProcessErr> {
        match &self.unextracted.data {
            Some(TextOrBinary::Text(txt_data)) => {
                let lat_line = txt_data.get(1).ok_or_else(|| {
                    DataExtractionErr::MissingFileData(
                        "Line 2".to_string(),
                        self.unextracted.path.clone(),
                    )
                })?;

                let mut lat_line_iter = lat_line.split_whitespace();

                if let Some(element) = lat_line_iter.nth(1) {
                    let latitude = element.parse::<f64>().map_err(|_| {
                        DataExtractionErr::PatternNotMatched(
                            "latitude".to_string(),
                            self.unextracted.path.clone(),
                        )
                    })?;
                    Ok(latitude)
                } else {
                    Err(DataExtractionErr::MissingFileData(
                        "latitude".to_string(),
                        self.unextracted.path.clone(),
                    ))?
                }
            }
            Some(TextOrBinary::Binary(_bin_data)) => unimplemented!(),
            None => unreachable!(),
        }
    }

    fn extract_longitude(&self) -> Result<f64, ProcessErr> {
        match &self.unextracted.data {
            Some(TextOrBinary::Text(txt_data)) => {
                let lat_line = txt_data.get(2).ok_or_else(|| {
                    DataExtractionErr::MissingFileData(
                        "Line 3".to_string(),
                        self.unextracted.path.clone(),
                    )
                })?;

                let mut lon_line_iter = lat_line.split_whitespace();

                if let Some(element) = lon_line_iter.nth(1) {
                    let longitude = element.parse::<f64>().map_err(|_| {
                        DataExtractionErr::PatternNotMatched(
                            "longitude".to_string(),
                            self.unextracted.path.clone(),
                        )
                    })?;
                    Ok(longitude)
                } else {
                    Err(DataExtractionErr::MissingFileData(
                        "longitude".to_string(),
                        self.unextracted.path.clone(),
                    ))?
                }
            }
            Some(TextOrBinary::Binary(_bin_data)) => unimplemented!(),
            None => unreachable!(),
        }
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
        match &self.unextracted.data {
            Some(TextOrBinary::Text(txt_data)) => {
                let unprocessed_scale_factor = txt_data.get(13).ok_or_else(|| {
                    DataExtractionErr::MissingFileData(
                        "Line 14".to_string(),
                        self.unextracted.path.clone(),
                    )
                })?;

                let mut split_sf_iter = unprocessed_scale_factor.split_whitespace();

                if let Some(element) = split_sf_iter.nth(2) {
                    if let Some(caps) = RE_SCALE_FACTOR.captures(element) {
                        let numerator = caps
                            .name("numerator")
                            .and_then(|m| m.as_str().parse::<u64>().ok())
                            .ok_or_else(|| {
                                DataExtractionErr::MissingFileData(
                                    "scale factor numerator".to_string(),
                                    self.unextracted.path.clone(),
                                )
                            })?;
                        let denominator = caps
                            .name("denominator")
                            .and_then(|m| m.as_str().parse::<u64>().ok())
                            .ok_or_else(|| {
                                DataExtractionErr::MissingFileData(
                                    "scale factor denominator".to_string(),
                                    self.unextracted.path.clone(),
                                )
                            })?;

                        // 後のA/D値->加速度計算負荷軽減のために，先にスケールファクタの除算を計算
                        Ok(calculate_scale_factor(numerator, denominator))
                    } else {
                        Err(DataExtractionErr::PatternNotMatched(
                            "scale factor".to_string(),
                            self.unextracted.path.clone(),
                        ))?
                    }
                } else {
                    Err(DataExtractionErr::MissingFileData(
                        "scale factor".to_string(),
                        self.unextracted.path.clone(),
                    ))?
                }
            }
            Some(TextOrBinary::Binary(_bin_data)) => unimplemented!(), // MEMO: K-net, Kik-net バイナリ形式には未対応(対応予定)
            None => unreachable!(), // データの存在は担保されている
        }
    }
}

// A/D値とスケールファクタとの積を計算し，加速度値を求める
/// K-net, Kik-net, ASCII type only.
fn _to_acceleration_using_scale_factor(scale_factor: f64, ad_values: Vec<f64>) -> Vec<f64> {
    ad_values.iter().map(|acc| acc * scale_factor).collect()
}

// 計算量削減のため，先にscale factorの分数を計算する
fn calculate_scale_factor(numerator: u64, denominator: u64) -> f64 {
    numerator as f64 / denominator as f64
}
