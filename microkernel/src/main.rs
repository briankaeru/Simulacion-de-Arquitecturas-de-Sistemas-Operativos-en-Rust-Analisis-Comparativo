use std::fmt;

struct Mensaje {
    origen: String,
    destino: String,
    contenido: String,
}

impl fmt::Display for Mensaje {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} -> {}]: {}", self.origen, self.destino, self.contenido)
    }
}

struct NodoMensaje {
    mensaje: Mensaje,
    siguiente: Option<Box<NodoMensaje>>,
}

struct ColaMensajes {
    cabeza: Option<Box<NodoMensaje>>,
}

impl ColaMensajes {

    fn nueva() -> Self {
        ColaMensajes { cabeza: None }
    }

    fn encolar(&mut self, mensaje: Mensaje) {
        let nuevo_nodo = Box::new(NodoMensaje {
            mensaje,
            siguiente: None,
        });

        match self.cabeza {
            None => {
                self.cabeza = Some(nuevo_nodo);
            }
            Some(ref mut cabeza) => {
                let mut actual = cabeza;
                while actual.siguiente.is_some() {
                    actual = actual.siguiente.as_mut().unwrap();
                }
                actual.siguiente = Some(nuevo_nodo);
            }
        }
    }

    fn desencolar(&mut self) -> Option<Mensaje> {
        self.cabeza.take().map(|nodo| {
            self.cabeza = nodo.siguiente;
            nodo.mensaje
        })
    }

    fn esta_vacia(&self) -> bool {
        self.cabeza.is_none()
    }
}

fn main() {
    println!("=== Simulación: Arquitectura Microkernel ===");
    println!("--- Fase 2: Prueba de Cola de Mensajes IPC ---\n");

    let mut cola = ColaMensajes::nueva();
    println!("[+] Encolando mensajes...");

    let mensajes_prueba = [
        ("ProcesoA", "ServidorDisco", "LEER archivo.txt"),
        ("ProcesoB", "ServidorRed",   "ENVIAR paquete"),
        ("ProcesoC", "ServidorDisco", "ESCRIBIR log.txt"),
    ];

    for (origen, destino, contenido) in &mensajes_prueba {
        let msg = Mensaje {
            origen: origen.to_string(),
            destino: destino.to_string(),
            contenido: contenido.to_string(),
        };
        println!("    Encolado: {}", msg);
        cola.encolar(msg);
    }

    println!("\n[-] Desencolando mensajes (orden FIFO)...");

    while !cola.esta_vacia() {
        if let Some(msg) = cola.desencolar() {
            println!("    Desencolado: {}", msg);
        }
    }
    
    println!("\n[!] Intentando desencolar de cola vacía...");
    match cola.desencolar() {
        Some(msg) => println!("    Mensaje: {}", msg),
        None      => println!("    Cola vacía — no hay mensajes pendientes."),
    }

    println!("\n--- Fin de prueba Fase 2 ---");
}
