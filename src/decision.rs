// src/decision.rs

/// Estructura del árbol de decisión para interpretar pistas de texto.
///
/// Cada nodo contiene:
///  - `texto_clave`: palabra o frase clave que buscamos dentro de la pista (minúscula).
///  - `hija_verdadero` y `hija_falso`: subárboles según contenga o no la pista esa palabra.
///  - `resultado`: si es hoja, Some(indice_nodo_grafo); si no es hoja, None.
#[derive(Debug)]
pub struct NodoDecision {
    pub texto_clave: String,
    pub hija_verdadero: Option<Box<NodoDecision>>,
    pub hija_falso: Option<Box<NodoDecision>>,
    pub resultado: Option<usize>,
}

impl NodoDecision {
    /// Crea un nodo hoja con un índice de resultado.
    pub fn nueva_hoja(resultado: usize) -> Self {
        NodoDecision {
            texto_clave: String::new(),
            hija_verdadero: None,
            hija_falso: None,
            resultado: Some(resultado),
        }
    }

    /// Crea un nodo interno con palabra clave `clave`, apuntando a dos subárboles opcionales.
    pub fn nuevo_nodo(
        clave: &str,
        hija_v: Option<NodoDecision>,
        hija_f: Option<NodoDecision>,
    ) -> Self {
        NodoDecision {
            texto_clave: clave.to_lowercase(),
            hija_verdadero: hija_v.map(Box::new),
            hija_falso: hija_f.map(Box::new),
            resultado: None,
        }
    }

    /// Recorre recursivamente el árbol con la `pista_texto` (en mayúsculas o minúsculas),
    /// devolviendo Some(indice_nodo) si encontró una hoja, o None si algo falla.
    pub fn interpretar(&self, pista_texto: &str) -> Option<usize> {
        // Si es hoja, devolvemos el `resultado`.
        if let Some(res) = self.resultado {
            return Some(res);
        }
        // Si no es hoja, comparamos `texto_clave` con la pista (todo en minúsculas).
        let pista_lower = pista_texto.to_lowercase();
        if pista_lower.contains(&self.texto_clave) {
            // Vamos por la rama verdadera
            if let Some(ref v) = self.hija_verdadero {
                return v.interpretar(pista_texto);
            } else {
                return None;
            }
        } else {
            // Rama falsa
            if let Some(ref f) = self.hija_falso {
                return f.interpretar(pista_texto);
            } else {
                return None;
            }
        }
    }
}

/// ----------------------------------------------------
/// Función de ejemplo para construir el árbol de pistas.
/// ----------------------------------------------------
/// En tu proyecto real, debes adaptar estos índices a los que tenga tu grafo.
/// Por ejemplo, si `Beach` es índice 0, `Cave` índice 1, etc.
///
/// Ejemplo de lógica:
///  - Si la pista contiene "brilla" → Beach (índice 0)
///  - Si no contiene "brilla" pero contiene "sol" → Cave (índice 1)
///  - Si no contiene "brilla" ni "sol" pero contiene "árbol" → Forest (índice 3)
///  - Si no, si contiene "murmullo" → Woods (índice 4)
///  - Si no, si contiene "cima" → Mountain (índice 2)
///  - En último caso, Treasure (índice 5)
pub fn construir_arbol_decision_ejemplo() -> NodoDecision {
    // Las hojas con sus índices de ejemplo:
    let hoja_beach = NodoDecision::nueva_hoja(0);
    let hoja_cave = NodoDecision::nueva_hoja(1);
    let hoja_mountain = NodoDecision::nueva_hoja(2);
    let hoja_forest = NodoDecision::nueva_hoja(3);
    let hoja_woods = NodoDecision::nueva_hoja(4);
    let hoja_treasure = NodoDecision::nueva_hoja(5);

    // Nivel 1: "brilla"?
    let nodo_brilla = NodoDecision::nuevo_nodo(
        "brilla",
        Some(hoja_beach), // si contiene "brilla" → Beach
        Some(NodoDecision::nuevo_nodo(
            "sol",
            Some(hoja_cave), // si contiene "sol" → Cave
            Some(NodoDecision::nuevo_nodo(
                "árbol",
                Some(hoja_forest), // si "árbol" → Forest
                Some(NodoDecision::nuevo_nodo(
                    "murmullo",
                    Some(hoja_woods), // si "murmullo" → Woods
                    Some(NodoDecision::nuevo_nodo(
                        "cima",
                        Some(hoja_mountain), // si "cima" → Mountain
                        Some(hoja_treasure),  // sino → Treasure
                    )),
                )),
            )),
        )),
    );

    nodo_brilla
}