use std::fs;
use std::path::Path;
use serde_json;

use crate::models::{Tarea, ARCHIVO_DATOS};

// Carga la lista de tareas desde el archivo JSON
pub fn load_tasks() -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    // Si el archivo no existe o está vacío, devuelve un vector nuevo
    if !Path::new(ARCHIVO_DATOS).exists() || fs::metadata(ARCHIVO_DATOS)?.len() == 0 {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(ARCHIVO_DATOS)?;
    let tareas: Vec<Tarea> = serde_json::from_str(&data)?;
    Ok(tareas)
}

// Guarda la lista actual de tareas en el archivo JSON
pub fn save_tasks(tasks: &[Tarea]) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(tasks)?;
    fs::write(ARCHIVO_DATOS, json_data)?;
    Ok(())
}