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
}
