/*
 * SPDX-FileCopyrightText: 2023 perillamint
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

use axum::body::Body;
use axum::routing::get;
use axum::Router;
use jwt_authorizer::JwtAuthorizer;

use crate::config::Config;
use crate::types::auth_token::{JWTClaim, RozemyneClaim};
use crate::types::AppState;

pub(crate) async fn get_route(config: &Config) -> Router<AppState, Body> {
    let jwt_auth: JwtAuthorizer<JWTClaim<RozemyneClaim>> =
        JwtAuthorizer::from_secret(&config.jwt.secret.clone())
            .check(|claims| claims.claims.is_admin);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            jwt_auth
                .layer()
                .await
                .expect("Failed to initialize JWT auth"),
        )
}
