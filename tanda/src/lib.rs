#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Members,
    Amount,
    Round,
    Paid(u32, Address), // (ronda, miembro)
}

#[contract]
pub struct Tanda;

#[contractimpl]
impl Tanda {
    /// Crea la tanda con sus miembros y la cuota por ronda.
    pub fn init(env: Env, admin: Address, members: Vec<Address>, amount: i128) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("ya inicializado");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Members, &members);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::Round, &0u32);
    }

    /// Un miembro registra su pago de la ronda actual.
    pub fn pay(env: Env, member: Address) {
        member.require_auth();
        let members: Vec<Address> = env.storage().instance().get(&DataKey::Members).unwrap();
        if !members.contains(&member) {
            panic!("no es miembro");
        }
        let round: u32 = env.storage().instance().get(&DataKey::Round).unwrap();
        let key = DataKey::Paid(round, member);
        if env.storage().instance().has(&key) {
            panic!("ya pagó esta ronda");
        }
        env.storage().instance().set(&key, &true);
    }

    /// A quién le toca recibir en la ronda actual.
    pub fn recipient(env: Env) -> Address {
        let members: Vec<Address> = env.storage().instance().get(&DataKey::Members).unwrap();
        let round: u32 = env.storage().instance().get(&DataKey::Round).unwrap();
        members.get(round % members.len()).unwrap()
    }

    /// ¿Ya pagaron todos en esta ronda?
    pub fn is_round_complete(env: Env) -> bool {
        let members: Vec<Address> = env.storage().instance().get(&DataKey::Members).unwrap();
        let round: u32 = env.storage().instance().get(&DataKey::Round).unwrap();
        for m in members.iter() {
            if !env.storage().instance().has(&DataKey::Paid(round, m)) {
                return false;
            }
        }
        true
    }

    /// El admin pasa a la siguiente ronda (solo si todos pagaron).
    pub fn next_round(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        if !Self::is_round_complete(env.clone()) {
            panic!("faltan pagos");
        }
        let round: u32 = env.storage().instance().get(&DataKey::Round).unwrap();
        env.storage().instance().set(&DataKey::Round, &(round + 1));
    }
}

mod test;