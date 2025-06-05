use std::collections::HashMap;
use std::io;
use std::io::BufRead;

/// Info por cada nodo (índice):
/// - `pista`: el texto descriptivo (útil para mostrar en consola).
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

/// Lee “pistas.txt” con el formato dinámico:
///
///   Tesoro:NombreNodo
///
///   # (línea vacía o comentarios opcionales)
///   NombreNodo,Pista textual,NombreDestino
///
/// Retorna Ok((Vec<InfoUbicacion>, índice_del_tesoro)) o Err en caso de fallo.
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

    // 3) Saltar hasta la línea en blanco (separador)
    let mut en_seccion_pistas = false;
    for linea in &mut lineas {
        let l = linea?;
        if l.trim().is_empty() {
            en_seccion_pistas = true;
            break;
        }
    }
    if !en_seccion_pistas {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No se encontró la línea vacía que separa el Tesoro de las pistas",
        ));
    }

    // 4) Inicializar el vector de ubicaciones
    let n = mapa_nombre_indice.len();
    let mut ubicaciones = inicializar_ubicaciones_vacias(n);

    // 5) Cada línea: "NombreNodo,Pista textual,NombreDestino"
    for linea in &mut lineas {
        let linea = linea?;
        let texto = linea.trim();
        if texto.is_empty() || texto.starts_with('#') {
            continue;
        }
        let partes: Vec<&str> = texto.splitn(3, ',').map(|s| s.trim()).collect();
        if partes.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Línea mal formateada: '{}'", texto),
            ));
        }
        let nombre = partes[0];
        let pista_texto = partes[1].to_string();
        let destino_nombre = partes[2]; // puede estar vacío

        // Buscar índice del “nombre” en el grafo
        let idx_origen = mapa_nombre_indice.get(nombre).copied().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Nodo '{}' en pistas.txt no existe en el grafo", nombre),
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

        // Rellenar InfoUbicacion
        ubicaciones[idx_origen].pista = pista_texto;
        ubicaciones[idx_origen].next = idx_destino;
    }

    Ok((ubicaciones, indice_tesoro))
}