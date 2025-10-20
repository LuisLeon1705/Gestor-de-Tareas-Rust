use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

// Definición de estados y nombre de archivo
pub const PENDIENTE: &str = "pendiente";
pub const EN_PROCESO: &str = "en proceso";
pub const FINALIZADA: &str = "finalizada";
pub const ARCHIVO_DATOS: &str = "tareas.json";

// Representa la estructura de datos de una Tarea
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tarea {
    pub id: u32,
    pub nombre: String,
    #[serde(default)]
    #[serde(rename = "descripcion_detallada")]
    pub descripcion: Option<String>,
    pub fecha_creacion: DateTime<Local>,
    pub estado: String,
    pub fecha_finalizacion: Option<DateTime<Local>>,
}