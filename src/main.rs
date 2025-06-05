mod graph;
mod ubication;
mod io_utils;
mod search;

use crate::graph::Grafo;
use crate::ubication::leer_pistas_dinamico;
use crate::io_utils::{leer_grafo_desde_archivo, escribir_ruta_en_archivo};
use crate::search::{dfs_buscar_tesoro, dijkstra_ruta_minima};
use std::io;

fn main() -> io::Result<()> {
    // 1) Leer grafo de "grafo.txt"
    let ruta_grafo = "grafo.txt";
    println!("Cargando grafo desde '{}'\n", ruta_grafo);
    let grafo: Grafo = leer_grafo_desde_archivo(ruta_grafo)?;
    println!("Grafo cargado con {} nodos.\n", grafo.nombres.len());

    // 2) Leer pistas dinámicas de "pistas.txt" y obtener (Ubicaciones, indice_tesoro)
    let ruta_pistas = "pistas.txt";
    println!("Cargando pistas dinámicas desde '{}'\n", ruta_pistas);
    let (mut ubicaciones, indice_tesoro) =
        leer_pistas_dinamico(&grafo.mapa_nombre_indice, ruta_pistas)?;
    println!(
        "Índice del tesoro (dinámico) = {} ('{}').\n",
        indice_tesoro, grafo.nombres[indice_tesoro]
    );

    // 3) Pedir al usuario el nodo de inicio
    println!("Nodos disponibles:");
    for (i, nombre) in grafo.nombres.iter().enumerate() {
        println!("{}: {}", i, nombre);
    }
    print!("\nIngrese índice de inicio: ");
    io::Write::flush(&mut io::stdout())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let indice_inicio: usize = input.trim().parse().expect("Índice inválido");
    println!(
        "El pirata inicia en '{}' (índice {}).\n",
        grafo.nombres[indice_inicio], indice_inicio
    );

    // Bucle principal del menú
    loop {
        println!("\n=== MENÚ ===");
        println!("1) Seguir pistas (DFS)");
        println!("2) Camino más corto (Dijkstra)");
        println!("0) Salir");
        print!("\nOpción: ");
        io::Write::flush(&mut io::stdout())?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        let opcion: u32 = input.trim().parse().unwrap_or(u32::MAX);

        match opcion {
            1 => {
                println!("\n--- Opción 1: DFS siguiendo pistas ---\n");
                // Resetear vector de visitados
                let mut visitado = vec![false; grafo.nombres.len()];
                let mut ruta_indices = Vec::new();

                let hallado = dfs_buscar_tesoro(
                    &grafo,
                    indice_inicio,
                    indice_tesoro,
                    &mut visitado,
                    &mut ruta_indices,
                );
                if hallado {
                    let ruta_nombres: Vec<String> = ruta_indices
                        .iter()
                        .map(|&i| grafo.nombres[i].clone())
                        .collect();
                    println!("¡Tesoro encontrado! Ruta (DFS): {:?}\n", ruta_nombres);
                    escribir_ruta_en_archivo(&ruta_nombres, "ruta_tesoro.txt")?;
                    println!("Ruta guardada en 'ruta_tesoro.txt'.\n");
                } else {
                    println!("No se encontró el tesoro siguiendo las pistas.\n");
                }
            }

            2 => {
                println!("\n--- Opción 2: Camino más corto con Dijkstra ---\n");
                let ruta_indices = dijkstra_ruta_minima(&grafo, indice_inicio, indice_tesoro);
                if ruta_indices.is_empty() {
                    println!("No se encontró ruta con Dijkstra.\n");
                } else {
                    let ruta_nombres: Vec<String> = ruta_indices
                        .iter()
                        .map(|&i| grafo.nombres[i].clone())
                        .collect();
                    println!("¡Tesoro encontrado! Ruta (Dijkstra): {:?}\n", ruta_nombres);
                    escribir_ruta_en_archivo(&ruta_nombres, "ruta_tesoro.txt")?;
                    println!("Ruta guardada en 'ruta_tesoro.txt'.\n");
                }
            }

            0 => {
                println!("\nSaliendo del programa.\n");
                break;
            }

            _ => {
                println!("\nOpción inválida. Intente nuevamente.\n");
            }
        }
    }

    Ok(())
}