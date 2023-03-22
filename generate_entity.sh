#!/bin/bash

# SPDX-FileCopyrightText: 2023 perillamint
#
# SPDX-License-Identifier: CC0-1.0

set -e
cd "$(dirname "$0")"

sea-orm-cli generate entity -v -o rozemyne/src/entity --with-serde both

for file in rozemyne/src/entity/*.rs; do
    reuse addheader --copyright "perillamint" --license AGPL-3.0-or-later --template rust "$file"
done
