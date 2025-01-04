# Create the virtual environment

```
python3 -m venv venv
```

# Activate the virtual environment

```
source venv/bin/activate
```

# Install the required packages

```
pip install geopandas folium matplotlib fluvio
```

# Run the script

```
python3 plot.py
```

Open browser "nyc_taxi_zones_map.html" to see the map.

On mac:
```
open nyc_taxi_zones_map.html
```

The map will continously update with the latest data from the Fluvio topic every 30 seconds.