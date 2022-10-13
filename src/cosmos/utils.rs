use cosmrs::tx::Gas;

pub fn mul_gas_float(gas: impl Into<Gas>, f: f64) -> Gas {
    #[allow(clippy::cast_precision_loss)]
    let gas = gas.into().value() as f64;

    let gas = gas * f;

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let gas = gas as u64;
    Gas::from(gas)
}