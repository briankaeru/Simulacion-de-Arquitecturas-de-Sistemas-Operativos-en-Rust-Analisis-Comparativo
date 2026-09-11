use std::collections::VecDeque;
use std::fmt;
use colored::Colorize;

#[derive(Clone)]
enum Operacion {
    Leer(String),
    Escribir(String, String),
    Eliminar(String),
    Respuesta(String),
}

impl fmt::Display for Operacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operacion::Leer(arch) => write!(f, "LEER {}", arch.cyan()),
            Operacion::Escribir(arch, datos) => write!(f, "ESCRIBIR '{}' en {}", datos.cyan(), arch.cyan()),
            Operacion::Eliminar(arch) => write!(f, "ELIMINAR {}", arch.cyan()),
            Operacion::Respuesta(msg) => write!(f, "{}", msg.cyan()),
        }
    }
}

#[derive(Clone)]
struct Mensaje {
    origen: String,
    destino: String,
    contenido: Operacion,
}

impl fmt::Display for Mensaje {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} -> {}]: {}",
            self.origen.green(), self.destino.yellow(), self.contenido
        )
    }
}

struct ServidorDisco {
    nombre: String,
}

impl ServidorDisco {
    fn nuevo(nombre: &str) -> Self {
        ServidorDisco {
            nombre: nombre.to_string(),
        }
    }

    fn atender(&self, msg: &Mensaje) -> Mensaje {
        println!(
            "    [{}] procesando petición de {}",
            self.nombre.yellow(), msg.contenido
        );

        let resultado = match &msg.contenido {
            Operacion::Leer(arch) => format!("Contenido de '{}'", arch),
            Operacion::Escribir(arch, datos) => {
                format!("Datos '{}' guardados en '{}'", datos, arch)
            }
            Operacion::Eliminar(arch) => format!("Archivo '{}' eliminado", arch),
            Operacion::Respuesta(_) => String::from("Operación no válida para servidor"),
        };

        Mensaje {
            origen: self.nombre.clone(),
            destino: msg.origen.clone(),
            contenido: Operacion::Respuesta(format!("OK: {}", resultado)),
        }
    }
}

struct Microkernel {
    cola_entrada: VecDeque<Mensaje>,
    cola_salida: VecDeque<Mensaje>,
}

impl Microkernel {
    fn nuevo() -> Self {
        Microkernel {
            cola_entrada: VecDeque::new(),
            cola_salida: VecDeque::new(),
        }
    }

    fn enviar(&mut self, msg: Mensaje) {
        println!("  [{}] enrutando mensaje: {}", "Kernel".magenta().bold(), msg);
        self.cola_entrada.push_back(msg);
    }

    fn despachar(&mut self, servidor: &ServidorDisco) {
        println!("\n  [{}] despachando mensajes al servidor...", "Kernel".magenta().bold());
        while let Some(msg) = self.cola_entrada.pop_front() {
            let respuesta = servidor.atender(&msg);
            self.cola_salida.push_back(respuesta);
        }
    }

    fn entregar_respuestas(&mut self) {
        println!("\n  [{}] entregando respuestas a procesos...", "Kernel".magenta().bold());
        while let Some(resp) = self.cola_salida.pop_front() {
            println!("    -> {}", resp);
        }
    }
}

struct ProcesoUsuario {
    nombre: String,
}

impl ProcesoUsuario {
    fn nuevo(nombre: &str) -> Self {
        ProcesoUsuario {
            nombre: nombre.to_string(),
        }
    }

    fn solicitar(&self, operacion: Operacion, servidor_destino: &str) -> Mensaje {
        println!("  [{}] solicita: {}", self.nombre.green(), operacion);
        Mensaje {
            origen: self.nombre.clone(),
            destino: servidor_destino.to_string(),
            contenido: operacion,
        }
    }
}

fn main() {
    println!("{}", "╔══════════════════════════════════════════════╗".blue().bold());
    println!("{}", "║   Simulación: Arquitectura Microkernel       ║".blue().bold());
    println!("{}", "╚══════════════════════════════════════════════╝\n".blue().bold());

    let mut kernel = Microkernel::nuevo();
    let servidor = ServidorDisco::nuevo("ServidorDisco");
    let proc_a = ProcesoUsuario::nuevo("ProcesoA");
    let proc_b = ProcesoUsuario::nuevo("ProcesoB");
    let proc_c = ProcesoUsuario::nuevo("ProcesoC");

    println!("{}", "── Fase 1: Procesos envían solicitudes ──".yellow().bold());
    kernel.enviar(proc_a.solicitar(Operacion::Leer("archivo.txt".to_string()), "ServidorDisco"));
    kernel.enviar(proc_b.solicitar(
        Operacion::Escribir("log.txt".to_string(), "DATOS".to_string()),
        "ServidorDisco",
    ));
    kernel.enviar(proc_c.solicitar(Operacion::Eliminar("temp.dat".to_string()), "ServidorDisco"));

    println!("\n{}", "── Fase 2: Kernel despacha al servidor ──".yellow().bold());
    kernel.despachar(&servidor);

    println!("\n{}", "── Fase 3: Kernel entrega respuestas ──".yellow().bold());
    kernel.entregar_respuestas();

    println!("\n{}", "══ Simulación finalizada ══".green().bold());
}