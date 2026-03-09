use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod gestion_tareas {
    use super::*;

    // Funcion para crear una nueva lista de tareas
    // Parametros:
    // - nombre: Nombre de la lista de tareas
    pub fn crear_lista(ctx: Context<NuevaLista>, nombre: String) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();

        // Inicializa un vector vacio para las tareas
        let tareas: Vec<Tarea> = Vec::new();

        // Establece los datos de la cuenta lista
        ctx.accounts.list.set_inner(Lista {
            owner: owner_id,
            nombre,
            tareas,
        });

        msg!("Lista '{}' creada!", ctx.accounts.list.nombre);
        Ok(())
    }

    // Funcion para crear una nueva tarea
    // Parametros:
    // - descripcion: Descripcion de la tarea
    pub fn crear_tarea(ctx: Context<OperacionLista>, descripcion: String) -> Result<()> {
        // Verifica que el firmante sea el propietario de la lista
        require!(
            ctx.accounts.list.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Crea una nueva instancia de Tarea
        let tarea = Tarea {
            descripcion,
            completada: false,
        };

        // Agrega la tarea al vector de tareas de la lista
        ctx.accounts.list.tareas.push(tarea);

        msg!("Tarea creada!");
        Ok(())
    }

    // Funcion para leer todas las tareas
    // Parametros:
    pub fn leer_tareas(ctx: Context<OperacionLista>) -> Result<()> {
        // Verifica que el firmante sea el propietario de la lista
        require!(
            ctx.accounts.list.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Muestra en log la lista completa de tareas
        msg!("Tareas: {:#?}", ctx.accounts.list.tareas);
        Ok(())
    }

    // Funcion para actualizar una tarea existente
    // Parametros:
    // - indice: Posicion de la tarea en el vector (comienza en 0)
    // - nueva_descripcion: Nueva descripcion para la tarea
    // - esta_completada: Estado de completado de la tarea
    pub fn actualizar_tarea(
        ctx: Context<OperacionLista>,
        indice: u32,
        nueva_descripcion: String,
        esta_completada: bool,
    ) -> Result<()> {
        // Verifica que el firmante sea el propietario de la lista
        require!(
            ctx.accounts.list.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Convierte el indice de u32 a usize para usar con el vector
        let indice_usize = indice as usize;

        // Verifica que el indice exista en el vector
        if indice_usize < ctx.accounts.list.tareas.len() {
            // Obtiene una referencia mutable a la tarea
            let tarea = &mut ctx.accounts.list.tareas[indice_usize];
            // Actualiza los campos de la tarea
            tarea.descripcion = nueva_descripcion;
            tarea.completada = esta_completada;
            msg!("Tarea actualizada!");
            Ok(())
        } else {
            // Si no encuentra la tarea, retorna error
            Err(ErrorPersonalizado::TareaNoExiste.into())
        }
    }

    // Funcion para eliminar una tarea
    // Parametros:
    // - indice: Posicion de la tarea en el vector (comienza en 0)
    pub fn eliminar_tarea(ctx: Context<OperacionLista>, indice: u32) -> Result<()> {
        // Verifica que el firmante sea el propietario de la lista
        require!(
            ctx.accounts.list.owner == ctx.accounts.owner.key(),
            ErrorPersonalizado::NoAutorizado
        );

        // Convierte el indice de u32 a usize para usar con el vector
        let indice_usize = indice as usize;

        // Verifica que el indice exista en el vector
        if indice_usize < ctx.accounts.list.tareas.len() {
            // Elimina la tarea del vector
            ctx.accounts.list.tareas.remove(indice_usize);
            msg!("Tarea eliminada!");
            Ok(())
        } else {
            // Si no encuentra la tarea, retorna error
            Err(ErrorPersonalizado::TareaNoExiste.into())
        }
    }
}

// Definicion de errores personalizados para el programa
#[error_code]
pub enum ErrorPersonalizado {
    #[msg("Error: No eres el propietario de la lista")]
    NoAutorizado,
    #[msg("Error: La tarea no existe")]
    TareaNoExiste,
}

// Estructura de cuenta para la Lista
#[account]
#[derive(InitSpace)]
pub struct Lista {
    pub owner: Pubkey, // Clave publica del propietario

    #[max_len(30)]
    pub nombre: String, // Nombre de la lista (max 30 caracteres)

    #[max_len(50)]
    pub tareas: Vec<Tarea>, // Vector de tareas (max 50 tareas)
}

// Estructura para representar una Tarea
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Tarea {
    #[max_len(100)]
    pub descripcion: String, // Descripcion de la tarea (max 100 caracteres)

    pub completada: bool, // Estado de la tarea (true = completada, false = pendiente)
}

// Contexto para crear una nueva lista
#[derive(Accounts)]
pub struct NuevaLista<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Lista::INIT_SPACE,
        seeds = [b"lista", owner.key().as_ref()],
        bump
    )]
    pub list: Account<'info, Lista>,

    pub system_program: Program<'info, System>,
}

// Contexto para operaciones con tareas (crear, leer, actualizar, eliminar)
#[derive(Accounts)]
pub struct OperacionLista<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lista", owner.key().as_ref()],
        bump
    )]
    pub list: Account<'info, Lista>,
}