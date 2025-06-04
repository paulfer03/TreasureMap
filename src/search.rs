
use crate::graph::Grafo;
use crate::ubication::{Ubicaciones, InfoUbicacion};

/// ----------------------------------------------------
/// DFS recursivo para buscar el tesoro en el grafo.
/// - `grafo`: referencia al grafo completo.
/// - `ubicaciones`: Vec<InfoUbicacion> donde el índice coincide con el nodo.
/// - `arbol_decision`: árbol para interpretar cada pista textual.
/// - `indice_actual`: nodo donde estamos parados.
/// - `camino_actual`: Vec<usize> que acumula índices de nodos visitados en la rama actual.
/// - `ruta_encontrada`: Option<Vec<usize>> donde guardamos la ruta final (si se halla el tesoro).
///
/// Retorna `true` si encontramos el tesoro en esta rama, `false` si no.
///
/// Cuando encontramos el tesoro, clonamos `camino_actual` en `ruta_encontrada` y retornamos true
/// para propagar la señal hacia arriba y detener más búsquedas.
pub fn dfs_buscar_tesoro(
    grafo: &Grafo,
    ubicaciones: &mut Ubicaciones,
    arbol_decision: &crate::decision::NodoDecision,
    indice_actual: usize,
    camino_actual: &mut Vec<usize>,
    ruta_encontrada: &mut Option<Vec<usize>>,
) -> bool {
    // 1) Marcamos el nodo actual como visitado.
    if ubicaciones[indice_actual].visitado {
        return false;
    }
    ubicaciones[indice_actual].visitado = true;

    // 2) Añadimos al camino
    camino_actual.push(indice_actual);

    // 3) ¿Es este nodo el "Treasure"? (comparo por nombre en grafo)
    let nombre_actual = &grafo.nombres[indice_actual];
    let nom_lower = nombre_actual.to_lowercase();
    if nom_lower.contains("treasure") || nom_lower == "treasure" {
        // ¡Lo encontramos!
        *ruta_encontrada = Some(camino_actual.clone());
        return true;
    }

    // 4) Interpretar la pista textual de este nodo para saber a qué índice ir.
    let pista_texto = &ubicaciones[indice_actual].pista;
    if !pista_texto.is_empty() {
        if let Some(siguiente_indice) = arbol_decision.interpretar(pista_texto) {
            if !ubicaciones[siguiente_indice].visitado {
                // Llamada recursiva
                if dfs_buscar_tesoro(
                    grafo,
                    ubicaciones,
                    arbol_decision,
                    siguiente_indice,
                    camino_actual,
                    ruta_encontrada,
                ) {
                    return true;
                }
            }
        }
    }

    // 5) Si la pista nos dio un índice ya visitado o no existe pista,,
    //    exploramos *todas* las aristas adyacentes para no quedarnos bloqueados.

    for arista in &grafo.adyacencia[indice_actual] {
        let vecino = arista.destino;
        if !ubicaciones[vecino].visitado {
            if dfs_buscar_tesoro(
                grafo,
                ubicaciones,
                arbol_decision,
                vecino,
                camino_actual,
                ruta_encontrada,
            ) {
                return true;
            }
        }
    }

    // 6) Retrocedemos: desmarcamos `visitado` (para otras ramas) y sacamos del camino actual.
    ubicaciones[indice_actual].visitado = false;
    camino_actual.pop();

    false
}

/// ----------------------------------------------------
/// (Opcional) Función de búsqueda BFS pura si quisieras
/// encontrar la ruta de menor número de aristas.
/// La dejamos aquí solo como referencia.
/*
use std::collections::VecDeque;

/// BFS para hallar el camino más corto (en número de pasos) hasta el nodo "Treasure".
/// Retorna Option<Vec<usize>> con la secuencia de índices de nodos de la ruta, o None si no se halla.
pub fn bfs_encontrar_tesoro(
    grafo: &Grafo,
    indice_inicio: usize,
) -> Option<Vec<usize>> {
    let n = grafo.nombres.len();
    let mut visitado = vec![false; n];
    let mut padre: Vec<Option<usize>> = vec![None; n];
    let mut cola = VecDeque::new();

    visitado[indice_inicio] = true;
    cola.push_back(indice_inicio);

    let mut indice_tesoro: Option<usize> = None;
    while let Some(u) = cola.pop_front() {
        let nombre_u = &grafo.nombres[u];
        if nombre_u.to_lowercase().contains("treasure") {
            indice_tesoro = Some(u);
            break;
        }
        for arista in &grafo.adyacencia[u] {
            let v = arista.destino;
            if !visitado[v] {
                visitado[v] = true;
                padre[v] = Some(u);
                cola.push_back(v);
            }
        }
    }

    if let Some(idx_t) = indice_tesoro {
        // Reconstruir la ruta al revés
        let mut ruta_rev: Vec<usize> = Vec::new();
        let mut actual = Some(idx_t);
        while let Some(i) = actual {
            ruta_rev.push(i);
            actual = padre[i];
        }
        ruta_rev.reverse();
        return Some(ruta_rev);
    }

    None
}
*/