import sys
import os
import hashlib
import json

sys.path.insert(0, os.path.abspath('src'))
from btrace import BTracer
from scenes import readScene_3spheres

def get_hash():
    tracer = BTracer(size=(10, 10), testflag=False)
    tracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                        distance = 10.0,
                        vpwidth = 6.40 * 0.9,
                        vpheight = 4.80 * 0.9 )
    readScene_3spheres(world=tracer.world)
    data = tracer.get_data()
    return hashlib.md5(json.dumps(data).encode()).hexdigest()

print("Hash 1:", get_hash())
print("Hash 2:", get_hash())
