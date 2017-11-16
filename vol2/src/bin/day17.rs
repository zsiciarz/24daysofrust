#![feature(custom_attribute)]

#[cfg(target_family = "unix")]
#[macro_use]
extern crate diesel;

#[cfg(target_family = "unix")]
#[macro_use]
extern crate diesel_codegen;

#[cfg(target_family = "unix")]
extern crate dotenv;

#[cfg(target_family = "unix")]
use diesel::prelude::*;
#[cfg(target_family = "unix")]
use diesel::pg::PgConnection;
#[cfg(target_family = "unix")]
use diesel::expression::sql_literal::sql;

#[cfg(target_family = "unix")]
use std::env;

#[cfg(target_family = "unix")]
mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

#[cfg(target_family = "unix")]
use schema::*;

#[cfg(target_family = "unix")]
#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
}

#[cfg(target_family = "unix")]
#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User)]
pub struct Photo {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub tags: Vec<String>,
}

#[cfg(target_family = "unix")]
#[derive(Debug, Insertable)]
#[table_name = "photos"]
pub struct NewPhoto {
    pub user_id: i32,
    pub url: String,
}

#[cfg(target_family = "unix")]
impl User {
    fn new_photo(&self, url: &str) -> NewPhoto {
        NewPhoto {
            user_id: self.id,
            url: url.to_string(),
        }
    }
}


#[cfg(target_family = "windows")]
fn main() {
    println!("TODO");
}

#[cfg(target_family = "unix")]
fn main() {
    println!("24 Days of Rust vol. 2 - diesel");
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).expect("Couldn't connect to DB");

    let me = users::table.find(1).first::<User>(&conn).expect(
        "Error loading user",
    );
    println!("{:?}", me);

    let deleted_count = diesel::delete(photos::table.filter(photos::id.gt(1)))
        .execute(&conn)
        .expect("Failed to clean up photos");
    println!("Deleted {} photo(s)", deleted_count);

    let photo_count: i64 = Photo::belonging_to(&me).count().first(&conn).expect(
        "Error counting photos",
    );
    println!("User {} has {} photo(s)", me.username, photo_count);
    print_sql!(Photo::belonging_to(&me).count());
    let my_photos = Photo::belonging_to(&me).load::<Photo>(&conn).expect(
        "Error loading photos",
    );
    println!("{:?}", my_photos);

    let photos: Vec<(Photo, User)> = photos::table.inner_join(users::table).load(&conn).expect(
        "Error loading photos",
    );
    for (photo, user) in photos {
        println!("Photo #{} by {}", photo.id, user.username);
    }

    let users_with_cat_photos: Vec<String> = users::table
        .select(users::username)
        .inner_join(photos::table)
        .filter(photos::url.like("%cat%"))
        .group_by(users::id)
        .load(&conn)
        .expect("Error loading users");
    println!("{:?}", users_with_cat_photos);

    let photo = me.new_photo("http://lorempixel.com/output/cats-q-c-640-480-8.jpg");
    let mut inserted_photo = diesel::insert(&photo)
        .into(photos::table)
        .get_result::<Photo>(&conn)
        .expect("Failed to insert photo");
    println!("{:?}", inserted_photo);

    inserted_photo.tags = vec!["cute".to_string(), "kitten".to_string()];
    let updated_photo: Photo = inserted_photo.save_changes(&conn).expect(
        "Error updating photo",
    );
    println!("{:?}", updated_photo);

    let cute_cat_count: i64 = photos::table
        .filter(photos::tags.contains(vec!["cute", "kitten"]))
        .count()
        .get_result(&conn)
        .expect("Error counting cute kittens");
    println!("There's {} photos of cute cats", cute_cat_count);

    let cute_cat_count: i64 = sql(
        "select count(*) from photos \
                                   where tags @> array['cute', 'kitten']",
    ).get_result(&conn)
        .expect("Error executing raw SQL");
    println!("There's {} photos of cute cats", cute_cat_count);
}
