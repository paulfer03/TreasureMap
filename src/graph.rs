

use std::collections::HashMap;

/// Una arista conecta el nodo implícito (su posición en el Vec) con `destino`.
/// `costo` es el peso (u32) de esa conexión.
#[derive(Debug, Clone)]
pub struct Arista {
    pub destino: usize,
    pub costo: u32,
}

/// Grafo representado por lista de adyacencia.
/// - `adyacencia[i]` es un Vec<Arista> de todas las aristas salientes del nodo i.
/// - `nombres[i]` es el nombre (String) del nodo i.
/// - `mapa_nombre_indice` permite, en O(1), ir de nombre→índice.
#[derive(Debug)]
pub struct Grafo {
    pub adyacencia: Vec<Vec<Arista>>,
    pub nombres: Vec<String>,
    pub mapa_nombre_indice: HashMap<String, usize>,
}

impl Grafo {
    /// Crea un grafo vacío con `n` nodos (sin aristas).
    /// Las `nombres` se rellenarán después manualmente.
    pub fn new(n: usize) -> Self {
        Grafo {
            adyacencia: vec![Vec::new(); n],
            nombres: Vec::with_capacity(n),
            mapa_nombre_indice: HashMap::with_capacity(n),
        }
    }

    /// Agrega una arista no dirigida entre `u` y `v` con peso `costo`.
    pub fn agregar_arista_no_dirigida(&mut self, u: usize, v: usize, costo: u32) {
        self.adyacencia[u].push(Arista { destino: v, costo });
        self.adyacencia[v].push(Arista { destino: u, costo });
    }

    /// Devuelve Some(índice) si existe el nodo llamado `nombre`, o None en caso contrario.
    pub fn indice_por_nombre(&self, nombre: &str) -> Option<usize> {
        self.mapa_nombre_indice.get(nombre).copied()
    }
}