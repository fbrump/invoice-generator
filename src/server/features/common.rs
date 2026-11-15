use uuid::Uuid;

use crate::server::{
    features::{generate_csv::generate::GenerateCsv, generate_excel::generate::GenerateXls},
    models::transaction::Transaction,
    serializes::{ReportDataResponse, ReportTypeEnum},
};

#[derive(Debug, Clone)]
pub struct GenerateReportFactory {}

pub trait ReportFactory {
    fn get_generator(
        report_type: ReportTypeEnum,
        transactions: Vec<Transaction>,
    ) -> Result<ReportDataResponse, &'static str>;
}

impl ReportFactory for GenerateReportFactory {
    fn get_generator(
        report_type: ReportTypeEnum,
        transactions: Vec<Transaction>,
    ) -> Result<ReportDataResponse, &'static str> {
        match report_type {
            ReportTypeEnum::CSV => Ok(GenerateCsv::new().create(transactions).unwrap()),
            ReportTypeEnum::XLS => Ok(GenerateXls::new().create(transactions).unwrap()),
            _ => Err("There is no handled format"),
        }
    }
}

pub trait CreateReport {
    fn create(&self, items: Vec<Transaction>) -> Result<ReportDataResponse, &'static str>;
}

#[derive(Debug, Clone)]
pub struct GenerateReport {
    file_prefix: String,
    file_extension: String,
}

impl GenerateReport {
    pub fn new(file_prefix: String, file_extension: String) -> Self {
        GenerateReport {
            file_prefix: file_prefix,
            file_extension: file_extension,
        }
    }

    pub fn generate_file_name(&self, report_id: Option<Uuid>) -> String {
        match report_id {
            Some(id) => format!("{}-{id}.{}", self.file_prefix, self.file_extension),
            None => {
                format!(
                    "{}-{}.{}",
                    self.file_prefix,
                    Uuid::new_v4(),
                    self.file_extension
                )
            }
        }
    }
}
