macro_rules! CMPTLZ_LITERAL { () => { CMPTLZ_POSSLOT!() + (CMPTLZ_LEN_CONDITION_TO_POSSLOT!() << CMPTLZ_POS_SLOT_BITS!()) } }
pub(crate) use CMPTLZ_LITERAL;
