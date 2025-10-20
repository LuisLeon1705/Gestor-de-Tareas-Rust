use chrono::Local;

use crate::models;
use crate::models::Tarea;

// Calcula el siguiente ID disponible (basado en el ID máximo actual)
fn get_new_id(tareas: &[Tarea]) -> u32 {
    tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

// Crea y añade una nueva tarea al vector
pub fn add_task(tareas: &mut Vec<Tarea>, nombre: String, descripcion: Option<String>) -> u32 {
    let id = get_new_id(tareas);
    let nueva_tarea = Tarea {
        id,
        nombre,
        descripcion,
        fecha_creacion: Local::now(),
        estado: models::PENDIENTE.to_string(),
        fecha_finalizacion: None,
    };
    tareas.push(nueva_tarea);
    id
}

// Busca una tarea por ID y actualiza su nombre y/o descripción
pub fn edit_task(tareas: &mut [Tarea], id: u32, nuevo_nombre: Option<String>, nueva_descripcion: Option<String>) -> Result<bool, String> {
    if nuevo_nombre.is_none() && nueva_descripcion.is_none() {
        return Ok(false); 
    }

    for t in tareas.iter_mut() {
        if t.id == id {
            let mut changed = false;
            
            if let Some(name) = nuevo_nombre {
                t.nombre = name;
                changed = true;
            }
            
            if let Some(desc) = nueva_descripcion {
                t.descripcion = if desc.is_empty() { Some(desc) } else { None };
                changed = true;
            }

            return Ok(changed);
        }
    }
    Err(format!("Error: no se encontró tarea con ID {}", id))
}

// Cambia el estado de una tarea (pendiente, en proceso, finalizada)
pub fn change_status(tareas: &mut [Tarea], id: u32, nuevo_estado: &str) -> Result<(), String> {
    let estado_valido = match nuevo_estado {
        models::PENDIENTE | models::EN_PROCESO | models::FINALIZADA => true,
        _ => false,
    };
    if !estado_valido {
        return Err(format!("Error: el estado '{}' no es válido. Válidos: {}, {}, {}", nuevo_estado, models::PENDIENTE, models::EN_PROCESO, models::FINALIZADA));
    }

    for t in tareas.iter_mut() {
        if t.id == id {
            
            // Valida que la tarea no esté ya finalizada
            if t.estado == models::FINALIZADA {
                return Err(format!("Error: la tarea {} ya está finalizada. No se puede cambiar su estado.", id));
            }

            t.estado = nuevo_estado.to_string();

            // Asigna fecha de finalización si el estado es 'finalizada'
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

// Elimina una tarea del vector usando su ID
pub fn delete_task(tareas: &mut Vec<Tarea>, id: u32) -> Result<(), String> {
    let initial_len = tareas.len();
    tareas.retain(|t| t.id != id); 
    
    if tareas.len() < initial_len {
        Ok(())
    } else {
        Err(format!("Error: no se encontró tarea con ID {} para eliminar", id))
    }
}

// Imprime las tareas en la consola, aplicando un filtro de estado opcional
pub fn list_tasks(tareas: &[Tarea], state_filter: Option<&str>) {
    
    // Valida el filtro de estado, si se proporcionó
    if let Some(state) = state_filter {
        let estado_valido = match state {
            models::PENDIENTE | models::EN_PROCESO | models::FINALIZADA => true,
            _ => false,
        };
        if !estado_valido {
            println!("Error: el estado de filtro '{}' no es válido. Válidos: {}, {}, {}", state, models::PENDIENTE, models::EN_PROCESO, models::FINALIZADA);
            return;
        }
    }

    // Filtra las tareas si se proveyó un 'state_filter'
    let tareas_a_mostrar: Vec<&Tarea> = tareas.iter()
        .filter(|t| {
            match state_filter {
                Some(s) => t.estado == s,
                None => true,
            }
        })
        .collect();

    // Comprueba si la lista filtrada está vacía
    if tareas_a_mostrar.is_empty() {
        if let Some(s) = state_filter {
            println!("No se han registrado tareas con el estado: '{}'", s);
        } else {
            println!("No se han registrado tareas aún");
        }
        return;
    }

    if let Some(s) = state_filter {
        println!("\n--- Lista de Tareas (Filtradas por: {}) ---", s);
    } else {
        println!("\n--- Lista de Tareas To-Do ---");
    }

    // Itera e imprime los detalles de cada tarea filtrada
    for t in tareas_a_mostrar {
        let color = if t.estado == models::FINALIZADA { "\x1b[32m" } else { "\x1b[0m" };
        let reset = "\x1b[0m";

        println!("{}{}", color, "---------------------------");
        
        println!("[ID: {}] {} (Estado: {})", t.id, t.nombre, t.estado);

        let fecha_creacion_str = t.fecha_creacion.format("%Y-%m-%d %H:%M");
        let mut fecha_fin_str = "N/A".to_string();
        if let Some(fecha) = t.fecha_finalizacion {
            fecha_fin_str = fecha.format("%Y-%m-%d %H:%M").to_string();
        }
        println!("    Creada: {} | Finalizada: {}", fecha_creacion_str, fecha_fin_str);

        if let Some(desc) = &t.descripcion {
            if !desc.is_empty() {
                println!("    Desc: {}", desc);
            }
        }
        
        println!("{}", reset);
    }
    println!("---------------------------");
}