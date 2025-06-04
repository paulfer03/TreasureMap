// src/busqueda.rs

use crate::graph::Grafo;
use crate::ubication::{Ubicaciones, InfoUbicacion};

/// DFS recursivo que:
/// 1. Marca `visitado` en el nodo actual.
/// 2. Añade `indice_actual` a `camino_actual`.
/// 3. Si `indice_actual == indice_tesoro`, guarda `camino_actual` en `ruta_encontrada` y retorna true.
/// 4. Si existe `ubicaciones[indice_actual].next` y no está visitado, recursivamente vamos allí.
/// 5. Si eso falla, intentamos todos los vecinos adyacentes del grafo.
/// 6. Al deshacernos (backtrack), desmarcamos `visitado` y sacamos de `camino_actual`.
///
/// Parámetros:
/// - `grafo`: referencia al grafo completo.
/// - `ubicaciones`: &mut Vec<InfoUbicacion>, donde `i` es la info del nodo i.
/// - `indice_actual`: el índice del nodo donde estoy parado.
/// - `indice_tesoro`: el índice del nodo que identifica al tesoro.
/// - `camino_actual`: &mut Vec<usize> con los índices por donde llevo pasados.
/// - `ruta_encontrada`: &mut Option<Vec<usize>> donde se guardará la ruta final si hallamos el tesoro.
pub fn dfs_buscar_tesoro(
    grafo: &Grafo,
    ubicaciones: &mut Ubicaciones,
    indice_actual: usize,
    indice_tesoro: usize,
    camino_actual: &mut Vec<usize>,
    ruta_encontrada: &mut Option<Vec<usize>>,
) -> bool {
    // 1) Si ya lo visitamos, cortamos
    if ubicaciones[indice_actual].visitado {
        return false;
    }
    // Marcamos visitado
    ubicaciones[indice_actual].visitado = true;

    // 2) Añadir al camino actual
    camino_actual.push(indice_actual);

    // 3) Si es tesoro, guardamos la ruta (clon) y devolvemos true
    if indice_actual == indice_tesoro {
        *ruta_encontrada = Some(camino_actual.clone());
        return true;
    }

    // 4) Tratamos de seguir la pista dinámica:
    if let Some(sig) = ubicaciones[indice_actual].next {
        if !ubicaciones[sig].visitado {
            if dfs_buscar_tesoro(
                grafo,
                ubicaciones,
                sig,
                indice_tesoro,
                camino_actual,
                ruta_encontrada,
            ) {
                return true;
            }
        }
    }

    // 5) Si la pista no me llevó o estaba visitado, lo resolvemos explorando todos los vecinos:
    for arista in &grafo.adyacencia[indice_actual] {
        let vecino = arista.destino;
        if !ubicaciones[vecino].visitado {
            if dfs_buscar_tesoro(
                grafo,
                ubicaciones,
                vecino,
                indice_tesoro,
                camino_actual,
                ruta_encontrada,
            ) {
                return true;
            }
        }
    }

    // 6) Backtrack: desmarcamos “visitado” y sacamos del camino
    ubicaciones[indice_actual].visitado = false;
    camino_actual.pop();
    false
}