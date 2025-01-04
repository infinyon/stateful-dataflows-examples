import geopandas as gpd
import folium
from fluvio import Fluvio, Offset
import json


def process_tips_and_generate_map(tips):

    nyc_map = folium.Map(location=[40.7128, -74.0060], zoom_start=11)
    nyc_map.get_root().html.add_child(folium.Element('''
        <script type="text/javascript">
            setTimeout(function(){
                location.reload(true);
            }, 5000);
        </script>
    '''))

    tips_count = len(tips)
    nyc_map.get_root().html.add_child(folium.Element('<h1>NYC Taxi Zones count: {}</h1>'.format(tips_count)))

    for idx, zone in taxi_zones.iterrows():
        tip_info = tips.get(idx, 0.0)
        fill_opacity = 0.0
        if tip_info > 0.0:
            print("tip_info: ", zone["zone"], tip_info)
            fill_opacity = min(max(tip_info / 10.0, 0.0), 1.0)
            print("fill_opacity: ", fill_opacity)

        geo_json = folium.GeoJson(
            data=zone["geometry"],
            style_function=lambda x, fill_opacity=fill_opacity: {
                "fillColor": "blue",
                "color": "black",
                "weight": 2.0,
                "fillOpacity": fill_opacity
            },
            tooltip=folium.Tooltip(zone["zone"] + " " + str(tip_info)),
        )
        geo_json.add_to(nyc_map)
    nyc_map.save("nyc_taxi_zones_map.html")




# Load NYC Taxi Zone shapefile
taxi_zones = gpd.read_file("zones")
# Ensure CRS is set correctly (e.g., WGS84 for Folium)
taxi_zones = taxi_zones.to_crs("EPSG:4326")

# generate empty map
process_tips_and_generate_map({})

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


    process_tips_and_generate_map(tips_dict)

