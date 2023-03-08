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

use axum::routing::get;
use axum::Router;
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};

use jwt_authorizer::JwtAuthorizer;

mod api;
mod config;
mod types;
mod util;
//mod middleware;

use config::read_config;
use types::auth_token::JWTTokenClaim;
use types::AppState;

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

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let cfg = read_config(&ARGS.config);
    tracing_subscriber::fmt::init();

    // Connect to the database
    let mut connopt = ConnectOptions::new(cfg.database.url.clone());

    connopt
        .max_connections(cfg.database.max_connections)
        .min_connections(cfg.database.min_connections);

    let conn = Database::connect(connopt)
        .await
        .expect("Failed to connect to database!");

    // Migrate the database
    Migrator::up(&conn, None).await.unwrap();

    let state: AppState = AppState { dbconn: conn };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api::get_route(&cfg).await)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    tracing::info!("Listening on localhost:3000...");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
