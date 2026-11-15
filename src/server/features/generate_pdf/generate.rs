use uuid::Uuid;

use crate::server::{
    features::common::{CreateReport, GenerateReport},
    models::transaction::Transaction,
    serializes::{ReportDataResponse, ReportTypeEnum},
};

pub struct GeneratePdf {
    pub generate_report: GenerateReport,
}

impl GeneratePdf {
    pub fn new() -> Self {
        GeneratePdf {
            generate_report: GenerateReport::new(String::from("transactions"), String::from("pdf")),
        }
    }
}

impl CreateReport for GeneratePdf {
    fn create(&self, items: Vec<Transaction>) -> Result<ReportDataResponse, &'static str> {
        let report_id = Uuid::new_v4();
        let file_name = self.generate_report.generate_file_name(Some(report_id));

        let full_path = format!("{}{file_name}", Self::BASE_PATH.to_string());
        // TO-DO: Implement here the pdf stuff

        Ok(ReportDataResponse::new(
            ReportTypeEnum::PDF,
            report_id,
            file_name,
            full_path,
        ))
    }
}
