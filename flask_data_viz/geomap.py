import folium
import geohash
import pandas as pd


def process_data(latest_file_name):
    data = pd.read_csv(latest_file_name)
    data['lat'] = data['geohash'].apply(lambda x: geohash.decode(x)[0])
    data['lon'] = data['geohash'].apply(lambda x: geohash.decode(x)[1])

    # Get the maximum speed in the dataset
    max_speed = data['avg_speed'].max()

    # Create a map centered on the first geohash in the dataset
    map = folium.Map(location=[data['lat'][0], data['lon'][0]], zoom_start=10)

    for _, row in data.iterrows():
        # get the color of the marker based on the average speed
        color = (row['avg_speed'] / max_speed) * 255
        color = f'rgb({255 - color}, {color}, 0)'

        decoded = geohash.bbox(row['geohash'])
        W = decoded['w']
        E = decoded['e']
        N = decoded['n']
        S = decoded['s']

        folium.Rectangle(bounds=[[S, W], [N, E]], color=color, fill=True, fill_color=color, popup=row['geohash']).add_to(map)
    
    map.save('templates/map.html')
