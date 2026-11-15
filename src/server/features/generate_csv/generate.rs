use csv::Writer;
use uuid::Uuid;

use crate::server::{
    features::common::{CreateReport, GenerateReport},
    models::transaction::Transaction,
    serializes::{ReportDataResponse, ReportTypeEnum},
};

pub struct GenerateCsv {
    pub generate_report: GenerateReport,
}

impl GenerateCsv {
    pub fn new() -> Self {
        GenerateCsv {
            generate_report: GenerateReport::new(String::from("transactions"), String::from("csv")),
        }
    }
}

impl CreateReport for GenerateCsv {
    fn create(&self, items: Vec<Transaction>) -> Result<ReportDataResponse, &'static str> {
        let report_id = Uuid::new_v4();
        let file_name = self.generate_report.generate_file_name(Some(report_id));

        let full_path = format!("{}{file_name}", "/tmp/".to_string());
        let csv_writer = Writer::from_path(&full_path);

        match csv_writer {
            Ok(mut file) => {
                // header
                file.write_record(&["id", "merchant", "value"]).unwrap();
                // rows
                for item in items {
                    file.write_record(&[
                        item.id.to_string(),
                        item.merchant_name,
                        item.value.to_string(),
                    ])
                    .unwrap();
                }
                let _ = file.flush();

                Ok(ReportDataResponse::new(
                    ReportTypeEnum::CSV,
                    report_id,
                    file_name,
                    full_path,
                ))
            }
            Err(_) => Err("There is a problem on generate report"),
        }
    }
}
