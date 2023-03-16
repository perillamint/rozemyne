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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWTClaim<T> {
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
    pub claims: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RozemyneClaim {
    pub is_admin: bool,
    pub permissions: Vec<String>,
}
