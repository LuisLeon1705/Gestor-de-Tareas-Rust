use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

// Constantes
pub const PENDIENTE: &str = "pendiente";
pub const EN_PROCESO: &str = "en proceso";
pub const FINALIZADA: &str = "finalizada";
pub const ARCHIVO_DATOS: &str = "tareas.json";

// Estructura Tarea
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tarea {
    pub id: u32,
    pub descripcion: String,
    pub fecha_creacion: DateTime<Local>,
    pub estado: String,
    pub fecha_finalizacion: Option<DateTime<Local>>,
}