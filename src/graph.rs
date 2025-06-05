use std::collections::HashMap;

/// Cada arista conecta implícitamente el nodo “u” (su índice en el Vec) con `destino` y un `costo`.
#[derive(Debug, Clone)]
pub struct Arista {
    pub destino: usize,
    pub costo: u32,
}

/// Grafo con lista de adyacencia.
/// - `nombres[i]` es el nombre del nodo i.
/// - `mapa_nombre_indice` mapea String→usize en O(1).
/// - `adyacencia[i]` es Vec<Arista> con todos los vecinos de i.
#[derive(Debug)]
pub struct Grafo {
    pub nombres: Vec<String>,
    pub mapa_nombre_indice: HashMap<String, usize>,
    pub adyacencia: Vec<Vec<Arista>>,
}

impl Grafo {
    /// Crea un grafo vacío de `n` nodos (sin aristas).
    /// Posteriormente se agrega `nombres` y `mapa_nombre_indice`.
    pub fn new(n: usize) -> Self {
        Grafo {
            nombres: Vec::with_capacity(n),
            mapa_nombre_indice: HashMap::with_capacity(n),
            adyacencia: vec![Vec::new(); n],
        }
    }

    /// Agrega una arista no dirigida (bidireccional) entre u y v con peso `costo`.
    pub fn agregar_arista_no_dirigida(&mut self, u: usize, v: usize, costo: u32) {
        self.adyacencia[u].push(Arista { destino: v, costo });
        self.adyacencia[v].push(Arista { destino: u, costo });
    }

    /// Devuelve Some(índice) si existe `nombre` en el grafo, o None si no existe.
    pub fn indice_por_nombre(&self, nombre: &str) -> Option<usize> {
        self.mapa_nombre_indice.get(nombre).copied()
    }
}