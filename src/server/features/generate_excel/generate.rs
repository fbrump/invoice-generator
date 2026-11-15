use rust_xlsxwriter::workbook::Workbook;
use uuid::Uuid;

use crate::server::{
    features::common::{CreateReport, GenerateReport},
    models::transaction::Transaction,
    serializes::{ReportDataResponse, ReportTypeEnum},
};

pub struct GenerateXls {
    generate_report: GenerateReport,
}

impl GenerateXls {
    pub fn new() -> Self {
        GenerateXls {
            generate_report: GenerateReport::new(
                String::from("transactions"),
                String::from("xlsx"),
            ),
        }
    }
}

impl CreateReport for GenerateXls {
    fn create(&self, items: Vec<Transaction>) -> Result<ReportDataResponse, &'static str> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // header
        let header_row = 0;
        match worksheet.write(header_row, 0, "id") {
            Ok(_) => println!("Header ID wrote with success"),
            Err(_) => {
                todo!()
            }
        }
        match worksheet.write(header_row, 1, "merchant") {
            Ok(_) => println!("Header merchant wrote with success"),
            Err(_) => {
                todo!()
            }
        }
        match worksheet.write(header_row, 2, "value") {
            Ok(_) => println!("Header value wrote with success"),
            Err(_) => {
                todo!()
            }
        }

        // rows
        for (i, item) in items.iter().enumerate() {
            let row_index: u32 = (i + 1).try_into().unwrap();
            match worksheet.write(row_index, 0, item.id.to_string()) {
                Ok(_) => println!("Row ID wrote with success"),
                Err(_) => {
                    todo!()
                }
            }
            match worksheet.write(row_index, 1, &item.merchant_name) {
                Ok(_) => println!("Row merchant wrote with success"),
                Err(_) => {
                    todo!()
                }
            }
            match worksheet.write(row_index, 2, item.value.to_string()) {
                Ok(_) => println!("Row value wrote with success"),
                Err(_) => {
                    todo!()
                }
            }
        }

        // save
        let report_id = Uuid::new_v4();
        let file_name = self.generate_report.generate_file_name(Some(report_id));
        let full_path = format!("{}{file_name}", Self::BASE_PATH.to_string());

        match workbook.save(&full_path) {
            Ok(_) => Ok(ReportDataResponse::new(
                ReportTypeEnum::XLS,
                report_id,
                file_name,
                full_path,
            )),
            Err(_) => Err("There is a problem on generate report"),
        }
    }
}
