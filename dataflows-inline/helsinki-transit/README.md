Demo of SSDK with helsinki metro MQTT



# Start MQTT connector using cloud

## Download Jolt SmartModule

```
$ fluvio hub smartmodule download infinyon/jolt@0.3.0
```

## Start MQTT connector

Then start the MQTT connector using the following command:

```
fluvio cloud connector create --config mqtt-helsinki.yaml
```

You should see the following output:

```
$ fluvio consume helsinki | jq

{
  "vehicle": 456,
  "tst": "2024-03-19T02:28:08.028Z",
  "speed": 3.96,
  "lat": 60.197156,
  "long": 24.718909,
  "route": "2235N"
}
{
  "vehicle": 1423,
  "tst": "2024-03-19T02:28:08.149Z",
  "speed": 0,
  "lat": 60.20017,
  "long": 24.685558,
  "route": "2134N"
}
{
  "vehicle": 1828,
  "tst": "2024-03-19T02:28:08.181Z",
  "speed": 0.15,
  "lat": 60.178577,
  "long": 24.950038,
  "route": "4600"
}

```

Read top 5 bus speed: 
```
$ fluvio consume top-vehicle | jq
```
