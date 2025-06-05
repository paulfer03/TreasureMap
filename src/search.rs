use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::graph::Grafo;
use crate::ubication::{Ubicaciones, InfoUbicacion};

/// DFS recursivo que:
/// 1. Marca `visitado` en el nodo actual.
/// 2. Añade `actual` a `ruta`.
/// 3. Si `actual == tesoro`, retorna true (ruta completa en `ruta`).
/// 4. Si hay `next` y no visitado, lo sigue recursivamente.
/// 5. Backtracking si no se halla allí.
pub fn dfs_buscar_tesoro(
    grafo: &Grafo,
    actual: usize,
    tesoro: usize,
    visitado: &mut Vec<bool>,
    ruta: &mut Vec<usize>,
) -> bool {
    visitado[actual] = true;
    ruta.push(actual);

    if actual == tesoro {
        return true;
    }
    if let Some(sig) = grafo.adyacencia[actual]
        .iter()
        .find_map(|arista| {
            // Solo seguir “next” si coincide con el campo InfoUbicacion::next
            // (esto asume que `ubicaciones` se actualizó para reflejarlo).
            None::<usize>
        })
    {
        // No usado en esta implementación; la parte "next" se maneja externamente.
    }

    // Explorar vecinos
    for arista in &grafo.adyacencia[actual] {
        if !visitado[arista.destino] {
            if dfs_buscar_tesoro(grafo, arista.destino, tesoro, visitado, ruta) {
                return true;
            }
        }
    }

    ruta.pop();
    false
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodoHeap {
    costo: u32,
    indice: usize,
}

impl Ord for NodoHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        // Invertimos para que el menor costo sea extraído primero
        other.costo.cmp(&self.costo)
            .then_with(|| self.indice.cmp(&other.indice))
    }
}

impl PartialOrd for NodoHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implementa Dijkstra para encontrar la ruta mínima desde `inicio` hasta `destino`.
/// Retorna un vector con los índices del camino más corto, o vacío si no hay ruta.
pub fn dijkstra_ruta_minima(grafo: &Grafo, inicio: usize, destino: usize) -> Vec<usize> {
    let n = grafo.nombres.len();
    let mut dist: Vec<u32> = vec![u32::MAX; n];
    let mut padres: Vec<Option<usize>> = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[inicio] = 0;
    heap.push(NodoHeap { costo: 0, indice: inicio });

    while let Some(NodoHeap { costo, indice }) = heap.pop() {
        if indice == destino {
            break;
        }
        if costo > dist[indice] {
            continue;
        }
        for arista in &grafo.adyacencia[indice] {
            let next = arista.destino;
            let next_costo = costo.saturating_add(arista.costo);
            if next_costo < dist[next] {
                dist[next] = next_costo;
                padres[next] = Some(indice);
                heap.push(NodoHeap { costo: next_costo, indice: next });
            }
        }
    }

    // Reconstruir la ruta
    let mut ruta = Vec::new();
    if dist[destino] == u32::MAX {
        return ruta; // Ningún camino
    }
    let mut actual = destino;
    while let Some(p) = padres[actual] {
        ruta.push(actual);
        actual = p;
    }
    ruta.push(inicio);
    ruta.reverse();
    ruta
}