# Youtube Stats Connector
In this example, there is a connector that will produce entries that lists a youtuber's stats and most recent videos. Based off of (https://github.com/ArnavK-09/youtube_live_stats)[https://github.com/ArnavK-09/youtube_live_stats]. The SDF used to generate entries is converted into a connector.

## Prerequisites
This example runs only on fluvios cdk with `infinyon/jolt@0.4.1` and `infinyon-http-source-0.3.8`. The packages will be automatically installed for you in the script.

## Usage
```
./generate_connector.sh <channel> <key> [<consume = true?>]
```
Please enter a `channel` and the api key you have for youtube. A third param could be used so that the consume is automatically running after the connector is started. A connector file is generated for you in the `conn` folder.

## Full Body of the response
The following is the fullbody of the response from youtube. Jolt can be used to reduce the amount of data that is potentially useless. 

```
{
  "kind": "youtube#channelListResponse",
  "etag": "dTsADNLpCouI9PnyvcaqHlA0je4",
  "pageInfo": {
    "totalResults": 1,
    "resultsPerPage": 1
  },
  "items": [
    {
      "kind": "youtube#channel",
      "etag": "R-EoSs-XS-P8w8gJZnz0oNGCpUY",
      "id": "UCPNgldkaHwl_8-M_7ez7XPw",
      "snippet": {
        "title": "Tsuchi",
        "description": "I like driving old Japanese cars that barely run on the private mountain I occasionally have a permit for\n",
        "customUrl": "@tsuchifd",
        "publishedAt": "2012-11-09T05:42:45Z",
        "thumbnails": {
          "default": {
            "url": "https://yt3.ggpht.com/gJ5bSd0YMKVRFrsV0k5_4BSuN3KZ8xpfVSBQ6R0nMNhzbdN8wONwH0Uh7yIacUM_FPG5eBqjkng=s88-c-k-c0x00ffffff-no-rj",
            "width": 88,
            "height": 88
          },
          "medium": {
            "url": "https://yt3.ggpht.com/gJ5bSd0YMKVRFrsV0k5_4BSuN3KZ8xpfVSBQ6R0nMNhzbdN8wONwH0Uh7yIacUM_FPG5eBqjkng=s240-c-k-c0x00ffffff-no-rj",
            "width": 240,
            "height": 240
          },
          "high": {
            "url": "https://yt3.ggpht.com/gJ5bSd0YMKVRFrsV0k5_4BSuN3KZ8xpfVSBQ6R0nMNhzbdN8wONwH0Uh7yIacUM_FPG5eBqjkng=s800-c-k-c0x00ffffff-no-rj",
            "width": 800,
            "height": 800
          }
        },
        "localized": {
          "title": "Tsuchi",
          "description": "I like driving old Japanese cars that barely run on the private mountain I occasionally have a permit for\n"
        },
        "country": "US"
      },
      "statistics": {
        "viewCount": "13690144",
        "subscriberCount": "161000",
        "hiddenSubscriberCount": false,
        "videoCount": "382"
      }
    }
  ]
}
```
