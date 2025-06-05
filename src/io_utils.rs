use crate::graph::{Arista, Grafo};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

/// Lee `grafo.txt` con formato:
///   Nodos: A,B,C,...
///   Aristas:
///   A,B,3
///   B,C,5
pub fn leer_grafo_desde_archivo(ruta: &str) -> io::Result<Grafo> {
    let archivo = File::open(ruta)?;
    let lector = BufReader::new(archivo);
    let mut lineas = lector.lines();

    // 1) Leer "Nodos: A,B,C,..."
    let primera = lineas
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Archivo vacío"))??;
    let prefix = "Nodos:";
    if !primera.starts_with(prefix) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Se esperaba línea que comience con 'Nodos:'",
        ));
    }
    let lista_nombres = primera[prefix.len()..].trim();
    let nombres_vec: Vec<String> = lista_nombres
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let n = nombres_vec.len();
    let mut grafo = Grafo::new(n);

    // Guardar nombres e índice
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
            "Se esperaba línea 'Aristas:'",
        ));
    }

    // 3) Cada línea posterior: "origen,destino,costo"
    for linea in lineas {
        let linea = linea?;
        let partes: Vec<&str> = linea.trim().split(',').map(|s| s.trim()).collect();
        if partes.len() != 3 {
            // Saltar líneas vacías o comentarios
            continue;
        }
        let origen = partes[0];
        let destino = partes[1];
        let costo: u32 = partes[2].parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Costo no es un número válido")
        })?;

        // Obtener índices
        if let (Some(&i_origen), Some(&i_destino)) = (
            grafo.mapa_nombre_indice.get(origen),
            grafo.mapa_nombre_indice.get(destino),
        ) {
            grafo.agregar_arista_no_dirigida(i_origen, i_destino, costo);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Nodo desconocido: '{}' o '{}'", origen, destino),
            ));
        }
    }

    Ok(grafo)
}

/// Escribe la ruta encontrada en un archivo (una línea por ubicación).
pub fn escribir_ruta_en_archivo(ruta: &[String], ruta_salida: &str) -> io::Result<()> {
    let mut archivo = File::create(ruta_salida)?;
    for ubic in ruta {
        writeln!(archivo, "{}", ubic)?;
    }
    Ok(())
}