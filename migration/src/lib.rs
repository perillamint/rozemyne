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

pub use sea_orm_migration::prelude::*;

mod m20220902_200728_create_user_table;
mod m20230322_044417_create_user_idp_id_table;
mod m20230322_051542_create_book_table;
mod m20230322_061805_create_facet_table;
mod m20230322_070150_create_book_file_table;
mod m20230322_074753_create_book_x_facet_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220902_200728_create_user_table::Migration),
            Box::new(m20230322_044417_create_user_idp_id_table::Migration),
            Box::new(m20230322_051542_create_book_table::Migration),
            Box::new(m20230322_061805_create_facet_table::Migration),
            Box::new(m20230322_070150_create_book_file_table::Migration),
            Box::new(m20230322_074753_create_book_x_facet_table::Migration),
        ]
    }
}
