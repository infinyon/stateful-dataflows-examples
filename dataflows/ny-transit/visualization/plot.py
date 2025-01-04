import geopandas as gpd
import folium
from fluvio import Fluvio, Offset
import json

# Load NYC Taxi Zone shapefile

taxi_zones = gpd.read_file("zones")

# Ensure CRS is set correctly (e.g., WGS84 for Folium)
taxi_zones = taxi_zones.to_crs("EPSG:4326")

TOPIC_NAME = "pu-tips"

fluvio = Fluvio.connect()

consumer = fluvio.partition_consumer(TOPIC_NAME, 0)
for idx, record in enumerate( consumer.stream(Offset.from_end(0)) ):
    tips = json.loads(record.value_string())

    tips_dict = {}
    for tip in tips:
        pu_zone = tip.get("pu_zone")
        avg_tip = tip.get("avg_tip")
        tips_dict[pu_zone] = avg_tip
        print(f"Zone: {pu_zone}, Avg Tip: {avg_tip}")


    # Create a Folium map centered on NYC
    nyc_map = folium.Map(location=[40.7128, -74.0060], zoom_start=11)

    # Add Taxi Zones to the map
    for _, zone in taxi_zones.iterrows():
        print("zone: ", zone["Name"])
        # Extract geometry and tooltip information
        geo_json = folium.GeoJson(
            data=zone["geometry"],
            style_function=lambda x: {
                "fillColor": "blue",
                "color": "black",
                "weight": 2.0,
                "fillOpacity": 0.0
            },
            tooltip=folium.Tooltip(zone["zone"]),
        )
        # print("zone: ", zone["zone"])
        geo_json.add_to(nyc_map)

    # Save and display the map
    nyc_map.save("nyc_taxi_zones_map.html")


