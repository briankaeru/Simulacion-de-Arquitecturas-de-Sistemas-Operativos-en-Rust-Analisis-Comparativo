use std::fmt;

// proceso que corre dentro de un contenedor
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
            self.nombre, self.pid_real, self.pid_virtual, self.memoria_mb
        )
    }
}

// nodo de la lista enlazada de procesos
struct NodoProceso {
    proceso: Proceso,
    siguiente: Option<Box<NodoProceso>>,
}

// lista enlazada para rastrear los procesos de un contenedor
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

    // agrega un proceso al final de la lista
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

    // muestra todos los procesos de la lista
    fn mostrar(&self) {
        let mut actual = &self.cabeza;
        while let Some(nodo) = actual {
            println!("    - {}", nodo.proceso);
            actual = &nodo.siguiente;
        }
    }

    // calcula la memoria total usada por todos los procesos
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

// simula los cgroups de linux: limita recursos del contenedor
struct Cgroup {
    limite_memoria_mb: u32,
}

impl Cgroup {
    fn nuevo(limite_mb: u32) -> Self {
        Cgroup {
            limite_memoria_mb: limite_mb,
        }
    }

    // verifica si agregar un proceso excederia el limite de memoria
    fn permitir(&self, memoria_actual: u32, memoria_nueva: u32) -> bool {
        memoria_actual + memoria_nueva <= self.limite_memoria_mb
    }
}

// simula los namespaces de linux: asigna pids virtuales aislados
struct Namespace {
    siguiente_pid: u32,
}

impl Namespace {
    fn nuevo() -> Self {
        Namespace { siguiente_pid: 1 }
    }

    // asigna el proximo pid virtual disponible
    fn asignar_pid(&mut self) -> u32 {
        let pid = self.siguiente_pid;
        self.siguiente_pid += 1;
        pid
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Simulación: Arquitectura de Contenedores  ║");
    println!("╚══════════════════════════════════════════════╝\n");
    println!("── Fase 4: Prueba de estructuras de control ──\n");

    let mut lista = ListaProcesos::nueva();
    let cgroup = Cgroup::nuevo(512);
    let mut ns = Namespace::nuevo();

    // procesos de prueba con distinto consumo de memoria
    let pruebas = [
        (100, "nginx", 128),
        (101, "redis", 256),
        (102, "node-app", 64),
    ];

    println!(
        "[+] Agregando procesos al contenedor (limite: {} MB)...",
        cgroup.limite_memoria_mb
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
            println!("    OK: {} asignado (PID virtual: {})", nombre, pid_v);
            lista.agregar(proc);
        } else {
            println!(
                "    RECHAZADO: {} excede el limite ({} + {} > {})",
                nombre, mem_actual, mem, cgroup.limite_memoria_mb
            );
        }
    }

    println!("\n[*] Procesos activos en el contenedor:");
    lista.mostrar();
    println!(
        "\n    Memoria total usada: {} / {} MB",
        lista.memoria_total(),
        cgroup.limite_memoria_mb
    );

    println!("\n── Fin de prueba Fase 4 ──");
}
