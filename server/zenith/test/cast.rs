use insta::assert_json_snapshot;
use test_macros::test;

use crate::{TestApp, with_app};

#[test(with_app)]
async fn set_match_for_unmatched_episode_with_matched_show(mut app: TestApp) {
    assert_json_snapshot!(app.get("/cast/config").await);
}
