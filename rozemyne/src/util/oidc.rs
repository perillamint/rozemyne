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

use std::marker::PhantomData;

use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::http_client;
use openidconnect::{ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope};
use serde::{Deserialize, Serialize};

use crate::config::OIDCConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct OIDCContext {
    csrf_token: CsrfToken,
    nonce: Nonce,
}

pub struct OIDCAuthProvider<T> {
    client: CoreClient,
    claims: Vec<Scope>,
    _marker: PhantomData<T>,
}

impl<T> OIDCAuthProvider<T> {
    pub async fn new(config: &OIDCConfig, callback_url: &str) -> Result<Self, anyhow::Error> {
        let oidc_client_id = ClientId::new(config.client_id.to_owned());
        let oidc_client_secret = ClientSecret::new(config.client_secret.to_owned());
        let oidc_issuer = IssuerUrl::new(config.issuer.to_owned())?;

        let provider_meta = CoreProviderMetadata::discover(&oidc_issuer, http_client)?;

        let client = CoreClient::from_provider_metadata(
            provider_meta,
            oidc_client_id,
            Some(oidc_client_secret),
        )
        .set_redirect_uri(
            //FIXME: redirect url
            RedirectUrl::new(callback_url.to_string())?,
        );

        Ok(Self {
            client,
            claims: config
                .claims
                .iter()
                .map(|c| Scope::new(c.to_owned()))
                .collect(),
            _marker: PhantomData,
        })
    }

    pub async fn get_authorize_url(&self) -> (String, OIDCContext) {
        let mut authorize_req = self.client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );

        for scope in self.claims.iter() {
            authorize_req = authorize_req.add_scope(scope.clone());
        }

        let (auth_url, csrf_token, nonce) = authorize_req.url();

        (auth_url.to_string(), OIDCContext { csrf_token, nonce })
    }
}
