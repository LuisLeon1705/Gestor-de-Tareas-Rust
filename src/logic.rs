use chrono::Local;

// Importamos lo necesario de otros módulos declarados en main.rs
use crate::models; // <--- Importamos el módulo completo
use crate::models::Tarea;

// --- Utilidad ---
fn get_new_id(tareas: &[Tarea]) -> u32 {
    tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

// --- Funciones CRUD ---

pub fn add_task(tareas: &mut Vec<Tarea>, descripcion: String) -> u32 {
    let id = get_new_id(tareas);
    let nueva_tarea = Tarea {
        id,
        descripcion,
        fecha_creacion: Local::now(),
        estado: models::PENDIENTE.to_string(), // <--- Uso explícito del módulo
        fecha_finalizacion: None,
    };
    tareas.push(nueva_tarea);
    id
}

pub fn edit_task(tareas: &mut [Tarea], id: u32, nueva_descripcion: String) -> Result<(), String> {
    for t in tareas.iter_mut() {
        if t.id == id {
            t.descripcion = nueva_descripcion;
            return Ok(());
        }
    }
    Err(format!("Error: no se encontró tarea con ID {}", id))
}

pub fn change_status(tareas: &mut [Tarea], id: u32, nuevo_estado: &str) -> Result<(), String> {
    // CORRECCIÓN E0408: Uso de la ruta completa de las constantes para el match
    let estado_valido = match nuevo_estado {
        models::PENDIENTE | models::EN_PROCESO | models::FINALIZADA => true,
        _ => false,
    };
    if !estado_valido {
        return Err(format!("Error: el estado '{}' no es válido. Válidos: {}, {}, {}", nuevo_estado, models::PENDIENTE, models::EN_PROCESO, models::FINALIZADA));
    }

    for t in tareas.iter_mut() {
        if t.id == id {
            t.estado = nuevo_estado.to_string();

            if nuevo_estado == models::FINALIZADA {
                t.fecha_finalizacion = Some(Local::now());
            } else {
                t.fecha_finalizacion = None;
            }
            return Ok(());
        }
    }
    Err(format!("Error: no se encontró tarea con ID {} para cambiar estado", id))
}

pub fn delete_task(tareas: &mut Vec<Tarea>, id: u32) -> Result<(), String> {
    let initial_len = tareas.len();
    tareas.retain(|t| t.id != id); // Elimina tareas cuyo ID coincide
    
    if tareas.len() < initial_len {
        Ok(())
    } else {
        Err(format!("Error: no se encontró tarea con ID {} para eliminar", id))
    }
}

pub fn list_tasks(tareas: &[Tarea]) {
    if tareas.is_empty() {
        println!("No se han registrado tareas aún");
        return;
    }

    println!("\n--- Lista de Tareas To-Do ---");
    println!("[ID] DESCRIPCION (ESTADO)");
    for t in tareas {
        let mut status = format!("({})", t.estado);
        if t.estado == models::FINALIZADA {
            if let Some(fecha) = t.fecha_finalizacion {
                status = format!("(FINALIZADA en {})", fecha.format("%d %b %H:%M"));
            }
        }

        // Códigos ANSI para color: \x1b[32m (verde) y \x1b[0m (reset)
        let color = if t.estado == models::FINALIZADA { "\x1b[32m" } else { "\x1b[0m" };
        let reset = "\x1b[0m";

        println!("[{}] {} {}{}{}", t.id, t.descripcion, color, status, reset);
    }
    println!("---------------------------");
}
