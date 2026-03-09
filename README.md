# Gestion-Tareas-Solana

> Smart Contract en Solana para la gestión personal de tareas (CRUD) directamente en la blockchain.

* * *

## ¿Qué es Gestion-Tareas-Solana?

Gestion-Tareas-Solana es un programa desplegado en la **blockchain de Solana** que permite a cualquier usuario crear y gestionar su propia lista de tareas. Cada lista contiene tareas con su **descripción** y **estado** (completada/pendiente), almacenadas como un vector dinámico dentro de una cuenta PDA única.

Desarrollado con **Rust** y el framework **Anchor** como parte del aprendizaje práctico de desarrollo en Solana.

* * *

## Instrucciones del Programa

| Instrucción | Acción | Descripción |
| :--- | :--- | :--- |
| `crear_lista` | **CREATE** | Inicializa una nueva lista de tareas para el usuario |
| `crear_tarea` | **CREATE** | Agrega una nueva tarea a la lista |
| `leer_tareas` | **READ** | Lee los datos de la lista y todas sus tareas desde el client |
| `actualizar_tarea` | **UPDATE** | Cambia la descripción y/o el estado de una tarea |
| `eliminar_tarea` | **DELETE** | Elimina una tarea de la lista |

* * *

## Estructura de Datos

La lista de tareas se almacena en una cuenta PDA única para cada usuario, con los siguientes campos:

```rust
pub struct Lista {
    pub owner: Pubkey,      // Wallet del propietario de la lista
    pub nombre: String,     // "Mis Tareas", "Trabajo", etc.
    pub tareas: Vec<Tarea>, // Vector dinámico con las tareas
}

pub struct Tarea {
    pub descripcion: String, // "Estudiar Solana", "Comprar leche"
    pub completada: bool,    // true = completada, false = pendiente
}

La dirección de cada cuenta Lista se deriva con las semillas:

["lista", owner_wallet]

Esto garantiza que solo el propietario de la wallet pueda acceder y modificar su lista de tareas.


Cómo Ejecutarlo
1. Importar en Solana Playground
Copia el enlace de tu repositorio y ábrelo en Solana Playground:

text
https://beta.solpg.io/github.com/DannanMariel/Gestion-Tareas-Solana
Haz clic en Import y asigna un nombre.

2. Conectar Wallet
Haz clic en Not Connected (parte inferior izquierda) para conectarte a la Devnet y crear tu wallet de prueba.

Pide SOL de prueba en la terminal:

bash
solana airdrop 2

3. Build & Deploy
Clic en Build — espera la marca verde de compilación exitosa.

Clic en Deploy — espera el mensaje "Deployment successful".

4. Ejecutar Pruebas
En la terminal de SolPG escribe:

bash
run
sto ejecuta client/client.ts que realiza el ciclo CRUD completo:

text
SISTEMA DE GESTION DE TAREAS - OPERACIONES CRUD

📍 PDA derivada para la lista: 5XJiKjAuMrCZvgv5qDkKDsNV

--- 1. CREANDO LISTA ---
 Transacción de creación exitosa. Hash: 4xKpJ7...

--- 2. CREANDO TAREAS ---
 Tarea "Estudiar Solana" creada.
 Tarea "Hacer ejercicio" creada.
 Tarea "Comprar leche" creada.

--- 3. LEYENDO TAREAS ---
 Datos extraídos de la PDA:
   - Propietario: CEdMTr2c52JvKVfTiH3p9ihLP1
   - Nombre de lista: MIS TAREAS
   - Total tareas: 3
   - Tarea 0: "Estudiar Solana" [✗]
   - Tarea 1: "Hacer ejercicio" [✗]
   - Tarea 2: "Comprar leche" [✗]

--- 4. ACTUALIZANDO TAREA ---
 Transacción de actualización exitosa.
 Tarea 0 actualizada: "Estudiar Solana Avanzado" [✓]

--- 5. ELIMINANDO TAREA ---
  Transacción de eliminación exitosa.
  Tarea 2 eliminada.
¡Prueba del CRUD de Gestion-Tareas-Solana completada con éxito!

Estructura del Proyecto
Gestion-Tareas-Solana/
├── programs/
│   └── gestion-tareas/
│       └── src/
│           └── lib.rs          # Smart Contract (Rust + Anchor)
├── client/
│   └── client.ts                # Script de pruebas CRUD (TypeScript)
├── tests/
│   └── gestion-tareas.ts        # Tests unitarios
├── images/                       # Imágenes del README
└── README.md

Tecnologías

Herramienta	Uso
Rust	Lógica del Smart Contract
Anchor	Framework para desarrollo en Solana
TypeScript	Cliente de pruebas e integración
Solana Devnet	Red de pruebas para despliegue
Solana Playground	IDE en el navegador

Autor
Desarrollado por DannanMariel