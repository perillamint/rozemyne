/*
 * SPDX-FileCopyrightText: 2022 perillamint
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};

mod config;
//mod middleware;

use config::read_config;

#[derive(clap::Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, short = 'c', value_name = "CONFIG")]
    config: String,
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = read_config(&ARGS.config);

    // Connect to the database
    let mut connopt = ConnectOptions::new(cfg.database.url);

    connopt
        .max_connections(cfg.database.max_connections)
        .min_connections(cfg.database.min_connections);

    let conn = Database::connect(connopt).await.unwrap();

    // Migrate the database
    Migrator::up(&conn, None).await.unwrap();

    HttpServer::new(|| App::new().service(hello))
        .bind((cfg.http.bind, cfg.http.port))?
        .run()
        .await
}
