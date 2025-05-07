use std::{
    cell::RefCell,
    path::{Path, PathBuf},
};

use log::debug;

use crate::{
    analysis_config::TextOrBinary,
    error::{AppError, DataExtractionErr, ProcessErr},
    extractor::extractor::mock_seismic_ir_data,
    model::ir::{ProcessableFile, SeismicIr},
    util::{to_fixed_chunks, to_little_endian},
};

use super::extractor::{Acceleration, Extractor};

pub struct TwPalertSacExtractor {
    pub unextracted: RefCell<ProcessableFile>,
}

impl Extractor for TwPalertSacExtractor {
    fn extract(&self) -> Result<SeismicIr, Vec<AppError>> {
        self.swap_little_endian()
            .map_err(|e| vec![AppError::from(e)])?;

        println!("{:?}", self.extract_latitude());
        println!("{:?}", self.extract_longitude());
        println!("{:?}", self.extract_unit_type());

        // Match文で、Toごとに抽出を切り替える
        // 加速度以外のデータが方向成分で同一であることを確認
        Ok(mock_seismic_ir_data()) // 処理継続のためのモックデータ
    }

    fn extract_latitude(&self) -> Result<f64, ProcessErr> {
        let result = self.with_each_binary(|bin, path| {
            let chunks = to_fixed_chunks::<4>(bin.to_vec());
            let lat_bytes = chunks.get(31).ok_or_else(|| {
                DataExtractionErr::FailedExtraction("latitude".into(), path.to_path_buf())
            })?;
            let lat = f32::from_le_bytes(*lat_bytes) as f64;
            Ok(lat)
        });

        match result {
            Ok(lat) => Ok(lat),
            Err(e) => Err(e),
        }
    }

    fn extract_longitude(&self) -> Result<f64, ProcessErr> {
        let result = self.with_each_binary(|bin, path| {
            let chunks = to_fixed_chunks::<4>(bin.to_vec());
            let lon_bytes = chunks.get(32).ok_or_else(|| {
                DataExtractionErr::FailedExtraction("longitude".into(), path.to_path_buf())
            })?;
            let lon = f32::from_le_bytes(*lon_bytes) as f64;
            Ok(lon)
        });

        match result {
            Ok(lon) => Ok(lon),
            Err(e) => Err(e),
        }
    }

    // https://www.seis.nagoya-u.ac.jp/~maeda/ymaeda_opentools_doc/include/sac/grobal.h/index.html をもとにマッチ
    fn extract_unit_type(&self) -> Result<String, ProcessErr> {
        let result = self.with_each_binary(|bin, path| {
            let chunks = to_fixed_chunks::<4>(bin.to_vec());
            let unit_bytes = chunks.get(86).ok_or_else(|| {
                DataExtractionErr::FailedExtraction("unit type".into(), path.to_path_buf())
            })?;

            let idep_val = i32::from_le_bytes(*unit_bytes);
            // eprintln!("idep_val: {:?}", idep_val);

            let unit_type = match idep_val {
                -12345 => "undefined", // 未定義値
                5 => "unknown",        // sacenum_IUNKN 複数データ？
                6 => "disp(nm)",       // sacenum_IDISP 変位 nm.
                7 => "Vel(nm/sec)",    // sacenum_IVEL 速度 nm/sec.
                8 => "nm/sec/sec",     // sacenum_IACC 加速度 nm/sec/sec.
                50 => "volts",         // sacenum_IVOLTS 速度 volts.
                _ => "notfound",       // エラー
            };

            Ok(unit_type.to_string())
        });

        match result {
            Ok(unit) => Ok(unit),
            Err(e) => Err(e),
        }
    }

    fn extract_acc_values(&self) -> Result<Acceleration, ProcessErr> {
        todo!()
    }

    fn extract_initial_time(&self) -> Result<String, ProcessErr> {
        todo!()
    }
}

impl TwPalertSacExtractor {
    pub fn new(unextracted: ProcessableFile) -> Self {
        Self {
            unextracted: RefCell::new(unextracted),
        }
    }

    fn with_each_binary<T, F>(&self, mut f: F) -> Result<T, ProcessErr>
    where
        F: FnMut(&[u8], &Path) -> Result<T, ProcessErr>,
    {
        let unextracted = self.unextracted.borrow();

        if let Some(data) = &unextracted.data {
            match data {
                TextOrBinary::Binary(bin) => {
                    let val = f(bin, &unextracted.path)?;
                    Ok(val)
                }
                TextOrBinary::Text(_) => {
                    return Err(DataExtractionErr::FormatUnsupported(
                        "Text type of SAC".to_string(),
                    )
                    .into());
                }
            }
        } else {
            return Err(DataExtractionErr::MissingFileData(unextracted.path.clone()).into());
        }
    }

    fn swap_little_endian(&self) -> Result<(), ProcessErr> {
        let path = self.unextracted.borrow().path.clone(); // RefCellを介したborrow_mutの制約回避のため，可変参照を行う前にclone
        let mut unextracted = self.unextracted.borrow_mut(); // <--- ここから可変参照

        if let Some(data) = &mut unextracted.data {
            match data {
                TextOrBinary::Text(_) => todo!(), // MEMO: Text型のSACには現在未対応
                TextOrBinary::Binary(bin) => {
                    // SACでは，1word = 4byteだから1chunk = 4byteを指定
                    let bin_chunks = to_fixed_chunks::<4>(bin.to_vec());
                    match SacHeader::swap_endian(bin_chunks, path.as_path()) {
                        Ok(swaped_chunks) => {
                            let swapped_bytes: Vec<u8> = swaped_chunks
                                .iter()
                                .flat_map(|arr| arr.iter())
                                .copied()
                                .collect();

                            *data = TextOrBinary::Binary(swapped_bytes);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct SacHeader {
    pub delta: f32,  // 時間刻み
    pub b: f32,      // 独立変数の開始値
    pub e: f32,      // 独立変数の終了値
    pub depmin: f32, // 従属変数の最小値
    pub depmax: f32, // 従属変数の最大値
    pub nvhdr: i32,  // SAC version
}

impl SacHeader {
    // エンディアン判別ロジック: SACファイルの特徴に基づいて判別(Little_Endianをデフォルトとする)
    // 1. deltaの正常範囲(0.001~10.0)
    // 2. 独立変数の開始値と終了値(B,E)の大小関係
    // 3. 従属変数の最大最小(depmin/max)の大小関係
    // 4. NVHDRの正常値(6 or 7)
    // エンディアンを判別する関数
    pub fn detect_endian(bin_chunks: &Vec<[u8; 4]>) -> Result<Endian, ProcessErr> {
        let be_header = Self::parse_sac_header_be(bin_chunks);
        let le_header = Self::parse_sac_header_le(bin_chunks);

        let mut be_valid = 0;
        let mut le_valid = 0;

        // 1. deltaのチェック (0.001〜10の範囲内かつ正の実数)
        if be_header.delta > 0.001 && be_header.delta < 10.0 {
            be_valid += 1;
        }
        if le_header.delta > 0.001 && le_header.delta < 10.0 {
            le_valid += 1;
        }

        // 2. 独立変数の関係チェック
        if be_header.e > be_header.b {
            be_valid += 1;
        }
        if le_header.e > le_header.b {
            le_valid += 1;
        }

        // 3. 従属変数の関係チェック
        if be_header.depmax > be_header.depmin {
            be_valid += 1;
        }
        if le_header.depmax > le_header.depmin {
            le_valid += 1;
        }

        // nvhdrの正常値チェック
        if be_header.nvhdr == 6 || be_header.nvhdr == 7 {
            be_valid += 1;
        }
        if le_header.nvhdr == 6 || le_header.nvhdr == 7 {
            le_valid += 1;
        }

        debug!(
            "\nBig Endian Interpretation:\n \
             delta = {}\n \
             b = {}\n \
             e = {}\n \
             depmin = {}\n \
             depmax = {}\n \
             nvhdr = {}",
            be_header.delta,
            be_header.b,
            be_header.e,
            be_header.depmin,
            be_header.depmax,
            be_header.nvhdr
        );

        debug!(
            "\nLittle Endian Interpretation:\n \
             delta = {}\n \
             b = {}\n \
             e = {}\n \
             depmin = {}\n \
             depmax = {}\n \
             nvhdr = {}",
            le_header.delta,
            le_header.b,
            le_header.e,
            le_header.depmin,
            le_header.depmax,
            le_header.nvhdr
        );

        debug!(
            "\nDiscrimination Criterion Score - Big Endian: {}, Little Endian: {}",
            be_valid, le_valid
        );

        if be_valid > le_valid {
            Ok(Endian::Big)
        } else if be_valid < le_valid {
            Ok(Endian::Little)
        } else {
            Err(ProcessErr::Extraction(
                DataExtractionErr::EndianDetectionFailed(PathBuf::new()), // file_pathはここでは不要
            ))
        }
    }

    // エンディアンに基づいてスワップを行う関数
    pub fn process_endian(bin_chunks: Vec<[u8; 4]>, endian: Endian) -> Vec<[u8; 4]> {
        match endian {
            Endian::Big => to_little_endian(bin_chunks),
            Endian::Little => bin_chunks,
        }
    }

    pub fn swap_endian(
        bin_chunks: Vec<[u8; 4]>,
        file_path: &Path,
    ) -> Result<Vec<[u8; 4]>, ProcessErr> {
        match Self::detect_endian(&bin_chunks) {
            Ok(endian) => Ok(Self::process_endian(bin_chunks, endian)),
            Err(_) => Err(ProcessErr::Extraction(
                DataExtractionErr::EndianDetectionFailed(file_path.to_path_buf()), // エラーをここで新規に構築
            )),
        }
    }

    fn parse_sac_header_be(bytes: &Vec<[u8; 4]>) -> SacHeader {
        SacHeader {
            delta: f32::from_be_bytes(*bytes.get(0).unwrap()),
            b: f32::from_be_bytes(*bytes.get(5).unwrap()),
            e: f32::from_be_bytes(*bytes.get(6).unwrap()),
            depmin: f32::from_be_bytes(*bytes.get(1).unwrap()),
            depmax: f32::from_be_bytes(*bytes.get(2).unwrap()),
            nvhdr: i32::from_be_bytes(*bytes.get(76).unwrap()),
        }
    }

    fn parse_sac_header_le(bytes: &Vec<[u8; 4]>) -> SacHeader {
        SacHeader {
            delta: f32::from_le_bytes(*bytes.get(0).unwrap()),
            b: f32::from_le_bytes(*bytes.get(5).unwrap()),
            e: f32::from_le_bytes(*bytes.get(6).unwrap()),
            depmin: f32::from_le_bytes(*bytes.get(1).unwrap()),
            depmax: f32::from_le_bytes(*bytes.get(2).unwrap()),
            nvhdr: i32::from_le_bytes(*bytes.get(76).unwrap()),
        }
    }
}

pub enum Endian {
    Big,
    Little,
}
