use std::{collections::HashSet, fmt::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    error::{AnalysisConfigErr, AppError, ConfigValidationErr},
    model::ir::ProcessableFile,
};

const MULTIPLE_AXIS_TYPE: [&FromType; 2] = [&FromType::JpNiedKnet, &FromType::TkAfadAsc];

/// File format before conversion.  
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FromType {
    JpNiedKnet,
    UsScsnV2,
    NzGeonetV1a,
    NzGeonetV2a,
    TwPalertSac,
    TkAfadAsc,
}

impl FromType {
    fn to_snake_case(&self) -> &str {
        match self {
            FromType::JpNiedKnet => "jp_nied_knet",
            FromType::UsScsnV2 => "us_scsn_v2",
            FromType::NzGeonetV1a => "nz_geonet_v1a",
            FromType::NzGeonetV2a => "nz_geonet_v2a",
            FromType::TwPalertSac => "tw_palert_sac",
            FromType::TkAfadAsc => "tk_afad_asc",
        }
    }
}

/// File format after conversion.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToType {
    JpJmaCsv,
    JpStera3dTxt,
}

/// File format before conversion.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccAxis {
    Ns,
    Ew,
    Ud,
}

impl AccAxis {
    fn as_str(&self) -> &str {
        match self {
            AccAxis::Ns => "ns",
            AccAxis::Ew => "ew",
            AccAxis::Ud => "ud",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum NameFormat {
    /// ## **Example: 20240101-161018-ISK005-knet.csv.**
    /// - yyyymmdd:  Date and time of observation start date and time.
    /// - hhmmss: Hour, minute, second of the observation start date and time.
    /// - sn: Observation station name(ISK005, WVAS, etc...).
    /// - n: Institution name(knet, geonet, etc...).
    YyyymmddHhmmssSnN,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub global: GlobalConfig,
    pub conversion: Vec<ConversionConfig>,
}

impl Config {
    pub fn validate(&self) -> Result<(), Vec<AppError>> {
        let mut errors: Vec<AppError> = Vec::new();
        let mut all_names: Vec<String> = Vec::new();

        for conv_config in &self.conversion {
            let _ = conv_config.validate().map_err(|e| {
                errors.extend(e.into_iter().map(AppError::from));
            });
            all_names.push(conv_config.name.to_string());
        }

        let _ = self.validate_duplicate_name(all_names).map_err(|e| {
            errors.push(e.into());
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_duplicate_name(&self, all_names: Vec<String>) -> Result<(), AnalysisConfigErr> {
        let mut duplicate_name_set = HashSet::new();

        for name in all_names {
            if !duplicate_name_set.insert(name) {
                return Err(ConfigValidationErr::DuplicateNames(hashset_to_string(
                    &duplicate_name_set,
                ))
                .into());
            }
        }

        Ok(())
    }
}

// MEMO: 列挙型はtomlによってバリデーションが行われるため、この構造体でバリデーション実装は行いません。
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobalConfig {
    pub name_format: NameFormat,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ConversionConfig {
    pub name: String,
    pub from: FromType,
    pub to: ToType,
    pub group: Vec<GroupConfig>,
}

impl ConversionConfig {
    pub fn validate(&self) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        for (g_index, group_config) in self.group.iter().enumerate() {
            let id: usize = g_index + 1;
            let acceptable_exts: &[&str] = Self::assign_ext_based_on_from(&self.from);
            let _ = group_config
                .validate(&self.from, acceptable_exts, &self.name, id)
                .map_err(|e| {
                    errors.extend(e.into_iter());
                });
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    // tomlファイル構造体から処理構造体への変換
    pub fn iter_processable_files(&self) -> Vec<ProcessableFile> {
        let mut result = Vec::new();

        for (g_idx, group) in self.group.iter().enumerate() {
            for (f_idx, file) in group.files.iter().enumerate() {
                result.push(ProcessableFile {
                    conv_name: self.name.clone(),
                    from: self.from.clone(),
                    to: self.to.clone(),
                    group_index: g_idx,
                    file_index: f_idx,
                    acc_axis: file.acc_axis.clone(),
                    path: file.path.clone(),
                    data: file.data.clone(),
                });
            }
        }

        result
    }

    // 加速度の方向成分が別々のファイルで指定されているタイプのファイル
    fn assign_ext_based_on_from(from: &FromType) -> &[&str] {
        match from {
            FromType::JpNiedKnet => &["ns", "ew", "ud"],
            FromType::UsScsnV2 => &["v2"],
            FromType::NzGeonetV1a => &["v1a"],
            FromType::NzGeonetV2a => &["v2a"],
            FromType::TwPalertSac => &["sac"],
            FromType::TkAfadAsc => &["asc"],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GroupConfig {
    pub files: Vec<FileConfig>,
}

impl GroupConfig {
    pub fn validate(
        &self,
        from: &FromType,
        acceptable_exts: &[&str],
        name: &str,
        id: usize,
    ) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        for file in &self.files {
            let _ = file.validate(acceptable_exts).map_err(|e| {
                errors.extend(e.into_iter());
            });
        }

        self.validate_file_by_acc_axis(from, name, id)?;

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_file_by_acc_axis(
        &self,
        from: &FromType,
        name: &str,
        id: usize,
    ) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        // 各成分が別のファイルで管理されている形式の場合はNS,EW,UDの3つが必要
        if MULTIPLE_AXIS_TYPE.contains(&from) {
            let mut required_axis = vec!["ns", "ew", "ud"];
            for file in &self.files {
                // acc_axisが存在するか
                if let Some(acc_axis) = &file.acc_axis {
                    // 一致する要素が存在するか
                    if let Some(pos) = required_axis.iter().position(|&x| x == acc_axis.as_str()) {
                        required_axis.remove(pos);
                    } else {
                        errors.push(
                            ConfigValidationErr::DuplicateAccAxis(
                                from.to_snake_case().to_string(),
                                name.to_string(),
                                id,
                            )
                            .into(),
                        );
                    }
                } else {
                    errors.push(ConfigValidationErr::RequiredAccAxis(name.to_string(), id).into());
                }
            }
        // 全ての成分が単一ファイル内で管理されている形式
        } else {
            for file in &self.files {
                if !&file.acc_axis.is_none() {
                    errors.push(
                        ConfigValidationErr::MismatchedAccAxis(
                            from.to_snake_case().to_string(),
                            name.to_string(),
                            id,
                        )
                        .into(),
                    );
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FileConfig {
    pub path: PathBuf,
    pub acc_axis: Option<AccAxis>,
    #[serde(skip)]
    pub data: Option<TextOrBinary>, // 処理で必要なファイル内容を保持
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TextOrBinary {
    Text(Vec<String>),
    Binary(Vec<u8>),
}

impl FileConfig {
    pub fn validate(&self, acceptable_exts: &[&str]) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        let _ = self
            .validate_extension_for_acceptable_exts(acceptable_exts)
            .map_err(|e| {
                errors.push(e);
            });

        let _ = self.validate_path().map_err(|e| {
            errors.push(e);
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_path(&self) -> Result<(), AnalysisConfigErr> {
        if !self.path.exists() {
            return Err(ConfigValidationErr::PathDoesNotExist(self.path.to_path_buf()).into());
        } else if !self.path.is_file() {
            return Err(ConfigValidationErr::PathIsNotFile(self.path.to_path_buf()).into());
        }

        Ok(())
    }

    fn validate_extension_for_acceptable_exts(
        &self,
        acceptable_exts: &[&str],
    ) -> Result<(), AnalysisConfigErr> {
        if let Some(extension) = self
            .path
            .extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
        {
            if !acceptable_exts.contains(&extension.as_str()) {
                return Err(ConfigValidationErr::InvalidExtension(
                    extension,
                    acceptable_exts.join(", "),
                )
                .into());
            }
        } else {
            return Err(ConfigValidationErr::NoExtension(self.path.to_path_buf()).into());
        }

        Ok(())
    }
}

fn hashset_to_string(set: &HashSet<String>) -> String {
    let mut result = String::new();
    for item in set {
        writeln!(&mut result, "{}", item).unwrap();
    }
    result
}
