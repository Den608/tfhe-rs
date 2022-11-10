pub use crate::core_crypto::prelude::{
    DecompositionBaseLog, DecompositionLevelCount, DispersionParameter, GlweDimension,
    LweDimension, PolynomialSize, StandardDev,
};
use crate::shortint::parameters::{CarryModulus, MessageModulus};
use crate::shortint::Parameters;

pub const ALL_PARAMETER_VEC_WOPBS_NORM2: [Parameters; 31] = [
    WOPBS_PARAM_MESSAGE_1_NORM2_2,
    WOPBS_PARAM_MESSAGE_1_NORM2_4,
    WOPBS_PARAM_MESSAGE_1_NORM2_6,
    WOPBS_PARAM_MESSAGE_1_NORM2_8,
    WOPBS_PARAM_MESSAGE_2_NORM2_2,
    WOPBS_PARAM_MESSAGE_2_NORM2_4,
    WOPBS_PARAM_MESSAGE_2_NORM2_6,
    WOPBS_PARAM_MESSAGE_2_NORM2_8,
    WOPBS_PARAM_MESSAGE_3_NORM2_2,
    WOPBS_PARAM_MESSAGE_3_NORM2_4,
    WOPBS_PARAM_MESSAGE_3_NORM2_6,
    WOPBS_PARAM_MESSAGE_3_NORM2_8,
    WOPBS_PARAM_MESSAGE_4_NORM2_2,
    WOPBS_PARAM_MESSAGE_4_NORM2_4,
    WOPBS_PARAM_MESSAGE_4_NORM2_6,
    WOPBS_PARAM_MESSAGE_4_NORM2_8,
    WOPBS_PARAM_MESSAGE_5_NORM2_2,
    WOPBS_PARAM_MESSAGE_5_NORM2_4,
    WOPBS_PARAM_MESSAGE_5_NORM2_6,
    WOPBS_PARAM_MESSAGE_5_NORM2_8,
    WOPBS_PARAM_MESSAGE_6_NORM2_2,
    WOPBS_PARAM_MESSAGE_6_NORM2_4,
    WOPBS_PARAM_MESSAGE_6_NORM2_6,
    WOPBS_PARAM_MESSAGE_6_NORM2_8,
    WOPBS_PARAM_MESSAGE_7_NORM2_2,
    WOPBS_PARAM_MESSAGE_7_NORM2_4,
    WOPBS_PARAM_MESSAGE_7_NORM2_6,
    WOPBS_PARAM_MESSAGE_7_NORM2_8,
    WOPBS_PARAM_MESSAGE_8_NORM2_2,
    WOPBS_PARAM_MESSAGE_8_NORM2_4,
    WOPBS_PARAM_MESSAGE_8_NORM2_6,
];

pub const WOPBS_PARAM_MESSAGE_1_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(512),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0003472352121441949901),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(24),
    pbs_level: DecompositionLevelCount(1),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(1),
    pfks_base_log: DecompositionBaseLog(24),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(2),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_1_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(502),
    glwe_dimension: GlweDimension(3),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.00041688866384199045524),
    glwe_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_1_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(499),
    glwe_dimension: GlweDimension(3),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.0004403915565001254653),
    glwe_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(9),
    pfks_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_1_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(500),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00043241360644590172285),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(16),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(16),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(2),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_2_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(488),
    glwe_dimension: GlweDimension(3),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.0005384866525630595423),
    glwe_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_2_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(488),
    glwe_dimension: GlweDimension(3),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.0005384866525630595423),
    glwe_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(9),
    pfks_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_2_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(16),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(16),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_2_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(497),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00045679174732062467505),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(16),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(16),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(4),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_3_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(488),
    glwe_dimension: GlweDimension(3),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.0005384866525630595423),
    glwe_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    cbs_level: DecompositionLevelCount(3),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(8),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_3_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(497),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00045679174732062467505),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(15),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(8),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_3_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(494),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00048254425233109359873),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(16),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(16),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(8),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_3_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(494),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00048254425233109359873),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(11),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(8),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_4_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(486),
    glwe_dimension: GlweDimension(3),
    polynomial_size: PolynomialSize(512),
    lwe_modular_std_dev: StandardDev(0.00055853990682276860028),
    glwe_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(9),
    pfks_modular_std_dev: StandardDev(0.000000000002573000821792597679153983627),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_4_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(497),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00045679174732062467505),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(15),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_4_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(11),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_4_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(11),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_5_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(497),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00045679174732062467505),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(15),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(32),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_5_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(15),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(32),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_5_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(32),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_5_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(5),
    message_modulus: MessageModulus(32),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_6_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(497),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00045679174732062467505),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(15),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(4),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(15),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_6_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(11),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_6_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_6_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(6),
    message_modulus: MessageModulus(64),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_7_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(16),
    pbs_level: DecompositionLevelCount(2),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(2),
    pfks_base_log: DecompositionBaseLog(16),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(128),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_7_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(11),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(128),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_7_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(6),
    message_modulus: MessageModulus(128),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_7_NORM2_8: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(4),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(4),
    pfks_base_log: DecompositionBaseLog(9),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(6),
    message_modulus: MessageModulus(128),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_8_NORM2_2: Parameters = Parameters {
    lwe_dimension: LweDimension(493),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00049144710341316649172),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(11),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(5),
    ks_base_log: DecompositionBaseLog(2),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(11),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(6),
    cbs_base_log: DecompositionBaseLog(3),
    message_modulus: MessageModulus(256),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_8_NORM2_4: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(12),
    pbs_level: DecompositionLevelCount(3),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(3),
    pfks_base_log: DecompositionBaseLog(12),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(5),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(256),
    carry_modulus: CarryModulus(1),
};
pub const WOPBS_PARAM_MESSAGE_8_NORM2_6: Parameters = Parameters {
    lwe_dimension: LweDimension(481),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.00061200133780220371345),
    glwe_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    pbs_base_log: DecompositionBaseLog(9),
    pbs_level: DecompositionLevelCount(4),
    ks_level: DecompositionLevelCount(9),
    ks_base_log: DecompositionBaseLog(1),
    pfks_level: DecompositionLevelCount(4),
    pfks_base_log: DecompositionBaseLog(9),
    pfks_modular_std_dev: StandardDev(0.00000000000000022148688116005568513645324585951),
    cbs_level: DecompositionLevelCount(4),
    cbs_base_log: DecompositionBaseLog(6),
    message_modulus: MessageModulus(256),
    carry_modulus: CarryModulus(1),
};

pub const PARAM_4_BITS_5_BLOCKS: Parameters = Parameters {
    lwe_dimension: LweDimension(667),
    glwe_dimension: GlweDimension(2),
    polynomial_size: PolynomialSize(1024),
    lwe_modular_std_dev: StandardDev(0.0000000004168323308734758),
    glwe_modular_std_dev: StandardDev(0.00000000000000000000000000000004905643852600863),
    pbs_base_log: DecompositionBaseLog(7),
    pbs_level: DecompositionLevelCount(6),
    ks_base_log: DecompositionBaseLog(1),
    ks_level: DecompositionLevelCount(14),
    pfks_level: DecompositionLevelCount(6),
    pfks_base_log: DecompositionBaseLog(7),
    pfks_modular_std_dev: StandardDev(0.00000000000000000000000000000004905643852600863),
    cbs_level: DecompositionLevelCount(7),
    cbs_base_log: DecompositionBaseLog(4),
    message_modulus: MessageModulus(16),
    carry_modulus: CarryModulus(1),
};