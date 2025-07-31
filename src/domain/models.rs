#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AkariPrecisionScore {
    Perfect,
    ImperfectWithPercentage(i64),
    NotAvailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AkariScore {
    pub precision: AkariPrecisionScore,
    pub time_sec: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct User {
    pub name: String,
    pub id: uuid::Uuid,
}
