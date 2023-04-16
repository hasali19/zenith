use camino::Utf8Path;
use time::{Date, OffsetDateTime};

use crate::config::{ImportMatcher, ImportMatcherTarget};

#[derive(Debug, PartialEq, Eq)]
pub struct MoviePathMeta {
    pub name: String,
    pub year: Option<OffsetDateTime>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EpisodePathMeta<'a> {
    pub show_name: &'a str,
    pub show_path: &'a Utf8Path,
    pub season: u32,
    pub episode: u32,
    pub name: Option<&'a str>,
}

pub struct SubtitlePathMeta<'a> {
    pub name: &'a str,
    pub lang: Option<&'a str>,
    pub sdh: bool,
    pub forced: bool,
}

pub struct PathParser<'a> {
    matchers: &'a [ImportMatcher],
}

impl<'a> PathParser<'a> {
    pub fn new(matchers: &[ImportMatcher]) -> PathParser {
        PathParser { matchers }
    }

    pub fn parse_movie_path(&self, path: &Utf8Path) -> Option<MoviePathMeta> {
        let name = path.file_name()?;
        self.matchers
            .iter()
            .filter(|m| m.target == ImportMatcherTarget::Movie)
            .find_map(|matcher| {
                let captures = matcher.regex.captures(name)?;

                let title = captures.name("title")?.as_str().replace('.', " ");
                let year = captures
                    .name("year")
                    .and_then(|v| v.as_str().parse::<u32>().ok());

                let year = year
                    .and_then(|year| Date::from_ordinal_date(year as i32, 1).ok())
                    .and_then(|date| date.with_hms(0, 0, 0).ok())
                    .map(|dt| dt.assume_utc());

                Some(MoviePathMeta { name: title, year })
            })
    }

    pub fn parse_episode_path<'b>(&self, path: &'b Utf8Path) -> Option<EpisodePathMeta<'b>> {
        let parent_path = path.parent()?;
        let parent_is_season = parent_path
            .file_name()
            .map(|name| name.starts_with("Season "))
            .unwrap_or(false);

        let show_path = if parent_is_season {
            parent_path.parent()?
        } else {
            parent_path
        };

        let file_name = path.file_name()?;
        let show_folder_name = show_path.file_name()?;

        self.matchers
            .iter()
            .filter(|m| m.target == ImportMatcherTarget::Episode)
            .find_map(|matcher| {
                let captures = matcher.regex.captures(file_name)?;

                let show_name = captures.name("show_name").map(|s| s.as_str());
                let name = captures.name("name").map(|s| s.as_str());
                let season = captures.name("season")?.as_str().parse().ok()?;
                let episode = captures.name("episode")?.as_str().parse().ok()?;

                Some(EpisodePathMeta {
                    show_name: show_name.unwrap_or(show_folder_name),
                    show_path,
                    season,
                    episode,
                    name,
                })
            })
    }

    pub fn parse_subtitle_path<'b>(&self, path: &'b Utf8Path) -> Option<SubtitlePathMeta<'b>> {
        fn split_ext(path: &str) -> (&str, Option<&str>) {
            if let Some(index) = path.rfind('.') {
                let (head, tail) = path.split_at(index);
                (head, Some(&tail[1..]))
            } else {
                (path, None)
            }
        }

        let Some(mut sub_file_name) = path.file_stem() else {
            return None;
        };

        let mut sdh = false;
        let mut forced = false;

        loop {
            let (name, ext) = split_ext(sub_file_name);
            let Some(ext) = ext else { break };

            if ext == "sdh" {
                sdh = true;
            } else if ext == "forced" {
                forced = true;
            } else {
                break;
            }

            sub_file_name = name;
        }

        let (mut sub_file_name, lang) = split_ext(sub_file_name);

        {
            let (name, i) = split_ext(sub_file_name);
            if let Some(i) = i && i.parse::<i32>().is_ok() {
                sub_file_name = name;
            }
        }

        Some(SubtitlePathMeta {
            name: sub_file_name,
            lang,
            sdh,
            forced,
        })
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_eq;
    use time::macros::datetime;

    use crate::config::Config;

    use super::*;

    fn parser() -> PathParser<'static> {
        static CONFIG: Lazy<Config> =
            Lazy::new(|| serde_yaml::from_str(include_str!("tests/config.yml")).unwrap());
        PathParser::new(&CONFIG.import.matchers)
    }

    #[test]
    fn movie_with_no_year() -> eyre::Result<()> {
        let result =
            parser().parse_movie_path(Utf8Path::new("/media/movies/Movie Name/Movie Name.mkv"));

        assert_eq!(
            result,
            Some(MoviePathMeta {
                name: "Movie Name".to_owned(),
                year: None,
            })
        );

        Ok(())
    }

    #[test]
    fn movie_with_year() {
        let result = parser().parse_movie_path(Utf8Path::new(
            "/media/movies/Movie Name (2022)/Movie Name (2022).mkv",
        ));

        assert_eq!(
            result,
            Some(MoviePathMeta {
                name: "Movie Name".to_owned(),
                year: Some(datetime!(2022-01-01 0:00 UTC)),
            })
        );
    }

    #[test]
    fn episode_simple() {
        let result =
            parser().parse_episode_path(Utf8Path::new("/media/shows/Show Name/S02E25.mkv"));

        assert_eq!(
            result,
            Some(EpisodePathMeta {
                show_name: "Show Name",
                show_path: Utf8Path::new("/media/shows/Show Name"),
                season: 2,
                episode: 25,
                name: None,
            })
        );
    }

    #[test]
    fn episode_with_name() {
        let result = parser().parse_episode_path(Utf8Path::new(
            "/media/shows/Show Name/Show Name - S02E25 - Episode Name.mkv",
        ));

        assert_eq!(
            result,
            Some(EpisodePathMeta {
                show_name: "Show Name",
                show_path: Utf8Path::new("/media/shows/Show Name"),
                season: 2,
                episode: 25,
                name: None,
            })
        );
    }

    #[test]
    fn episode_with_season_folder() {
        let result = parser().parse_episode_path(Utf8Path::new(
            "/media/shows/Show Name/Season 2/Show Name - S02E25 - Episode Name.mkv",
        ));

        assert_eq!(
            result,
            Some(EpisodePathMeta {
                show_name: "Show Name",
                show_path: Utf8Path::new("/media/shows/Show Name"),
                season: 2,
                episode: 25,
                name: None,
            })
        );
    }
}
