use clap::{Parser, Subcommand, CommandFactory};
use std::process;

// Declaración de los módulos locales
mod models;
mod storage;
mod logic;

use models::Tarea;

// Define la estructura de los argumentos CLI (usando clap)
#[derive(Parser, Debug)]
#[command(author, version, about = "Aplicación To-Do CLI en Rust")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

// Define los subcomandos (add, list, edit, etc.)
#[derive(Subcommand, Debug)]
enum Commands {
    /// Añade una nueva tarea.
    Add {
        #[arg(short, long, help = "Nombre (título) de la tarea.")]
        name: String,
        #[arg(short, long, help = "Descripción detallada (opcional).")]
        desc: Option<String>,
    },
    /// Lista las tareas (opcionalmente filtra por estado).
    List {
        #[arg(short, long, help = "Filtra por estado: pendiente, en proceso, finalizada.")]
        state: Option<String>,
    },
    /// Edita el nombre y/o descripción de una tarea.
    Edit {
        #[arg(short, long, help = "ID de la tarea a editar.")]
        id: u32,
        #[arg(short = 'n', long, help = "Nuevo nombre (título) de la tarea.")]
        name: Option<String>,
        #[arg(short = 'd', long, help = "Nueva descripción de la tarea (usar '' para borrar).")]
        desc: Option<String>,
    },
    /// Cambia el estado de una tarea.
    Status {
        #[arg(short, long, help = "ID de la tarea a actualizar.")]
        id: u32,
        #[arg(short, long, help = "Nuevo estado: pendiente, en proceso, finalizada.")]
        state: String,
    },
    /// Elimina una tarea por ID.
    Delete {
        #[arg(short, long, help = "ID de la tarea a eliminar.")]
        id: u32,
    },
}

// Punto de entrada principal
fn main() {
    let args = Args::parse();

    // 1. Cargar tareas desde el disco
    let mut tareas = match storage::load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error al cargar tareas: {}", e);
            process::exit(1);
        }
    };

    let mut tareas_actualizadas: Option<Vec<Tarea>> = None;
    let mut message: Option<String> = None;

    // 2. Ejecutar el comando proporcionado por el usuario
    if let Some(command) = args.command {
        // Procesa el subcomando específico
        match command {
            Commands::Add { name, desc } => {
                let id = logic::add_task(&mut tareas, name, desc);
                tareas_actualizadas = Some(tareas.clone());
                message = Some(format!("Tarea añadida exitosamente (id: {})", id));
            }
            Commands::List { state } => {
                let state_filter = state.map(|s| s.to_lowercase());
                logic::list_tasks(&tareas, state_filter.as_deref());
            }
            Commands::Edit { id, name, desc } => {
                match logic::edit_task(&mut tareas, id, name, desc) {
                    Ok(true) => {
                        tareas_actualizadas = Some(tareas.clone());
                        message = Some(format!("Tarea editada exitosamente (id: {})", id));
                    }
                    Ok(false) => {
                        println!("No se especificó ningún campo para editar (use --name o --desc).");
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
            Commands::Status { id, state } => {
                match logic::change_status(&mut tareas, id, &state.to_lowercase()) {
                    Ok(_) => {
                        tareas_actualizadas = Some(tareas.clone());
                        message = Some(format!("Estado de la tarea actualizado exitosamente (id: {}) (estado: {})", id, state.to_lowercase()));
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
            Commands::Delete { id } => {
                match logic::delete_task(&mut tareas, id) {
                    Ok(_) => {
                        tareas_actualizadas = Some(tareas.clone());
                        message = Some(format!("Tarea eliminada exitosamente (id: {})", id));
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
        }
    } else {
        // Si no se da ningún comando, imprime la ayuda
        Args::command().print_help().unwrap();
        println!("\nEjemplos de uso:");
        println!("  ./trabajo-rust add --name \"Comprar ingredientes\" --desc \"Leche, huevos y pan\"");
        println!("  ./trabajo-rust list");
        println!("  ./trabajo-rust list --state pendiente");
        println!("  ./trabajo-rust status --id 1 --state finalizada");
    }

    // 3. Guardar cambios si se modificó la lista de tareas
    if let Some(updated_tasks) = tareas_actualizadas {
        if let Err(e) = storage::save_tasks(&updated_tasks) {
            eprintln!("Error al guardar tareas: {}", e);
            process::exit(1);
        }
        println!("{}", message.unwrap_or_default());
    }
}