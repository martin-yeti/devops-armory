use chrono::{Duration, NaiveDate, Utc};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Bool, Nullable, Text};
use log::{info, warn};

/// Daily `logs` partitions are created this many days ahead, so a
/// collector that is down for a while doesn't leave whole days to land in
/// the default partition. Rows there can't be pruned by date, and once a
/// day's rows are in it, that day's partition can no longer be created.
pub const PARTITION_DAYS_AHEAD: i64 = 7;

#[derive(QueryableByName)]
struct PartitionedTable {
    #[diesel(sql_type = Nullable<Text>)]
    default_partition: Option<String>,
}

#[derive(QueryableByName)]
struct PartitionName {
    #[diesel(sql_type = Text)]
    name: String,
}

#[derive(QueryableByName)]
struct HasRows {
    #[diesel(sql_type = Bool)]
    has_rows: bool,
}

/// Creates the daily partitions `logs_p_YYYY_MM_DD`, covering
/// [day 00:00 UTC, next day 00:00 UTC), for today and the next
/// PARTITION_DAYS_AHEAD days. New partitions get the parent's indexes
/// automatically.
/// Does nothing when `logs` isn't partitioned, as with the plain table the
/// bundled migrations create. A day that already has rows in the default
/// partition is skipped with a warning: creating it would scan the whole
/// default partition while locking `logs`, and then fail anyway.
pub fn ensure_log_partitions(connection: &mut PgConnection) -> Result<(), anyhow::Error> {
    let Some(table) = sql_query(
        "SELECT NULLIF(partdefid, 0)::regclass::text AS default_partition \
         FROM pg_partitioned_table WHERE partrelid = 'logs'::regclass",
    )
    .get_result::<PartitionedTable>(connection)
    .optional()?
    else {
        return Ok(());
    };

    let existing: Vec<String> = sql_query(
        "SELECT c.relname::text AS name FROM pg_inherits i \
         JOIN pg_class c ON c.oid = i.inhrelid \
         WHERE i.inhparent = 'logs'::regclass",
    )
    .load::<PartitionName>(connection)?
    .into_iter()
    .map(|p| p.name)
    .collect();

    let today = Utc::now().date_naive();
    for offset in 0..=PARTITION_DAYS_AHEAD {
        let day = today + Duration::days(offset);
        let name = partition_name(day);
        if existing.contains(&name) {
            continue;
        }
        if let Err(err) = create_partition(connection, &name, day, table.default_partition.as_deref()) {
            warn!("Couldn't create log partition {}: {}", name, err);
        }
    }

    Ok(())
}

fn partition_name(day: NaiveDate) -> String {
    format!("logs_p_{}", day.format("%Y_%m_%d"))
}

fn create_partition(
    connection: &mut PgConnection,
    name: &str,
    day: NaiveDate,
    default_partition: Option<&str>,
) -> Result<(), anyhow::Error> {
    let from = format!("{} 00:00:00+00", day);
    let to = format!("{} 00:00:00+00", day + Duration::days(1));

    if let Some(default_partition) = default_partition {
        let day_rows = sql_query(format!(
            "SELECT EXISTS (SELECT 1 FROM {default_partition} \
             WHERE time >= $1::timestamptz AND time < $2::timestamptz) AS has_rows"
        ))
        .bind::<Text, _>(&from)
        .bind::<Text, _>(&to)
        .get_result::<HasRows>(connection)?;
        if day_rows.has_rows {
            warn!(
                "Skipping log partition {}: {} already holds rows for that day; move them into a partition by hand",
                name, default_partition
            );
            return Ok(());
        }

        let any_rows = sql_query(format!(
            "SELECT EXISTS (SELECT 1 FROM {default_partition}) AS has_rows"
        ))
        .get_result::<HasRows>(connection)?;
        if any_rows.has_rows {
            warn!(
                "{} isn't empty, so creating {} scans all of it while holding a lock on logs",
                default_partition, name
            );
        }
    }

    // CREATE TABLE ... PARTITION OF takes an ACCESS EXCLUSIVE lock on
    // `logs`. While it waits for that lock behind a long-running query,
    // every insert and query queued after it waits too, so give up quickly
    // and retry on the next check instead.
    connection.transaction::<_, anyhow::Error, _>(|connection| {
        sql_query("SET LOCAL lock_timeout = '5s'").execute(connection)?;
        sql_query(format!(
            "CREATE TABLE IF NOT EXISTS {name} PARTITION OF logs \
             FOR VALUES FROM ('{from}') TO ('{to}')"
        ))
        .execute(connection)?;
        Ok(())
    })?;

    info!("Created log partition {}", name);
    Ok(())
}
