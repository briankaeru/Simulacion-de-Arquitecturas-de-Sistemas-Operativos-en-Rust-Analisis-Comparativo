use std::fmt;
use colored::Colorize;

struct Proceso {
    pid_real: u32,
    pid_virtual: u32,
    nombre: String,
    memoria_mb: u32,
}

impl fmt::Display for Proceso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  (PID real: {}, PID virtual: {}, mem: {} MB)",
            self.nombre.green(),
            self.pid_real.to_string().cyan(),
            self.pid_virtual.to_string().yellow(),
            self.memoria_mb.to_string().cyan()
        )
    }
}

struct NodoProceso {
    proceso: Proceso,
    siguiente: Option<Box<NodoProceso>>,
}

struct ListaProcesos {
    cabeza: Option<Box<NodoProceso>>,
    cantidad: u32,
}

impl ListaProcesos {
    fn nueva() -> Self {
        ListaProcesos {
            cabeza: None,
            cantidad: 0,
        }
    }

    fn agregar(&mut self, proceso: Proceso) {
        let nuevo = Box::new(NodoProceso {
            proceso,
            siguiente: None,
        });

        match self.cabeza {
            None => {
                self.cabeza = Some(nuevo);
            }
            Some(ref mut cabeza) => {
                let mut actual = cabeza;
                while actual.siguiente.is_some() {
                    actual = actual.siguiente.as_mut().unwrap();
                }
                actual.siguiente = Some(nuevo);
            }
        }
        self.cantidad += 1;
    }

    fn mostrar(&self) {
        let mut actual = &self.cabeza;
        while let Some(nodo) = actual {
            println!("      - {}", nodo.proceso);
            actual = &nodo.siguiente;
        }
    }

    fn memoria_total(&self) -> u32 {
        let mut total = 0;
        let mut actual = &self.cabeza;
        while let Some(nodo) = actual {
            total += nodo.proceso.memoria_mb;
            actual = &nodo.siguiente;
        }
        total
    }
}

struct Cgroup {
    limite_memoria_mb: u32,
}

impl Cgroup {
    fn nuevo(limite_mb: u32) -> Self {
        Cgroup {
            limite_memoria_mb: limite_mb,
        }
    }

    fn permitir(&self, memoria_actual: u32, memoria_nueva: u32) -> bool {
        memoria_actual + memoria_nueva <= self.limite_memoria_mb
    }
}

struct Namespace {
    siguiente_pid: u32,
}

impl Namespace {
    fn nuevo() -> Self {
        Namespace { siguiente_pid: 1 }
    }

    fn asignar_pid(&mut self) -> u32 {
        let pid = self.siguiente_pid;
        self.siguiente_pid += 1;
        pid
    }
}

// un contenedor agrupa su propio namespace, cgroup y lista de procesos
struct Contenedor {
    nombre: String,
    namespace: Namespace,
    cgroup: Cgroup,
    procesos: ListaProcesos,
}

impl Contenedor {
    fn nuevo(nombre: &str, limite_memoria_mb: u32) -> Self {
        Contenedor {
            nombre: nombre.to_string(),
            namespace: Namespace::nuevo(),
            cgroup: Cgroup::nuevo(limite_memoria_mb),
            procesos: ListaProcesos::nueva(),
        }
    }

    // intenta ejecutar un proceso dentro del contenedor
    fn ejecutar_proceso(&mut self, pid_real: u32, nombre: &str, memoria_mb: u32) {
        let mem_actual = self.procesos.memoria_total();

        if self.cgroup.permitir(mem_actual, memoria_mb) {
            let pid_v = self.namespace.asignar_pid();
            let proc = Proceso {
                pid_real,
                pid_virtual: pid_v,
                nombre: nombre.to_string(),
                memoria_mb,
            };
            println!(
                "    {}: {} -> PID virtual {}, mem {} MB",
                "OK".green().bold(),
                nombre.green(),
                pid_v.to_string().yellow(),
                memoria_mb.to_string().cyan()
            );
            self.procesos.agregar(proc);
        } else {
            println!(
                "    {}: {} necesita {} MB pero solo quedan {} MB libres",
                "RECHAZADO".red().bold(),
                nombre.green(),
                memoria_mb.to_string().cyan(),
                (self.cgroup.limite_memoria_mb - mem_actual).to_string().cyan()
            );
        }
    }

    // muestra el estado del contenedor
    fn mostrar_estado(&self) {
        println!(
            "\n  {} [{}]",
            "📦".to_string(),
            self.nombre.bright_blue().bold()
        );
        println!(
            "    Limite de memoria: {} MB",
            self.cgroup.limite_memoria_mb.to_string().cyan()
        );
        println!(
            "    Memoria usada:    {} MB",
            self.procesos.memoria_total().to_string().cyan()
        );
        println!(
            "    Procesos activos: {}",
            self.procesos.cantidad.to_string().yellow()
        );
        if self.procesos.cantidad > 0 {
            self.procesos.mostrar();
        }
    }
}

fn main() {
    println!("{}", "╔══════════════════════════════════════════════╗".red().bold());
    println!(
        "{} {} {}",
        "║".red().bold(),
        "Simulación: Arquitectura de Contenedores".bright_blue().bold(),
        "   ║".red().bold()
    );
    println!("{}", "╚══════════════════════════════════════════════╝\n".red().bold());

    // cada contenedor tiene su propio namespace y cgroup aislado
    let mut web_container = Contenedor::nuevo("web-server", 512);
    let mut db_container = Contenedor::nuevo("database", 256);

    // --- lanzar procesos en el contenedor web ---
    println!(
        "{}",
        "── Fase 1: Lanzando procesos en contenedor 'web-server' (512 MB) ──".yellow().bold()
    );
    web_container.ejecutar_proceso(1001, "nginx", 128);
    web_container.ejecutar_proceso(1002, "node-app", 256);
    web_container.ejecutar_proceso(1003, "logger", 64);
    // este deberia ser rechazado por exceder la cuota
    web_container.ejecutar_proceso(1004, "monitoring", 128);

    // --- lanzar procesos en el contenedor db ---
    println!(
        "\n{}",
        "── Fase 2: Lanzando procesos en contenedor 'database' (256 MB) ──".yellow().bold()
    );
    db_container.ejecutar_proceso(2001, "postgres", 128);
    db_container.ejecutar_proceso(2002, "redis", 64);
    // este deberia ser rechazado
    db_container.ejecutar_proceso(2003, "mysql", 128);

    // --- mostrar aislamiento ---
    println!(
        "\n{}",
        "── Fase 3: Estado de los contenedores (aislamiento) ──".yellow().bold()
    );
    web_container.mostrar_estado();
    db_container.mostrar_estado();

    // --- demostrar que los PIDs virtuales son independientes ---
    println!(
        "\n{}",
        "── Fase 4: Demostración de aislamiento de PIDs ──".yellow().bold()
    );
    println!(
        "  Contenedor '{}': PIDs virtuales van de {} a {}",
        "web-server".bright_blue(),
        "1".yellow(),
        web_container.procesos.cantidad.to_string().yellow()
    );
    println!(
        "  Contenedor '{}': PIDs virtuales van de {} a {}",
        "database".bright_blue(),
        "1".yellow(),
        db_container.procesos.cantidad.to_string().yellow()
    );
    println!(
        "  -> Ambos contenedores tienen PID {} pero son procesos distintos y aislados",
        "1".yellow().bold()
    );

    println!("\n{}", "══ Simulación finalizada ══".green().bold());
}