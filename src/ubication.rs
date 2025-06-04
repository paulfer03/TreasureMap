// src/ubicaciones.rs

use std::collections::HashMap;
use std::io;
use std::io::BufRead;

/// Info por cada nodo (índice):
/// - `pista`: el texto descriptivo (no usado para lógica, pero útil para consola).
/// - `next`: Option<usize> → índice del “siguiente nodo” que sugiere esta pista.
/// - `visitado`: marca si ya pasamos por aquí durante la DFS.
#[derive(Debug, Clone)]
pub struct InfoUbicacion {
    pub pista: String,
    pub next: Option<usize>,
    pub visitado: bool,
}

/// Representamos las `Ubicaciones` como un Vec<InfoUbicacion>
/// en el que la posición `i` corresponde al nodo índice `i`.
pub type Ubicaciones = Vec<InfoUbicacion>;

/// Crea un Vec<InfoUbicacion> de largo `n`, inicializado con campos vacíos.
/// Luego, quienes llamen a `leer_pistas_dinamico` rellenarán cada índice.
pub fn inicializar_ubicaciones_vacias(n: usize) -> Ubicaciones {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(InfoUbicacion {
            pista: String::new(),
            next: None,
            visitado: false,
        });
    }
    v
}

/// Lee “pistas.txt” con el nuevo formato dinámico:
///   - La primera línea debe ser    Tesoro:<NombreNodo>
///   - A partir de la línea en blanco, cada línea relevante es:
///       NombreNodo,Pista textual,NombreDestino
///
/// Parámetros:
/// - `mapa_nombre_indice`: HashMap<String, usize> para saber el índice de cada nombre.
/// - `ruta_pistas`: la ruta a “pistas.txt”.
///
/// Retorna Ok((Vec<InfoUbicacion>, índice_del_tesoro)) o Err(...) en caso de fallo de E/S o mal formato.
pub fn leer_pistas_dinamico(
    mapa_nombre_indice: &HashMap<String, usize>,
    ruta_pistas: &str,
) -> io::Result<(Ubicaciones, usize)> {
    // 1) Abrir el archivo
    let archivo = std::fs::File::open(ruta_pistas)?;
    let lector = std::io::BufReader::new(archivo);
    let mut lineas = lector.lines();

    // 2) Leer la primera línea: debe ser "Tesoro:NombreNodo"
    let primera = lineas
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Archivo pistas.txt vacío"))??;
    let primera = primera.trim();
    let prefix_tesoro = "Tesoro:";
    if !primera.starts_with(prefix_tesoro) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "El primer renglón de pistas.txt debe comenzar con 'Tesoro:'",
        ));
    }
    let nombre_tesoro = primera[prefix_tesoro.len()..].trim();
    // Buscar su índice en el mapa
    let indice_tesoro = mapa_nombre_indice
        .get(nombre_tesoro)
        .copied()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("El nombre de tesoro '{}' no existe en el grafo", nombre_tesoro),
            )
        })?;

    // 3) Saltar hasta la línea en blanco (o hasta que encontremos la sección de pistas)
    //    (Admitimos que haya comentarios o líneas vacías; buscamos la primera línea no vacía tras "Tesoro:")
    let mut en_seccion_pistas = false;
    for linea in &mut lineas {
        let l = linea?;
        if l.trim().is_empty() {
            // La línea vacía marca que lo siguiente es sección de pistas
            en_seccion_pistas = true;
            break;
        }
        // Si hay comentarios antes (ej. líneas con '#') o espacios, los ignoramos.
    }
    if !en_seccion_pistas {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No se encontró la línea vacía que separa el Tesoro de las pistas",
        ));
    }

    // 4) Inicializar el vector de InfoUbicacion de largo n
    let n = mapa_nombre_indice.len();
    let mut ubicaciones = inicializar_ubicaciones_vacias(n);

    // 5) Ahora, cada línea que no sea vacía se procesa como: NombreNodo,Pista,NombreDestino
    for linea in &mut lineas {
        let linea = linea?;
        let texto = linea.trim();
        if texto.is_empty() || texto.starts_with('#') {
            // saltamos líneas vacías o comentarios
            continue;
        }
        // Dividimos en 3 partes EXACTAS con splitn(3)
        let partes: Vec<&str> = texto.splitn(3, ',').map(|s| s.trim()).collect();
        if partes.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Línea de pistas mal formateada (se esperaban 3 campos): '{}'",
                    texto
                ),
            ));
        }
        let nombre = partes[0];
        let pista_texto = partes[1].to_string();
        let destino_nombre = partes[2]; // puede estar vacío

        // Buscar índice del “nombre” en el grafo
        let idx_origen = mapa_nombre_indice.get(nombre).copied().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Nombre de nodo '{}' en pistas.txt no existe en el grafo", nombre),
            )
        })?;

        // Determinar índice del destino (si existe)
        let idx_destino = if destino_nombre.is_empty() {
            None
        } else {
            let i = mapa_nombre_indice.get(destino_nombre).copied().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "El nodo destino '{}' para '{}' no existe en el grafo",
                        destino_nombre, nombre
                    ),
                )
            })?;
            Some(i)
        };

        // Rellenamos la InfoUbicacion en la posición idx_origen
        ubicaciones[idx_origen].pista = pista_texto;
        ubicaciones[idx_origen].next = idx_destino;
        // visitado ya viene como false por defecto
    }

    Ok((ubicaciones, indice_tesoro))
}