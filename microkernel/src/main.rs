use std::fmt;

// mensaje que se pasa entre procesos via ipc
struct Mensaje {
    origen: String,
    destino: String,
    contenido: String,
}

impl fmt::Display for Mensaje {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} -> {}]: {}",
            self.origen, self.destino, self.contenido
        )
    }
}

// nodo de la lista enlazada
struct NodoMensaje {
    mensaje: Mensaje,
    siguiente: Option<Box<NodoMensaje>>,
}

// cola fifo con lista enlazada
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

// servidor que corre en espacio de usuario y atiende peticiones de disco
struct ServidorDisco {
    nombre: String,
}

impl ServidorDisco {
    fn nuevo(nombre: &str) -> Self {
        ServidorDisco {
            nombre: nombre.to_string(),
        }
    }

    // procesa un mensaje y devuelve una respuesta
    fn atender(&self, msg: &Mensaje) -> Mensaje {
        let respuesta = format!("OK: '{}' procesado por {}", msg.contenido, self.nombre);
        println!("    [{}] procesando: {}", self.nombre, msg.contenido);

        Mensaje {
            origen: self.nombre.clone(),
            destino: msg.origen.clone(),
            contenido: respuesta,
        }
    }
}

// el microkernel solo enruta mensajes entre procesos y servidores
struct Microkernel {
    cola_entrada: ColaMensajes,
    cola_salida: ColaMensajes,
}

impl Microkernel {
    fn nuevo() -> Self {
        Microkernel {
            cola_entrada: ColaMensajes::nueva(),
            cola_salida: ColaMensajes::nueva(),
        }
    }

    // recibe un mensaje de un proceso y lo pone en la cola de entrada
    fn enviar(&mut self, msg: Mensaje) {
        println!("  [Kernel] enrutando mensaje: {}", msg);
        self.cola_entrada.encolar(msg);
    }

    // despacha todos los mensajes pendientes al servidor correspondiente
    fn despachar(&mut self, servidor: &ServidorDisco) {
        println!("\n  [Kernel] despachando mensajes al servidor...");
        while !self.cola_entrada.esta_vacia() {
            if let Some(msg) = self.cola_entrada.desencolar() {
                let respuesta = servidor.atender(&msg);
                self.cola_salida.encolar(respuesta);
            }
        }
    }

    // entrega las respuestas a los procesos de usuario
    fn entregar_respuestas(&mut self) {
        println!("\n  [Kernel] entregando respuestas a procesos...");
        while !self.cola_salida.esta_vacia() {
            if let Some(resp) = self.cola_salida.desencolar() {
                println!("    -> {}", resp);
            }
        }
    }
}

// proceso de usuario que hace peticiones al microkernel
struct ProcesoUsuario {
    nombre: String,
}

impl ProcesoUsuario {
    fn nuevo(nombre: &str) -> Self {
        ProcesoUsuario {
            nombre: nombre.to_string(),
        }
    }

    // crea un mensaje dirigido al servidor de disco
    fn solicitar(&self, operacion: &str, servidor_destino: &str) -> Mensaje {
        println!("  [{}] solicita: {}", self.nombre, operacion);
        Mensaje {
            origen: self.nombre.clone(),
            destino: servidor_destino.to_string(),
            contenido: operacion.to_string(),
        }
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Simulación: Arquitectura Microkernel      ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // crear componentes del sistema
    let mut kernel = Microkernel::nuevo();
    let servidor = ServidorDisco::nuevo("ServidorDisco");
    let proc_a = ProcesoUsuario::nuevo("ProcesoA");
    let proc_b = ProcesoUsuario::nuevo("ProcesoB");
    let proc_c = ProcesoUsuario::nuevo("ProcesoC");

    // los procesos hacen peticiones que pasan por el kernel
    println!("── Fase 1: Procesos envían solicitudes ──");
    let msg1 = proc_a.solicitar("LEER archivo.txt", "ServidorDisco");
    kernel.enviar(msg1);

    let msg2 = proc_b.solicitar("ESCRIBIR log.txt", "ServidorDisco");
    kernel.enviar(msg2);

    let msg3 = proc_c.solicitar("ELIMINAR temp.dat", "ServidorDisco");
    kernel.enviar(msg3);

    // el kernel despacha al servidor
    println!("\n── Fase 2: Kernel despacha al servidor ──");
    kernel.despachar(&servidor);

    // el kernel devuelve las respuestas
    println!("\n── Fase 3: Kernel entrega respuestas ──");
    kernel.entregar_respuestas();

    println!("\n══ Simulación finalizada ══");
}
