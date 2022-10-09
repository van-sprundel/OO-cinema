pub trait ExportBehavior {
    fn export(&self);
}

pub struct ExportToJSON;

impl ExportBehavior for ExportToJSON {
    fn export(&self) {}
}

pub struct ExportToPDF;

impl ExportBehavior for ExportToPDF {
    fn export(&self) {}
}
