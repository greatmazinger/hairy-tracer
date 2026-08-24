import sys
import os
import json

sys.path.insert(0, os.path.abspath('src'))
from btrace import BTracer
from scenes import readScene_3spheres

def get_data():
    tracer = BTracer(size=(10, 10), testflag=False)
    tracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                        distance = 10.0,
                        vpwidth = 6.40 * 0.9,
                        vpheight = 4.80 * 0.9 )
    readScene_3spheres(world=tracer.world)
    return tracer.get_data()

data1 = get_data()
data2 = get_data()

if data1 == data2:
    print("Exact match!")
else:
    for i, (p1, p2) in enumerate(zip(data1, data2)):
        if p1 != p2:
            print(f"Diff at pixel {i}: {p1} vs {p2}")
