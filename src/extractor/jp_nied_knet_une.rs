use crate::model::ir::ProcessableFile;

use super::extractor::Extractor;

pub struct JpNiedKnetUneExtractor {
    pub unextracted: ProcessableFile,
}

impl Extractor for JpNiedKnetUneExtractor {
    fn extract(&self) -> Result<crate::model::ir::SeismicIr, crate::error::AppError> {
        todo!()
    }

    fn extract_latitude(&self) -> Result<f64, crate::error::ProcessErr> {
        todo!()
    }

    fn extract_longitude(&self) -> Result<f64, crate::error::ProcessErr> {
        todo!()
    }

    fn extract_unit_type(&self) -> Result<String, crate::error::ProcessErr> {
        todo!()
    }

    fn extract_acc_values(
        &self,
    ) -> Result<super::extractor::Acceleration, crate::error::ProcessErr> {
        todo!()
    }

    fn extract_initial_time(&self) -> Result<String, crate::error::ProcessErr> {
        todo!()
    }
}

impl JpNiedKnetUneExtractor {
    pub fn new(unextracted: ProcessableFile) -> Self {
        Self { unextracted }
    }
}
