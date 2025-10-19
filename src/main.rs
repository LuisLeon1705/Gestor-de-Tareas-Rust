use clap::{Parser, Subcommand, CommandFactory}; // <--- CORRECCIÓN: Se añade CommandFactory
use std::process;

// Declaración de los módulos (los hace visibles)
mod models;
mod storage;
mod logic;

// Importamos solo lo necesario para el flujo principal
use models::Tarea;

// --- ARGUMENT PARSER (CLI Layer) ---

#[derive(Parser, Debug)]
#[command(author, version, about = "Aplicación To-Do CLI en Rust")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Añade una nueva tarea.
    Add {
        #[arg(short, long, help = "Descripción de la tarea.")]
        desc: String,
    },
    /// Lista todas las tareas.
    List,
    /// Edita la descripción de una tarea.
    Edit {
        #[arg(short, long, help = "ID de la tarea a editar.")]
        id: u32,
        #[arg(short, long, help = "Nueva descripción de la tarea.")]
        desc: String,
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

fn main() {
    let args = Args::parse();

    // 1. Cargar tareas (usa el módulo storage)
    let mut tareas = match storage::load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error al cargar tareas: {}", e);
            process::exit(1);
        }
    };

    let mut tareas_actualizadas: Option<Vec<Tarea>> = None;
    let mut message: Option<String> = None;

    // 2. Ejecutar comando (usa el módulo logic)
    if let Some(command) = args.command {
        match command {
            Commands::Add { desc } => {
                let id = logic::add_task(&mut tareas, desc);
                tareas_actualizadas = Some(tareas.clone());
                message = Some(format!("Tarea añadida exitosamente (id: {})", id));
            }
            Commands::List => {
                logic::list_tasks(&tareas);
                return;
            }
            Commands::Edit { id, desc } => {
                match logic::edit_task(&mut tareas, id, desc) {
                    Ok(_) => {
                        tareas_actualizadas = Some(tareas.clone());
                        message = Some(format!("Tarea editada exitosamente (id: {})", id));
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
            Commands::Status { id, state } => {
                // Se pasa a minúsculas antes de la validación
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
        // Imprime la ayuda si no hay comando
        Args::command().print_help().unwrap();
        println!("\nEjemplos de uso:");
        println!("  ./trabajo-rust add --desc \"Comprar ingredientes\"");
        println!("  ./trabajo-rust status --id 1 --state finalizada");
        println!("  ./trabajo-rust list");
        return;
    }

    // 3. Guardar las tareas si hubo cambios (usa el módulo storage)
    if let Some(updated_tasks) = tareas_actualizadas {
        if let Err(e) = storage::save_tasks(&updated_tasks) {
            eprintln!("Error al guardar tareas: {}", e);
            process::exit(1);
        }
        println!("{}", message.unwrap_or_default());
    }
}
