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
use axum::Router;
use jwt_authorizer::JwtAuthorizer;

use crate::config::Config;
use crate::types::auth_token::JWTTokenClaim;
use crate::types::AppState;

mod files;
mod user;

pub(crate) async fn get_route(config: &Config) -> Router<AppState, Body> {
    let jwt_auth: JwtAuthorizer<JWTTokenClaim> = JwtAuthorizer::from_secret("topsecret");
    Router::new()
        .nest("/user", user::get_route(config).await)
        .nest("/files", files::get_route(config).await)
        .layer(
            jwt_auth
                .layer()
                .await
                .expect("Failed to initialize JWT auth"),
        )
}
