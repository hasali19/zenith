use db::WriteConnection;
use db::items::MediaItem;
use eyre::bail;
use serde::Deserialize;
use serde_json::json;
use time::format_description::well_known::Iso8601;
use time::{Duration, OffsetDateTime};

pub struct TraktClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Clone, Copy, Debug)]
pub enum VideoType {
    Movie,
    Episode,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

impl TraktClient {
    /// Exchanges the refresh token for a new access token. Returns `None` if
    /// Trakt rejects the refresh token for some reason. This is an
    /// indication that the token is invalid and the user needs to
    /// reauthenticate.
    pub async fn exchange_tokens(
        &self,
        refresh_token: &str,
    ) -> eyre::Result<Option<TokenResponse>> {
        let res = self
            .client
            .post(format!("{}/oauth/token", self.base_url))
            .json(&json!({
                "refresh_token": refresh_token,
                "client_id": &self.client_id,
                "client_secret": &self.client_secret,
                "redirect_uri": &self.redirect_uri,
                "grant_type": "refresh_token",
            }))
            .send()
            .await?;

        if res.status().is_client_error() {
            return Ok(None);
        }

        let res = res.error_for_status()?.json().await?;

        Ok(Some(res))
    }

    pub async fn sync_history_add(
        &self,
        access_token: &str,
        tmdb_id: i32,
        video_type: VideoType,
    ) -> eyre::Result<()> {
        let item_key = match video_type {
            VideoType::Movie => "movies",
            VideoType::Episode => "episodes",
        };

        self.client
            .post(format!("{}/sync/history", self.base_url))
            .header("trakt-api-version", "2")
            .header("trakt-api-key", &self.client_id)
            .bearer_auth(access_token)
            .json(&json!({
                item_key: [
                    {
                        "ids": {
                            "tmdb": tmdb_id,
                        },
                    },
                ],
            }))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

pub struct TraktService<'a> {
    client: &'a TraktClient,
    conn: &'a mut WriteConnection,
}

impl<'a> TraktService<'a> {
    pub fn new(client: &'a TraktClient, conn: &'a mut WriteConnection) -> TraktService<'a> {
        TraktService { client, conn }
    }

    pub async fn add_to_watch_history(
        &mut self,
        user_id: i64,
        item: &MediaItem,
    ) -> eyre::Result<bool> {
        let Some(access_token) = self.get_access_token(user_id).await? else {
            tracing::trace!("skipping as user is not connected to trakt");
            return Ok(false);
        };

        let Some(tmdb_id) = item.tmdb_id else {
            tracing::trace!(item.id, "skipping due to missing tmdb id");
            return Ok(false);
        };

        let video_type = match item.kind {
            db::media::MediaItemType::Movie => VideoType::Movie,
            db::media::MediaItemType::Show => bail!("shows are not currently supported"),
            db::media::MediaItemType::Season => bail!("seasons are not currently supported"),
            db::media::MediaItemType::Episode => VideoType::Episode,
        };

        self.client
            .sync_history_add(&access_token, tmdb_id, video_type)
            .await?;

        Ok(true)
    }

    async fn get_access_token(&mut self, user_id: i64) -> eyre::Result<Option<String>> {
        let Some(trakt_user) = db::trakt::get_user(self.conn.as_read(), user_id).await? else {
            tracing::warn!("user has not connected to trakt");
            return Ok(None);
        };

        let Some(refresh_token) = trakt_user.refresh_token else {
            tracing::warn!(user_id, "trakt connection is invalid");
            return Ok(None);
        };

        let is_expired = if let Some(expires_at) = trakt_user.expires_at {
            OffsetDateTime::now_utc() >= OffsetDateTime::parse(&expires_at, &Iso8601::DEFAULT)?
        } else {
            true
        };

        let access_token = match trakt_user.access_token {
            Some(token) if !is_expired => token,
            _ => {
                tracing::info!("requesting new access token from trakt");

                let Some(res) = self.client.exchange_tokens(&refresh_token).await? else {
                    tracing::error!(
                        "Failed to exchange trakt refresh token. The user will need to reauthenticate."
                    );
                    db::trakt::invalidate_tokens(&mut *self.conn, user_id).await?;
                    return Ok(None);
                };

                let expires_at =
                    OffsetDateTime::now_utc().saturating_add(Duration::seconds(res.expires_in));

                db::trakt::update_tokens(
                    &mut *self.conn,
                    user_id,
                    &res.refresh_token,
                    &res.access_token,
                    &expires_at,
                )
                .await?;

                res.access_token
            }
        };

        Ok(Some(access_token))
    }
}
