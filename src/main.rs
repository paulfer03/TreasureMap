mod graph;
mod ubication;
mod io_utils;
mod decision;
mod search;


use crate::graph::Grafo;
use crate::ubication::{leer_pistas_a_vec, Ubicaciones};
use crate::io_utils::{leer_grafo_desde_archivo, escribir_ruta_en_archivo};
use crate::decision::construir_arbol_decision_ejemplo;
use crate::search::dfs_buscar_tesoro;
use std::io;

fn main() -> io::Result<()> {
    // 1) Leer el grafo de "grafo.txt"
    let ruta_grafo = "grafo.txt";
    println!("Cargando grafo desde '{}'", ruta_grafo);
    let mut grafo: Grafo = leer_grafo_desde_archivo(ruta_grafo)?;
    println!("Grafo cargado con {} nodos.", grafo.nombres.len());

    // 2) Leer las pistas y construir Vec<InfoUbicacion>
    let ruta_pistas = "pistas.txt";
    println!("Cargando pistas desde '{}'", ruta_pistas);
    // Pasamos `&grafo.mapa_nombre_indice` para mapear nombre→índice:
    let mut ubicaciones: Ubicaciones =
        leer_pistas_a_vec(&grafo.mapa_nombre_indice, ruta_pistas)?;
    println!(
        "Se crearon {} entradas de InfoUbicacion (Vec indexado).",
        ubicaciones.len()
    );

    // 3) Construir el árbol de decisión (ejemplo)
    //    **ATENCIÓN**: ajusta los índices en función del orden real de "Nodos:" en tu grafo.txt
    let arbol_decision = construir_arbol_decision_ejemplo();
    println!("Árbol de decisión construido.");

    // 4) Escoger el índice de inicio, por ejemplo "Beach"
    let indice_inicio = match grafo.indice_por_nombre("Beach") {
        Some(i) => i,
        None => {
            println!("No existe el nodo 'Beach' en el grafo.");
            return Ok(());
        }
    };
    println!(
        "El pirata inicia en '{}'.",
        grafo.nombres[indice_inicio]
    );

    // 5) Ejecutar DFS para encontrar el tesoro
    let mut camino_actual: Vec<usize> = Vec::new();
    let mut ruta_encontrada: Option<Vec<usize>> = None;

    let hallado = dfs_buscar_tesoro(
        &grafo,
        &mut ubicaciones,
        &arbol_decision,
        indice_inicio,
        &mut camino_actual,
        &mut ruta_encontrada,
    );

    if hallado {
        if let Some(vec_indices) = ruta_encontrada {
            // Convertir índices a nombres
            let ruta_nombres: Vec<String> = vec_indices
                .iter()
                .map(|&idx| grafo.nombres[idx].clone())
                .collect();

            println!("¡Tesoro encontrado! Ruta: {:?}", ruta_nombres);

            // 6) Guardar la ruta en archivo
            let archivo_salida = "ruta_tesoro.txt";
            escribir_ruta_en_archivo(&ruta_nombres, archivo_salida)?;
            println!("Ruta guardada en '{}'.", archivo_salida);
        }
    } else {
        println!("No se encontró el tesoro siguiendo las pistas.");
    }

    Ok(())
}
