import folium
import geohash2
import pandas as pd


def process_data(latest_file_name):
    data = pd.read_csv(latest_file_name)
    data['lat'] = data['geohash'].apply(lambda x: geohash2.decode(x)[0])
    data['lon'] = data['geohash'].apply(lambda x: geohash2.decode(x)[1])

    # Get the maximum speed in the dataset
    max_speed = data['avg_speed'].max()

    # Create a map centered on the first geohash in the dataset
    map = folium.Map(location=[data['lat'][0], data['lon'][0]], zoom_start=10)

    for index, row in data.iterrows():
        # get the color of the marker based on the average speed
        color = (row['avg_speed'] / max_speed) * 255
        color = 'rgb(255, {}, 0)'.format(int(255 - color)) # set color based on average_speed

        folium.CircleMarker(location=[row['lat'], row['lon']], radius=15, color=color, fill=True, fill_color=color).add_to(map)
    
    map.save('templates/map.html')
