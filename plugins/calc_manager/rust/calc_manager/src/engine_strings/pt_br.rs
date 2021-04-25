use super::EngineStringsId;
use lazy_static::lazy_static;
use std::collections::hash_map::HashMap;

lazy_static! {
    static ref EN_US: HashMap<EngineStringsId, &'static str> = [
        (EngineStringsId::Domain, "Entrada inválida"),
        (EngineStringsId::Undefined, "Resultado indefinido"),
        (EngineStringsId::Nomem, "Memória insuficiente"),
        (EngineStringsId::Overflow, "Estouro"),
        (EngineStringsId::Noresult, "Resultado não definido"),
        (EngineStringsId::ErrSgInvError, "Resultado não definido"),
        (EngineStringsId::ErrInputOverflow, "Estouro"),
        (EngineStringsId::ErrOutputOverflow, "Estouro"),
        (EngineStringsId::Dividebyzero, "Não é possível dividir por zero"),
    ]
    .iter()
    .cloned()
    .collect();
}
