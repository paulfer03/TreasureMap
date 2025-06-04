// src/ubicaciones.rs

use std::collections::HashMap;
use std::io::BufRead;

/// Información de cada ubicación (nodo):
/// - `pista`: cadena con la pista textual.
/// - `visitado`: booleano para marcar si ya lo recorrimos.
///
/// **NOTA DE RENDIMIENTO**: ya no almacenamos el índice dentro de InfoUbicacion,
///    porque el índice coincide con la posición en el Vec<InfoUbicacion>.
#[derive(Debug, Clone)]
pub struct InfoUbicacion {
    pub pista: String,
    pub visitado: bool,
}

/// Representamos las ubicaciones como un Vec<InfoUbicacion> de largo n:
/// el nodo i tiene su InfoUbicacion en ubicaciones[i].
pub type Ubicaciones = Vec<InfoUbicacion>;

impl InfoUbicacion {
    /// Crea una `InfoUbicacion` nueva con la pista dada. Inicialmente `visitado = false`.
    pub fn nueva(pista: String) -> Self {
        InfoUbicacion {
            pista,
            visitado: false,
        }
    }
}

/// ===============================================
/// Funciones de utilidad para inicializar el Vec.
/// ===============================================

/// Dado un número de nodos `n`, crea un Vec<InfoUbicacion> con n elementos vacíos (pista = String::new()).
/// Luego rellenaremos solo aquellos índices que efectivamente aparezcan en pistas.txt.
pub fn inicializar_ubicaciones(n: usize) -> Ubicaciones {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(InfoUbicacion {
            pista: String::new(),
            visitado: false,
        });
    }
    v
}

/// Rellena el `Vec<InfoUbicacion>` (de largo n) leyendo el archivo `pistas.txt`.
/// Para cada línea del tipo "NombreNodo,Pista textual", busca el índice del nodo en
/// `mapa_nombre_indice` y asigna la `pista` en la posición correspondiente del Vec.
///
/// - `mapa_nombre_indice`: HashMap<String, usize> con nombre → índice
/// - `ruta_pistas`: ruta al archivo "pistas.txt"
///
/// Retorna Ok(Vec<InfoUbicacion>) si todo va bien, o Err(…) en caso de formato inválido
/// o si algún nombre de nodo no existe en el grafo.
pub fn leer_pistas_a_vec(
    mapa_nombre_indice: &HashMap<String, usize>,
    ruta_pistas: &str,
) -> std::io::Result<Ubicaciones> {
    let n = mapa_nombre_indice.len();
    let mut ubicaciones = inicializar_ubicaciones(n);

    // Abrimos el archivo:
    let archivo = std::fs::File::open(ruta_pistas)?;
    let lector = std::io::BufReader::new(archivo);

    for linea in lector.lines() {
        let linea = linea?;
        // Cada línea debe tener exactamente 2 partes: nombre,pista
        let partes: Vec<&str> = linea.trim().splitn(2, ',').map(|s| s.trim()).collect();
        if partes.len() != 2 {
            continue; // ignoramos líneas mal formateadas o vacías
        }
        let nombre = partes[0].to_string();
        let pista_texto = partes[1].to_string();

        if let Some(&idx) = mapa_nombre_indice.get(&nombre) {
            // Asignamos la pista al índice idx
            ubicaciones[idx].pista = pista_texto;
        } else {
            // Si aparece un nombre en pistas.txt que no existe en el grafo, error.
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Nodo '{}' en pistas.txt no existe en el grafo", nombre),
            ));
        }
    }

    Ok(ubicaciones)
}