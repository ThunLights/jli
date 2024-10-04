use std::io::{Error, ErrorKind};

use sqlx::{Pool, Sqlite, SqlitePool};

use super::{id::generate_id, url::url_format_check};

#[derive(sqlx::FromRow)]
pub struct Site {
	pub link: String,
	pub id: String,
}

pub struct DBClient {
    pub sites_database: Pool<Sqlite>,
	pub id_size: u16
}
impl DBClient {
    pub async fn new(db_path: &str, id_size: u16) -> Self {
        let sites_database = SqlitePool::connect(db_path).await.expect("DBClient fn new Err");
        Self {
            sites_database,
			id_size,
		}
    }
	pub async fn link2id(&self, link: &str) -> Result<String, Error> {
		if !url_format_check(link) {
			return Err(Error::new(ErrorKind::Other, "URL Format Error"));
		}
		if let Ok(site) = self.get_id(link).await {
			return Ok(site.id);
		} else {
			let id = self.id_generator().await;
			if self.add_link(link, &id).await {
				return Ok(id);
			} else {
				return Err(Error::new(ErrorKind::Other, "DataBase insert Error"));
			}
		}
	}
	pub async fn id2link(&self, id: &str) -> Result<String, Error> {
		if let Ok(site) = sqlx::query_as::<_, Site>("SELECT * FROM SITES where id = ?;").bind(id).fetch_one(&self.sites_database).await {
			return Ok(site.link);
		}

		Err(Error::new(ErrorKind::Other, "ID NOT FOUND"))
	}
	async fn id_generator(&self) -> String {
		let mut id = generate_id(self.id_size.into());
		while self.check_id(&id).await {
			id = generate_id(self.id_size.into());
		}
		id
	}
	async fn add_link(&self, link: &str, id: &str) -> bool {
		sqlx::query("INSERT INTO sites (link, id) VALUES ($1, $2)").bind(link).bind(id).execute(&self.sites_database).await.is_ok()
	}
	async fn check_id(&self, id: &str) -> bool {
		sqlx::query_as::<_, Site>("SELECT * FROM SITES where id = ?;").bind(id).fetch_one(&self.sites_database).await.is_ok()
	}
	async fn get_id(&self, link: &str) -> Result<Site, Error> {
        if let Ok(row) = sqlx::query_as::<_, Site>("SELECT * FROM SITES where link = ?;").bind(link).fetch_one(&self.sites_database).await {
            return Ok(row)
        }

        Err(Error::new(ErrorKind::Other, "Site Not Found"))
	}
}