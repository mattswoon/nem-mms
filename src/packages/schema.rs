use arrow::datatypes::{
    Schema,
    Field,
    DataType,
    TimeUnit,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DISPATCH_UNIT_SCADA: Schema = Schema::new(
        vec![
            Field::new("DUID", DataType::Utf8, false),
            Field::new("SETTLEMENTDATE", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("SCADAVALUE", DataType::Float64, true)
        ]
    );

    pub static ref DISPATCH_NEGATIVE_RESIDUE: Schema = Schema::new(
        vec![
            Field::new("SETTLEMENTDATE", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("NRM_DATETIME", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("DIRECTIONAL_INTERCONNECTORID", DataType::Utf8, false),
            Field::new("NRM_ACTIVATED_FLAG", DataType::Boolean, true),
            Field::new("CUMUL_NEGRESIDUE_AMOUNT", DataType::Float64, true),
            Field::new("CUMUL_NEGRESIDUE_PREV_TI", DataType::Float64, true),
            Field::new("NEGRESIDUE_CURRENT_TI", DataType::Float64, true),
            Field::new("NEGRESIDUE_PD_NEXT_TI", DataType::Float64, true),
            Field::new("PRICE_REVISION", DataType::Utf8, true),
            Field::new("PREDISPATCHSEQNO", DataType::Utf8, true),
            Field::new("EVENT_ACTIVATED_DI", DataType::Timestamp(TimeUnit::Second, None), true),
            Field::new("EVENT_DEACTIVATED_DI", DataType::Timestamp(TimeUnit::Second, None), true),
            Field::new("DI_NOTBINDING_COUNT", DataType::Int16, true),
            Field::new("DI_VIOLATED_COUNT", DataType::Int16, true),
            Field::new("NRM_CONSTRAINT_BLOCKED_FLAG", DataType::Boolean, true)
        ]
    );

    pub static ref DISPATCH_LOCAL_PRICE: Schema = Schema::new(
        vec![
            Field::new("SETTLEMENTDATE", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("DUID", DataType::Utf8, false),
            Field::new("LOCAL_PRICE_ADJUSTMENT", DataType::Float64, true),
            Field::new("LOCALLY_CONSTRAINED", DataType::Int8, true)
        ]
    );

    pub static ref ROOFTOP_PV_ACTUAL: Schema = Schema::new(
        vec![
            Field::new("INTERVAL_DATETIME", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("TYPE", DataType::Utf8, false),
            Field::new("REGIONID", DataType::Utf8, false),
            Field::new("POWER", DataType::Float64, true),
            Field::new("QI", DataType::Float64, true),
            Field::new("LASTCHANGED", DataType::Timestamp(TimeUnit::Second, None), true)
        ]
    );
}
