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

pub struct RozemyneError {
    error: anyhow::Error,
}

impl From<anyhow::Error> for RozemyneError {
    fn from(error: anyhow::Error) -> Self {
        Self { error }
    }
}

impl axum::response::IntoResponse for RozemyneError {
    fn into_response(self) -> axum::response::Response {
        let status_code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        (status_code, self.error.to_string()).into_response()
    }
}
