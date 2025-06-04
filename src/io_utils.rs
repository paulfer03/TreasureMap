

use crate::graph::{Arista, Grafo};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

/// ======================================
/// Lectura de grafo desde "grafo.txt"
/// ======================================

/// Formato esperado (ejemplo):
///   Nodos: Beach,Cave,Mountain,Forest,Woods,Treasure
///   Aristas:
///   Beach,Cave,3
///   Beach,Forest,5
///   Cave,Mountain,2
///   Forest,Woods,1
///   Mountain,Treasure,4
///   Woods,Treasure,6
///
/// Devuelve un `Grafo` completamente construido, con:
///  - `nombres`: Vec<String> con cada nodo en orden.
///  - `mapa_nombre_indice`: HashMap<String, usize> de nombre→índice.
///  - `adyacencia`: Vec<Vec<Arista>> con las conexiones no dirigidas.
///
/// Si hay un error de formato, se retorna Err(...).
pub fn leer_grafo_desde_archivo(ruta: &str) -> io::Result<Grafo> {
    let archivo = File::open(ruta)?;
    let lector = BufReader::new(archivo);
    let mut lineas = lector.lines();

    // 1) Primera línea: "Nodos: A,B,C,D"
    let primera = lineas
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Archivo vacío"))??;
    let prefix = "Nodos:";
    if !primera.starts_with(prefix) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Formato inválido: se esperaba 'Nodos:'",
        ));
    }
    let lista_nombres = primera[prefix.len()..].trim();
    let nombres_vec: Vec<String> = lista_nombres
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let n = nombres_vec.len();
    let mut grafo = Grafo::new(n);

    // Guardamos los nombres y construimos el HashMap nombre→índice
    grafo.nombres = nombres_vec.clone();
    for (i, nombre) in nombres_vec.into_iter().enumerate() {
        grafo.mapa_nombre_indice.insert(nombre, i);
    }

    // 2) Línea "Aristas:"
    let linea_aristas = lineas
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Faltó sección 'Aristas:'"))??;
    if linea_aristas.trim() != "Aristas:" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Formato inválido: se esperaba 'Aristas:'",
        ));
    }

    // 3) Cada línea posterior: "origen,destino,costo"
    for linea in lineas {
        let linea = linea?;
        let partes: Vec<&str> = linea.trim().split(',').map(|s| s.trim()).collect();
        if partes.len() != 3 {
            continue; // saltamos líneas vacías o mal formateadas
        }
        let origen = partes[0];
        let destino = partes[1];
        let costo: u32 = partes[2].parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Costo no es un número válido")
        })?;

        // Buscamos índices de origen y destino
        if let (Some(&i_origen), Some(&i_destino)) = (
            grafo.mapa_nombre_indice.get(origen),
            grafo.mapa_nombre_indice.get(destino),
        ) {
            grafo.agregar_arista_no_dirigida(i_origen, i_destino, costo);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Nombre de nodo desconocido: {} o {}", origen, destino),
            ));
        }
    }

    Ok(grafo)
}

/// ================================================
/// Escritura de la ruta encontrada en un archivo.
/// ================================================

/// Recibe un Vec<String> con los nombres de nodos (en orden) y escribe línea a línea
/// en `ruta_salida` (por ejemplo "ruta_tesoro.txt").
/// Si ocurre cualquier error de E/S, lo propaga hacia el llamador.
pub fn escribir_ruta_en_archivo(ruta: &[String], ruta_salida: &str) -> io::Result<()> {
    let mut archivo = File::create(ruta_salida)?;
    for ubic in ruta {
        writeln!(archivo, "{}", ubic)?;
    }
    Ok(())
}