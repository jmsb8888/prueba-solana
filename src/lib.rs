use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use rand::Rng;

/// Define la estructura de datos del estado almacenado en cuentas
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// Número aleatorio generado
    pub random_number: u32,
}

// Declara y exporta el punto de entrada (entrypoint) del programa
pub entrypoint!(process_instruction);

// Implementa la lógica principal del programa en la función process_instruction
pub fn process_instruction(
    _program_id: &Pubkey, // Clave pública de la cuenta donde se cargó el programa
    accounts: &[AccountInfo], // Cuentas involucradas en la transacción
    _instruction_data: &[u8], // Datos de instrucción (ignorados en este caso)
) -> ProgramResult {
    msg!("Punto de entrada del programa Rust de Generación de Número Aleatorio");

    // Obtener la cuenta a la que se va a enviar el número aleatorio
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Generar un número aleatorio
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100); // Número aleatorio entre 1 y 100

    // Crear una instancia de la estructura GreetingAccount con el número aleatorio
    let greeting_account = GreetingAccount { random_number };

    // Serializar la estructura y almacenarla en los datos de la cuenta
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Número aleatorio generado: {}", random_number);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .random_number,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let random_number = GreetingAccount::try_from_slice(&accounts[0].data.borrow())
            .unwrap()
            .random_number;
        assert!(random_number >= 1 && random_number <= 100);
    }
}