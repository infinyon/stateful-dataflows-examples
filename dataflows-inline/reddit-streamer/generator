#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <subreddit>"
  exit 1
fi

SUBREDDIT=$1

mkdir -p conn
cat > conn/reddit-connector-$SUBREDDIT.yaml <<EOL
apiVersion: 0.1.0
meta:
  version: 0.3.8
  name: reddit-${SUBREDDIT}
  type: http-source
  topic: reddit-sub-posts
  producer: 
    batch-size: 10000
http:
  method: GET
  endpoint: https://www.reddit.com/r/${SUBREDDIT}/new.json
  interval: 5s
transforms:
  - uses: infinyon/jolt@0.4.1
    with:
      spec:
        - operation: shift
          spec:
            data:
              children:
                "0": 
                  data:
                    subreddit: "subreddit"
                    title: "title"
                    author: "author"
                    selftext: "selftext"
                    ups: "ups"
                    permalink: "permalink"
                    id: "id"
EOL

echo "Connector file generated: reddit-connector-$SUBREDDIT.yaml"
echo "++++++++++++++"
echo "Making sure that http-source is installed"
if [ -e "infinyon-http-source-0.3.8.ipkg" ]; then
    echo "Package Found."
else
    echo "Package Not Found. Installing..."
    cdk hub download infinyon/http-source@0.3.8
fi
echo "++++++++++++++"
echo "Downloading jolt"
fluvio hub sm download infinyon/jolt@0.4.1
echo "++++++++++++++"
echo "Starting connector"
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg --config conn/reddit-connector-"$SUBREDDIT".yaml



