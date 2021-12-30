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

    pub static ref ROOFTOP_PV_FORECAST: Schema = Schema::new(
        vec![
            Field::new("VERSION_DATETIME", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("REGIONID", DataType::Utf8, false),
            Field::new("INTERVAL_DATETIME", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("POWERMEAN", DataType::Float64, true),
            Field::new("POWERPOE50", DataType::Float64, true),
            Field::new("POWERPOELOW", DataType::Float64, true),
            Field::new("POWERPOEHIGH", DataType::Float64, true),
            Field::new("LASTCHANGED", DataType::Timestamp(TimeUnit::Second, None), true)
        ]
    );

    pub static ref DISPATCHPRICE: Schema = Schema::new(
        vec![
            Field::new("SETTLEMENTDATE", DataType::Timestamp(TimeUnit::Second, None), false),
            Field::new("RUNNO", DataType::Int16, false),
            Field::new("REGIONID", DataType::Utf8, false),
            Field::new("DISPATCHINTERVAL", DataType::Utf8, false),
            Field::new("INTERVENTION", DataType::Int16, false),
            Field::new("RRP", DataType::Float64, true),
            Field::new("EEP", DataType::Float64, true),
            Field::new("ROP", DataType::Float64, true),
            Field::new("APCFLAG", DataType::Int16, true),
            Field::new("MARKETSUSPENDEDFLAG", DataType::Int16, true),
            Field::new("LASTCHANGED", DataType::Timestamp(TimeUnit::Second, None), true),
            Field::new("RAISE6SECRRP", DataType::Float64, true),
            Field::new("RAISE6SECROP", DataType::Float64, true),
            Field::new("RAISE6SECAPCFLAG", DataType::Int16, true),
            Field::new("RAISE60SECRRP", DataType::Float64, true),
            Field::new("RAISE60SECROP", DataType::Float64, true),
            Field::new("RAISE60SECAPCFLAG", DataType::Int16, true),
            Field::new("RAISE5MINRRP", DataType::Float64, true),
            Field::new("RAISE5MINROP", DataType::Float64, true),
            Field::new("RAISE5MINAPCFLAG", DataType::Int16, true),
            Field::new("RAISEREGRRP", DataType::Float64, true),
            Field::new("RAISEREGROP", DataType::Float64, true),
            Field::new("RAISEREGAPCFLAG", DataType::Int16, true),
            Field::new("LOWER6SECRRP", DataType::Float64, true),
            Field::new("LOWER6SECROP", DataType::Float64, true),
            Field::new("LOWER6SECAPCFLAG", DataType::Int16, true),
            Field::new("LOWER60SECRRP", DataType::Float64, true),
            Field::new("LOWER60SECROP", DataType::Float64, true),
            Field::new("LOWER60SECAPCFLAG", DataType::Int16, true),
            Field::new("LOWER5MINRRP", DataType::Float64, true),
            Field::new("LOWER5MINROP", DataType::Float64, true),
            Field::new("LOWER5MINAPCFLAG", DataType::Int16, true),
            Field::new("LOWERREGRRP", DataType::Float64, true),
            Field::new("LOWERREGROP", DataType::Float64, true),
            Field::new("LOWERREGAPCFLAG", DataType::Int16, true),
            Field::new("PRICE_STATUS", DataType::Utf8, true),
            Field::new("PRE_AP_ENERGY_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_RAISE6_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_RAISE60_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_RAISE5MIN_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_RAISEREG_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_LOWER6_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_LOWER60_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_LOWER5MIN_PRICE", DataType::Float64, true),
            Field::new("PRE_AP_LOWERREG_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_ENERGY_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_RAISE6_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_RAISE60_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_RAISE5MIN_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_RAISEREG_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_LOWER6_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_LOWER60_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_LOWER5MIN_PRICE", DataType::Float64, true),
            Field::new("CUMUL_PRE_AP_LOWERREG_PRICE", DataType::Float64, true),
            Field::new("OCD_STATUS", DataType::Utf8, true),
            Field::new("MII_STATUS", DataType::Utf8, true),
        ]
    );
}
