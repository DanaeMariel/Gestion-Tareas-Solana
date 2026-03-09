// Script para interactuar con el sistema de gestion de tareas

function getListaPDA() {
  const [listaPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("lista"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  );
  return listaPda;
}

const listaPda = getListaPDA();
console.log("Direccion PDA de la lista:", listaPda.toBase58());

// Funcion 1: Crear Lista
async function crearLista(nombre: string) {
  try {
    console.log(\nCreando lista: "${nombre}"...);

    const tx = await pg.program.methods
      .crearLista(nombre)
      .accounts({
        owner: pg.wallet.publicKey,
        list: listaPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Lista creada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.log("La lista ya existe");
  }
}

// Funcion 2: Crear Tarea
async function crearTarea(descripcion: string) {
  try {
    console.log(\nCreando tarea: "${descripcion}"...);

    const tx = await pg.program.methods
      .crearTarea(descripcion)
      .accounts({
        owner: pg.wallet.publicKey,
        list: listaPda,
      })
      .rpc();

    console.log("Tarea creada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.error("Error al crear tarea:", e.message);
  }
}

// Funcion 3: Ver todas las tareas
async function verTareas() {
  try {
    console.log("\nObteniendo lista de tareas...");

    const listaData = await pg.program.account.lista.fetch(listaPda);

    console.log("=== INFORMACION DE LA LISTA ===");
    console.log("Nombre:", listaData.nombre);
    console.log("Dueño:", listaData.owner.toBase58());
    console.log("Total tareas:", listaData.tareas.length);

    if (listaData.tareas.length === 0) {
      console.log("No hay tareas");
    } else {
      console.log("=== LISTA DE TAREAS ===");
      listaData.tareas.forEach((tarea, index) => {
        console.log(\nTarea #${index}:);
        console.log(`  Descripcion: ${tarea.descripcion}`);
        console.log(`  Completada: ${tarea.completada ? "✓" : "✗"}`);
      });
    }

    return listaData.tareas;
  } catch (e) {
    console.error("Error al ver tareas:", e.message);
  }
}

// Funcion 4: Actualizar tarea
async function actualizarTarea(
  indice: number,
  nuevaDescripcion: string,
  estaCompletada: boolean
) {
  try {
    console.log(\nActualizando tarea ${indice}...);

    const tx = await pg.program.methods
      .actualizarTarea(indice, nuevaDescripcion, estaCompletada)
      .accounts({
        owner: pg.wallet.publicKey,
        list: listaPda,
      })
      .rpc();

    console.log("Tarea actualizada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.error("Error al actualizar tarea:", e.message);
  }
}

// Funcion 5: Eliminar tarea
async function eliminarTarea(indice: number) {
  try {
    console.log(\nEliminando tarea ${indice}...);

    const tx = await pg.program.methods
      .eliminarTarea(indice)
      .accounts({
        owner: pg.wallet.publicKey,
        list: listaPda,
      })
      .rpc();

    console.log("Tarea eliminada exitosamente");
    console.log("Transaction:", tx);
  } catch (e) {
    console.error("Error al eliminar tarea:", e.message);
  }
}

// Funcion 6: Marcar tarea como completada (atajo)
async function completarTarea(indice: number) {
  try {
    const listaData = await pg.program.account.lista.fetch(listaPda);
    const tarea = listaData.tareas[indice];

    if (!tarea) {
      console.log(\nTarea ${indice} no existe);
      return;
    }

    await actualizarTarea(indice, tarea.descripcion, true);
    console.log(Tarea ${indice} marcada como completada);
  } catch (e) {
    console.error("Error:", e.message);
  }
}

// Funcion 7: Limpiar todas las tareas
async function limpiarTodasLasTareas() {
  try {
    const listaData = await pg.program.account.lista.fetch(listaPda);
    const tareas = [...listaData.tareas];

    for (let i = tareas.length - 1; i >= 0; i--) {
      console.log(\nEliminando tarea ${i}...);
      await pg.program.methods
        .eliminarTarea(i)
        .accounts({
          owner: pg.wallet.publicKey,
          list: listaPda,
        })
        .rpc();
    }

    console.log("\nTodas las tareas eliminadas");
  } catch (e) {
    console.error("Error al limpiar tareas:", e.message);
  }
}

// EJEMPLO DE USO
(async () => {
  console.log("SISTEMA DE GESTION DE TAREAS - OPERACIONES CRUD");

  // Crear lista (si no existe)
  await crearLista("TAREAS DIARIAS");

  // LIMPIAR TAREAS
  // await limpiarTodasLasTareas();

  // Crear tareas
  await crearTarea("Estudiar Solana");
  await crearTarea("Hacer ejercicio");
  await crearTarea("Comprar leche");

  // Ver tareas
  await verTareas();

  // Actualizar tarea 0
  await actualizarTarea(0, "Estudiar Solana avanzado", true);

  // Ver tareas despues de actualizar
  await verTareas();

  // Marcar tarea 1 como completada
  await completarTarea(1);

  // Ver tareas
  await verTareas();

  // Eliminar tarea 2
  await eliminarTarea(2);

  // Ver tareas finales
  await verTareas();
})();