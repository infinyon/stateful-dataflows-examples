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
        pu_zone = int(tip.get("pu_zone"))
        avg_tip = tip.get("avg_tip")
        tips_dict[pu_zone] = avg_tip
        print(f"Zone: {pu_zone}, Avg Tip: {avg_tip}")


    # Create a Folium map centered on NYC
    nyc_map = folium.Map(location=[40.7128, -74.0060], zoom_start=11)

    # Add Taxi Zones to the map
    for idx, zone in taxi_zones.iterrows():
        
        # Extract geometry and tooltip information
        tip_info = tips_dict.get(idx, 0.0)

        fill_opacity = 0.0
        
        
        if tip_info > 0.0:
            print("tip_info: ", zone["zone"],tip_info)
            #fill_opacity = min(max(tip_info / 10.0, 0.0), 1.0)
            #print("fill_opacity: ", fill_opacity)
            fill_opacity = 1.0


        geo_json = folium.GeoJson(
            data=zone["geometry"],
            style_function=lambda x: {
                "fillColor": "blue",
                "color": "black",
                "weight": 2.0,
                "fillOpacity": fill_opacity
            },
            tooltip=folium.Tooltip(zone["zone"]),
        )
        # print("zone: ", zone["zone"])
        geo_json.add_to(nyc_map)

    # Save and display the map
    nyc_map.save("nyc_taxi_zones_map.html")


