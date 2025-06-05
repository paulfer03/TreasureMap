use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::graph::Grafo;
use crate::ubication::{Ubicaciones, InfoUbicacion};

/// DFS guiado por pistas, sin usar `visitado` como filtro estricto.
/// Se basa en `camino_actual` para detectar ciclos, de modo que:
///   1) Si existe `ubicaciones[actual].next`, siempre lo intenta primero (si no está en la ruta actual).
///   2) Si no convence (o genera ciclo), explora los vecinos normales.
/// Al encontrar `tesoro`, clona `camino_actual` en `ruta_encontrada`.
pub fn dfs_buscar_tesoro(
    grafo: &Grafo,
    ubicaciones: &mut Ubicaciones,
    actual: usize,
    tesoro: usize,
    camino_actual: &mut Vec<usize>,
    ruta_encontrada: &mut Option<Vec<usize>>,
) -> bool {
    // Si el nodo `actual` ya está en la ruta parcial, hay un ciclo → cortamos
    if camino_actual.contains(&actual) {
        return false;
    }

    // Añadimos `actual` a la ruta parcial
    camino_actual.push(actual);

    // ¿Llegamos al tesoro?
    if actual == tesoro {
        *ruta_encontrada = Some(camino_actual.clone());
        return true;
    }

    // 1) Intentamos seguir la pista (`next`) antes que explorar vecinos
    if let Some(sig) = ubicaciones[actual].next {
        // Solo si `sig` no genera ciclo en la ruta parcial
        if !camino_actual.contains(&sig) {
            if dfs_buscar_tesoro(grafo, ubicaciones, sig, tesoro, camino_actual, ruta_encontrada) {
                return true;
            }
        }
    }

    // 2) Si no encontramos por la pista, exploramos todos los vecinos no cíclicos
    for arista in &grafo.adyacencia[actual] {
        let vecino = arista.destino;
        if !camino_actual.contains(&vecino) {
            if dfs_buscar_tesoro(grafo, ubicaciones, vecino, tesoro, camino_actual, ruta_encontrada) {
                return true;
            }
        }
    }

    // Backtracking: sacamos `actual` de la ruta parcial
    camino_actual.pop();
    false
}

/// Nodo auxiliar para la cola de prioridad de Dijkstra.
/// Contiene la `costo` acumulada hasta `indice`.
#[derive(Copy, Clone, Eq, PartialEq)]
struct NodoHeap {
    costo: u32,
    indice: usize,
}

// Hacemos que el `BinaryHeap` trate la menor `costo` como prioridad máxima.
impl Ord for NodoHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .costo
            .cmp(&self.costo)
            .then_with(|| self.indice.cmp(&other.indice))
    }
}

impl PartialOrd for NodoHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra: retorna un Vec<usize> con la ruta de menor costo desde `inicio` hasta `destino`.
/// Si no existe camino, devuelve un vector vacío.
pub fn dijkstra_ruta_minima(grafo: &Grafo, inicio: usize, destino: usize) -> Vec<usize> {
    let n = grafo.nombres.len();
    let mut dist: Vec<u32> = vec![u32::MAX; n];
    let mut padres: Vec<Option<usize>> = vec![None; n];
    let mut heap = BinaryHeap::new();

    // Inicializamos
    dist[inicio] = 0;
    heap.push(NodoHeap {
        costo: 0,
        indice: inicio,
    });

    while let Some(NodoHeap { costo, indice }) = heap.pop() {
        // Si ya mejoramos esa entrada, la ignoramos
        if costo > dist[indice] {
            continue;
        }
        // Si llegamos al destino, podemos detenernos
        if indice == destino {
            break;
        }
        // Relajamos cada arista saliente
        for arista in &grafo.adyacencia[indice] {
            let v = arista.destino;
            let nuevo_costo = costo.saturating_add(arista.costo);
            if nuevo_costo < dist[v] {
                dist[v] = nuevo_costo;
                padres[v] = Some(indice);
                heap.push(NodoHeap {
                    costo: nuevo_costo,
                    indice: v,
                });
            }
        }
    }

    // Si no existe camino al destino, devolvemos un Vec vacío
    if dist[destino] == u32::MAX {
        return Vec::new();
    }

    // Reconstruimos la ruta inversa desde `destino` hasta `inicio`
    let mut ruta: Vec<usize> = Vec::new();
    let mut actual = destino;
    while let Some(p) = padres[actual] {
        ruta.push(actual);
        actual = p;
    }
    ruta.push(inicio);
    ruta.reverse();
    ruta
}