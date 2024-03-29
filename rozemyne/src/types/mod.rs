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

use crate::config::Config;
use openidconnect::core::CoreClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::util::oidc::OIDCAuthProvider;

pub mod auth_token;

#[derive(Clone)]
pub(crate) struct AppState {
    pub dbconn: DatabaseConnection,
    pub config: Arc<Config>,
    pub oidc: Arc<OIDCAuthProvider<String>>,
}
