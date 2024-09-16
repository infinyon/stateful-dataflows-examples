# Reddit Subreddit Streamer
This example with stream new posts from different subreddits using the endpoint **https://www.reddit.com/r/.../new.json**

## Starting the connectors
By running the generator.sh script we are able to start a connector any subreddit.
```
./generator Honda
./generator Toyota
./generator Mazda
```
The script will generate a connector file. It will also automatically install all the packages necessary. The connector extracts the newest post via the jolt. Other params are available and the `generator` script can be edit to include them. The full list is in the bottom of the README. 

The connector will produce for only the `reddit-sub-posts` topic. It has no deduplication.

## Stateful for deduplication
A stateful dataflow is also included in the repo for deduplication. To run it:
```
sdf run --ephemeral --ui
```
The sink topic is `reddit-nodup`.

## Clean up
The connectors can add up and get pretty annoying. A clean script `cleanconn` is included to remove all running and stopped connectors. **It removes all connectors!**

## Problems
The endpoint actually is a dump of x amount of new posts for a subreddit and can get really big. Sometimes things don't work out or lag a lot. I may be good to edit the interval that the connector polls the subreddit. 

The connector is not suitable for reading subreddits that are very active and get more than one posts per five seconds. You can edit the interval it polls at. It can only read the new post.

The connector will read old posts if the new post is removed. So it may reread a post twice. It is not true deduplication(but close enough?).

## Full body of request
The reddit object will return the following, please edit the generator if you find any of the information useful or necessary.
```
"approved_at_utc": null,
"subreddit": "test",
"selftext": "test",
"author_fullname": "t2_idma42ut",
"saved": false,
"mod_reason_title": null,
"gilded": 0,
"clicked": false,
"title": "test",
"link_flair_richtext": [],
"subreddit_name_prefixed": "r/test",
"hidden": false,
"pwls": 6,
"link_flair_css_class": null,
"downs": 0,
"thumbnail_height": null,
"top_awarded_type": null,
"hide_score": true,
"name": "t3_1fg4o4m",
"quarantine": false,
"link_flair_text_color": "dark",
"upvote_ratio": 1,
"author_flair_background_color": null,
"subreddit_type": "public",
"ups": 1,
"total_awards_received": 0,
"media_embed": {},
"thumbnail_width": null,
"author_flair_template_id": null,
"is_original_content": false,
"user_reports": [],
"secure_media": null,
"is_reddit_media_domain": false,
"is_meta": false,
"category": null,
"secure_media_embed": {},
"link_flair_text": null,
"can_mod_post": false,
"score": 1,
"approved_by": null,
"is_created_from_ads_ui": false,
"author_premium": false,
"thumbnail": "self",
"edited": false,
"author_flair_css_class": null,
"author_flair_richtext": [],
"gildings": {},
"content_categories": null,
"is_self": true,
"mod_note": null,
"created": 1726259389,
"link_flair_type": "text",
"wls": 6,
"removed_by_category": null,
"banned_by": null,
"author_flair_type": "text",
"domain": "self.test",
"allow_live_comments": false,
"selftext_html": "&lt;!-- SC_OFF --&gt;&lt;div class=\"md\"&gt;&lt;p&gt;test&lt;/p&gt;\n&lt;/div&gt;&lt;!-- SC_ON --&gt;",
"likes": null,
"suggested_sort": null,
"banned_at_utc": null,
"view_count": null,
"archived": false,
"no_follow": false,
"is_crosspostable": false,
"pinned": false,
"over_18": false,
"all_awardings": [],
"awarders": [],
"media_only": false,
"can_gild": false,
"spoiler": false,
"locked": false,
"author_flair_text": null,
"treatment_tags": [],
"visited": false,
"removed_by": null,
"num_reports": null,
"distinguished": null,
"subreddit_id": "t5_2qh23",
"author_is_blocked": false,
"mod_reason_by": null,
"removal_reason": null,
"link_flair_background_color": "",
"id": "1fg4o4m",
"is_robot_indexable": true,
"report_reasons": null,
"author": "LucidDream9590",
"discussion_type": null,
"num_comments": 0,
"send_replies": true,
"whitelist_status": "all_ads",
"contest_mode": false,
"mod_reports": [],
"author_patreon_flair": false,
"author_flair_text_color": null,
"permalink": "/r/test/comments/1fg4o4m/test/",
"parent_whitelist_status": "all_ads",
"stickied": false,
"url": "https://www.reddit.com/r/test/comments/1fg4o4m/test/",
"subreddit_subscribers": 23859,
"created_utc": 1726259389,
"num_crossposts": 0,
"media": null,
"is_video": false
```

