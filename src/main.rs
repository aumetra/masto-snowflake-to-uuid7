use serde_json::Value;
use std::time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;
use uuid7::V7Generator;
use wyrand::WyRand;

const POST: &str = r##"
{
    "id":"110310997180177747",
    "created_at":"2023-05-04T14:57:17.246Z",
    "in_reply_to_id":"110310990615273183",
    "in_reply_to_account_id":"109564217772338331",
    "sensitive":false,
    "spoiler_text":"",
    "visibility":"unlisted",
    "language":"en",
    "uri":"https://floss.social/users/XOrgFoundation/statuses/110310997180177747",
    "url":"https://floss.social/@XOrgFoundation/110310997180177747",
    "replies_count":25,
    "reblogs_count":417,
    "favourites_count":415,
    "edited_at":null,
    "content":"\u003cp\u003e\u003cspan class=\"h-card\"\u003e\u003ca href=\"https://lea.pet/@lea\" class=\"u-url mention\"\u003e@\u003cspan\u003elea\u003c/span\u003e\u003c/a\u003e\u003c/span\u003e O :wayland: O\u003c/p\u003e",
    "reblog":null,
    "application":null,
    "account":{
    "id":"110153360857508949",
    "username":"XOrgFoundation",
    "acct":"XOrgFoundation",
    "display_name":"X.Org Foundation",
    "locked":false,
    "bot":false,
    "discoverable":true,
    "group":false,
    "created_at":"2023-04-06T00:00:00.000Z",
    "note":"\u003cp\u003eNon-profit supporting the development \u0026amp; promotion of the free graphics software stack (X11, Mesa, DRI, Wayland, etc.).\u003c/p\u003e",
    "url":"https://floss.social/@XOrgFoundation",
    "avatar":"https://cdn.masto.host/floss/accounts/avatars/110/153/360/857/508/949/original/3004374788c4713d.png",
    "avatar_static":"https://cdn.masto.host/floss/accounts/avatars/110/153/360/857/508/949/original/3004374788c4713d.png",
    "header":"https://floss.social/headers/original/missing.png",
    "header_static":"https://floss.social/headers/original/missing.png",
    "followers_count":4634,
    "following_count":10,
    "statuses_count":105,
    "last_status_at":"2023-05-06",
    "noindex":true,
    "emojis":[
    ],
    "roles":[
    ],
    "fields":[
    {
    "name":"Website",
    "value":"\u003ca href=\"https://x.org\" target=\"_blank\" rel=\"nofollow noopener noreferrer me\"\u003e\u003cspan class=\"invisible\"\u003ehttps://\u003c/span\u003e\u003cspan class=\"\"\u003ex.org\u003c/span\u003e\u003cspan class=\"invisible\"\u003e\u003c/span\u003e\u003c/a\u003e",
    "verified_at":"2023-05-03T23:24:04.529+00:00"
    }
    ]
    },
    "media_attachments":[
    ],
    "mentions":[
    {
    "id":"109564217772338331",
    "username":"lea",
    "url":"https://lea.pet/@lea",
    "acct":"lea@lea.pet"
    }
    ],
    "tags":[
    ],
    "emojis":[
    {
    "shortcode":"wayland",
    "url":"https://cdn.masto.host/floss/custom_emojis/images/000/136/040/original/1df1682fc952ccfa.png",
    "static_url":"https://cdn.masto.host/floss/custom_emojis/images/000/136/040/static/1df1682fc952ccfa.png",
    "visible_in_picker":true
    }
    ],
    "card":null,
    "poll":null
}
"##;

fn main() {
    let parsed_post: Value = serde_json::from_str(POST).unwrap();
    let post_id: u64 = parsed_post["id"].as_str().unwrap().parse().unwrap();
    let timestamp = Duration::from_millis((post_id >> 16) & 0xFF_FF_FF_FF_FF_FF);

    let rng = WyRand::new(post_id & 0xFF_FF);
    let mut generator = V7Generator::new(rng);
    let deterministic_id = generator
        .generate_or_abort_core(timestamp.as_millis() as u64, 10_000)
        .unwrap();

    println!("Deterministic Mastodon ID: {deterministic_id}");

    let rng = WyRand::new(post_id & 0xFF_FF);
    let mut generator = V7Generator::new(rng);
    let deterministic_id = generator
        .generate_or_abort_core(timestamp.as_millis() as u64, 10_000)
        .unwrap();

    println!("Did it again. Deterministic ID: {deterministic_id}");

    let masto_time =
        OffsetDateTime::from_unix_timestamp_nanos(timestamp.as_nanos() as i128).unwrap();
    let (uuid_time_secs, uuid_time_nanos) = Uuid::from(deterministic_id)
        .get_timestamp()
        .unwrap()
        .to_unix();
    let uuid_time_secs = Duration::from_secs(uuid_time_secs);
    let uuid_time = OffsetDateTime::from_unix_timestamp_nanos(
        (uuid_time_secs.as_nanos() as i128) + (uuid_time_nanos as i128),
    )
    .unwrap();

    println!("Mastodon timestamp: {masto_time}\nUUID timestamp: {uuid_time}");
}
