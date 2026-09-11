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
            println!("    - {}", nodo.proceso);
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

fn main() {
    println!("{}","╔══════════════════════════════════════════╗".red().bold());
    println!("{} {} {}", "║".red().bold(), "Simulación: Arquitectura de Contenedores".bright_blue().on_bright_white().bold(), "║".red().bold());
    println!("{}","╚══════════════════════════════════════════╝\n".red().bold());
    println!("{}","── Fase : Inicialización de Contenedores──\n".yellow().bold());

    let mut lista = ListaProcesos::nueva();
    let cgroup = Cgroup::nuevo(512);
    let mut ns = Namespace::nuevo();

    let pruebas = [
        (100, "nginx", 128),
        (101, "redis", 256),
        (102, "node-app", 64),
    ];

    println!(
        "{}",
        format!("[+] Agregando procesos al contenedor (limite: {} MB)...", cgroup.limite_memoria_mb).magenta().bold()
    );

    for (pid_real, nombre, mem) in &pruebas {
        let mem_actual = lista.memoria_total();

        if cgroup.permitir(mem_actual, *mem) {
            let pid_v = ns.asignar_pid();
            let proc = Proceso {
                pid_real: *pid_real,
                pid_virtual: pid_v,
                nombre: nombre.to_string(),
                memoria_mb: *mem,
            };
            println!(
                "    {}: {} asignado (PID virtual: {})",
                "OK".green().bold(),
                nombre.green(),
                pid_v.to_string().yellow()
            );
            lista.agregar(proc);
        } else {
            println!(
                "    {}: {} excede el limite ({} + {} > {})",
                "RECHAZADO".red().bold(),
                nombre.green(),
                mem_actual.to_string().cyan(),
                mem.to_string().cyan(),
                cgroup.limite_memoria_mb.to_string().cyan()
            );
        }
    }

    println!("\n{}", "[*] Procesos activos en el contenedor:".magenta().bold());
    lista.mostrar();
    println!(
        "\n    Memoria total usada: {} / {} MB",
        lista.memoria_total().to_string().cyan(),
        cgroup.limite_memoria_mb.to_string().cyan()
    );

    println!("\n{}", "── Fin de la simulación ──".green().bold());
}