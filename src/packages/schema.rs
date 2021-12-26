use arrow::datatypes::{
    Schema,
    Field,
    DataType,
    TimeUnit,
};
use lazy_static::lazy_static;

lazy_static! {
//    static ref CONTRACTAGC: Schema = Schema::new(
//        vec![
//            Field::new("CONTRACTID", DataType::Utf8, false),
//            Field::new("VERSIONNO", DataType::Int8, false),
//            Field::new("STARTDATE", DataType::Date32, false),
//            Field::new("ENDDATE", DataType::Date32, false),
//            Field::new("PARTICIPANTID", DataType::Utf8, false),
//            Field::new("DUID", DataType::Utf8, false),
//            Field::new("CRR", DataType::Int16, false),
//            Field::new("CRL", DataType::Int16, false),
//            Field::new("RLPRICE", DataType::Float32, false),
//            Field::new("CCPRICE", DataType::Float32, false),
//            Field::new("BS", DataType::Float32, false),
//            Field::new("AUTHORISEDBY", DataType::Utf8, false),
//            Field::new("AUTHORISEDDATE", DataType::Date32, false),
//            Field::new("LASTCHANGED", DataType::Date32, false)
//        ]
//    );

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
}
