import folium
import geohash
import pandas as pd
from branca.colormap import linear

def get_rectangles_bounds(gh):
    decoded = geohash.bbox(gh)
    if decoded is None:
        return None
    W = decoded['w']
    E = decoded['e']
    N = decoded['n']
    S = decoded['s']

    return [[S, W], [N, E]]


def process_data(data: pd.DataFrame, time: str):
    # map from red to green based on the avg_speed field
    cmap = linear.RdYlGn_09.scale(0, 60)
    # add color field to the data
    data['color'] = data['avg_speed'].apply(lambda x: cmap(x))
    data['lat'] = data['geohash'].apply(lambda x: geohash.decode(x)[0])
    data['lon'] = data['geohash'].apply(lambda x: geohash.decode(x)[1])

    data = data[data['time'] == time]
    data.reset_index(inplace=True) # reset index to avoid problems (like index 0 not existing)

    # Create a map centered on the first geohash in the dataset
    #map = folium.Map(location=[data['lat'][0], data['lon'][0]], zoom_start=10)
    map = folium.Map(location=[22.59, 114.11], zoom_start=10)

    for _, row in data.iterrows():
        bounds = get_rectangles_bounds(row['geohash'])
        if bounds is None:
            continue

        folium.Rectangle(bounds=bounds, color=row['color'], fill=True, fill_color=row['color'], popup="%.2f" % round(row['avg_speed'], 2)).add_to(map)
    
    map.save('templates/map.html')
