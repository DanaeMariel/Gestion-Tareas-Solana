# Gestion-Tareas-Solana

> Smart Contract en Solana para la gestion personal de tareas (CRUD) directamente en la blockchain.

![Banner del proyecto](./images/banner-tareas.jpg)

## Que es Gestion-Tareas-Solana

Gestion-Tareas-Solana es un programa desplegado en la blockchain de Solana que permite a cada usuario crear y gestionar su propia lista de tareas.

Cada lista contiene tareas con su descripcion y estado (completada/pendiente), almacenadas dentro de una cuenta PDA unica por wallet.

Desarrollado con Rust y Anchor como parte del aprendizaje practico en Solana.

## Instrucciones del programa

| Instruccion | Accion | Descripcion |
| :--- | :--- | :--- |
| `crear_lista` | CREATE | Inicializa una nueva lista de tareas para el usuario |
| `crear_tarea` | CREATE | Agrega una nueva tarea a la lista |
| `leer_tareas` | READ | Lee los datos de la lista y todas sus tareas desde el cliente |
| `actualizar_tarea` | UPDATE | Cambia la descripcion y/o el estado de una tarea |
| `eliminar_tarea` | DELETE | Elimina una tarea de la lista |

## Estructura de datos

La lista de tareas se almacena en una cuenta PDA unica para cada usuario:

```rust
pub struct Lista {
  pub owner: Pubkey,
  pub nombre: String,
  pub tareas: Vec<Tarea>,
}

pub struct Tarea {
  pub descripcion: String,
  pub completada: bool,
}
```

La direccion de cada cuenta `Lista` se deriva con estas semillas:

```text
["lista", owner_wallet]
```

Esto garantiza que solo el propietario de la wallet pueda acceder y modificar su lista.

## Como ejecutarlo en Solana Playground

1. Abrir el repositorio en GitHub.

![Repositorio en GitHub](./images/repo.png)

2. Copiar la URL del repo y abrirla en Solana Playground con este formato:

```text
https://beta.solpg.io/https://github.com/DanaeMariel/Gestion-Tareas-Solana
```

![URL en Solana Playground](./images/url.png)

3. Si es un fork, sincroniza antes de importar (opcional).

![Sync fork](./images/fork.png)

4. En Solana Playground, haz clic en `Import` y asigna el nombre del proyecto.

![Ventana de importacion](./images/import.png)

5. Conecta la wallet en Devnet.

![Estado no conectado](./images/playground1.png)

![Crear o importar wallet](./images/wallet.png)

![Wallet conectada en Devnet](./images/status.png)

6. Build y Deploy del programa.

![Proyecto cargado en SolPG](./images/pg.png)

7. Ejecuta el cliente en la terminal de SolPG:

```bash
run
```

## Estructura del proyecto

```text
Gestion-Tareas-Solana/
|-- client/
|   `-- client.ts
|-- images/
|-- src/
|   `-- lib.rs
|-- tests/
|   `-- anchor.test.ts
`-- README.md
```

## Tecnologias

| Herramienta | Uso |
| :--- | :--- |
| Rust | Logica del smart contract |
| Anchor | Framework para desarrollo en Solana |
| TypeScript | Cliente de pruebas e integracion |
| Solana Devnet | Red de pruebas para despliegue |
| Solana Playground | IDE en navegador |

## Autor

Desarrollado por Danae Mariel.