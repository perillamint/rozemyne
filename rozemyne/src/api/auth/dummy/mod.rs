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
use axum::error_handling::HandleError;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::RozemyneError;
use crate::types::AppState;

use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseClaim {
    /// JWT ID claim provides a unique idntifier for the JWT.
    pub jti: Option<String>,
    /// Subject
    pub sub: Option<String>,
    // iss (issuer)
    pub iss: String,
    /// exp is the expiration time in unix time
    pub exp: i64,
    /// nbf (not before) claim identifies the time before
    /// which JWT must not be accepted for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    /// iat (issued at that time) means issued time of the JWT.
    pub iat: i64,
}

// Return signed token, without any authorization.
// Not for production run, of course.
async fn issue_token(State(state): State<AppState>) -> Result<String, RozemyneError> {
    let cfg = state.config.as_ref();
    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };

    let claim = BaseClaim {
        jti: Some("test".to_owned()),
        sub: Some("testsub".to_owned()),
        iss: "Testissuer".to_owned(),
        exp: 9999999999999,
        nbf: None,
        iat: 1,
    };

    let token = encode(
        &header,
        &claim,
        &EncodingKey::from_secret(cfg.jwt.secret.as_bytes()),
    );
    Ok(token.unwrap())
}

pub(crate) async fn get_route(config: &Config) -> Router<AppState, Body> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/token", get(issue_token))
}
