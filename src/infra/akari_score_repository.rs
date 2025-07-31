use anyhow::{Context, Result};
use async_trait::async_trait;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::domain::{
    infra::AkariScoreRepository,
    models::{AkariPrecisionScore, AkariScore},
};

table! {
    scores (user_id) {
        user_id -> Varchar,
        has_precision -> Bool,
        is_perfect -> Bool,
        precision_percentage -> Int8,
        time_sec -> Int8,
    }
}

// This struct represents a score to be inserted into the database.
// We'll derive Insertable to allow Diesel to create an INSERT statement from it.
#[derive(Insertable)]
#[diesel(table_name = scores)]
pub struct NewScore {
    pub user_id: String,
    pub has_precision: bool,
    pub is_perfect: bool,
    pub precision_percentage: i64,
    pub time_sec: i64,
}

// This struct represents a score retrieved from the database.
// We'll derive Queryable to allow Diesel to map a database row to this struct.
#[derive(Queryable)]
pub struct Score {
    pub user_id: String,
    pub has_precision: bool,
    pub is_perfect: bool,
    pub precision_percentage: i64,
    pub time_sec: i64,
}

pub struct DieselAkariScoreRepository {
    pool: Arc<Mutex<Pool<ConnectionManager<MysqlConnection>>>>,
}

impl DieselAkariScoreRepository {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        DieselAkariScoreRepository {
            pool: Arc::new(Mutex::new(pool)),
        }
    }

    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<MysqlConnection>> {
        let conn = self
            .pool
            .lock()
            .expect("Failed to lock the connection pool")
            .get()
            .expect("Failed to get connection from pool");
        conn
    }
}

#[async_trait]
impl AkariScoreRepository for DieselAkariScoreRepository {
    async fn try_save_score(&self, score: AkariScore, user_id: uuid::Uuid) -> Result<()> {
        let new_score = NewScore {
            user_id: user_id.to_string(),
            has_precision: matches!(
                score.precision,
                AkariPrecisionScore::Perfect | AkariPrecisionScore::ImperfectWithPercentage(_)
            ),
            is_perfect: matches!(score.precision, AkariPrecisionScore::Perfect),
            precision_percentage: match score.precision {
                AkariPrecisionScore::Perfect => 100, // By definition, perfect score is 100%
                AkariPrecisionScore::ImperfectWithPercentage(p) => p,
                AkariPrecisionScore::NotAvailable => 0, // Assuming 0 for not available
            },
            time_sec: score.time_sec,
        };

        {
            let mut conn = self.get_conn();
            diesel::insert_into(scores::table)
                .values(&new_score)
                .execute(&mut conn)?;
            std::mem::drop(conn);
        }
        Ok(())
    }

    async fn get_scores(&self) -> Result<HashMap<Uuid, AkariScore>> {
        let results = {
            let mut conn = self.get_conn();
            let results: Vec<Score> = scores::table
                .load(&mut conn)
                .context("Failed to load scores from database")?;
            std::mem::drop(conn);
            results
        };
        let results_map = results
            .into_iter()
            .map(|score| {
                let user_id =
                    Uuid::parse_str(&score.user_id).expect("Failed to parse user_id from score");
                let precision = if score.has_precision {
                    if score.is_perfect {
                        AkariPrecisionScore::Perfect
                    } else {
                        AkariPrecisionScore::ImperfectWithPercentage(score.precision_percentage)
                    }
                } else {
                    AkariPrecisionScore::NotAvailable
                };
                (
                    user_id,
                    AkariScore {
                        precision,
                        time_sec: score.time_sec,
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        Ok(results_map)
    }

    async fn refresh_all_scores(&self) -> Result<()> {
        let mut conn = self.get_conn();
        diesel::delete(scores::table)
            .execute(&mut conn)
            .context("Failed to delete scores from database")?;
        Ok(())
    }
}
