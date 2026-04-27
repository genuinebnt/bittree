// bootstrap_db(cfg: &DatabaseSettings) -> Result<PgPool>
// matches DatabaseSettings::Postgres, builds PgPoolOptions with max/min/idle settings,
// returns connect_lazy pool
