// src/main.rs

mod graph;
mod ubication;
mod io_utils;
mod search;

use crate::graph::Grafo;
use crate::ubication::leer_pistas_dinamico;
use crate::io_utils::{leer_grafo_desde_archivo, escribir_ruta_en_archivo};
use crate::search::dfs_buscar_tesoro;
use std::io;

fn main() -> io::Result<()> {
    // 1) Leer el grafo de "grafo.txt"
    let ruta_grafo = "grafo.txt";
    println!("Cargando grafo desde '{}'", ruta_grafo);
    let grafo: Grafo = leer_grafo_desde_archivo(ruta_grafo)?;
    println!("Grafo cargado con {} nodos.", grafo.nombres.len());

    // 2) Leer las pistas dinámicas de "pistas.txt" y obtener (Ubicaciones, indice_tesoro)
    let ruta_pistas = "pistas.txt";
    println!("Cargando pistas dinámicas desde '{}'", ruta_pistas);
    let (mut ubicaciones, indice_tesoro) =
        leer_pistas_dinamico(&grafo.mapa_nombre_indice, ruta_pistas)?;
    println!(
        "Índice del tesoro (dinámico) = {} ('{}').",
        indice_tesoro, grafo.nombres[indice_tesoro]
    );

    // 3) Definir nodo de inicio. Por ejemplo “Beach” (puedes cambiarlo si quieres)
    let nombre_inicio = "Beach";
    let indice_inicio = grafo
        .indice_por_nombre(nombre_inicio)
        .unwrap_or_else(|| {
            panic!("El nodo de inicio '{}' no existe en grafo.txt", nombre_inicio)
        });
    println!("El pirata inicia en '{}' (índice {}).", nombre_inicio, indice_inicio);

    // 4) Llamar a DFS
    let mut camino_actual: Vec<usize> = Vec::new();
    let mut ruta_encontrada: Option<Vec<usize>> = None;

    let hallado = dfs_buscar_tesoro(
        &grafo,
        &mut ubicaciones,
        indice_inicio,
        indice_tesoro,
        &mut camino_actual,
        &mut ruta_encontrada,
    );

    // 5) Si se encuentra, convertir índices en nombres y guardar en "ruta_tesoro.txt"
    if hallado {
        if let Some(vec_indices) = ruta_encontrada {
            let ruta_nombres: Vec<String> = vec_indices
                .iter()
                .map(|&idx| grafo.nombres[idx].clone())
                .collect();
            println!("¡Tesoro encontrado! Ruta: {:?}", ruta_nombres);

            let archivo_salida = "ruta_tesoro.txt";
            escribir_ruta_en_archivo(&ruta_nombres, archivo_salida)?;
            println!("Ruta guardada en '{}'.", archivo_salida);
        }
    } else {
        println!("No se encontró el tesoro siguiendo las pistas dinámicas.");
    }

    Ok(())
}