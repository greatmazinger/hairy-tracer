import json
import numpy
from rvmath.geometry import mesh, triangle, ray
from rvlight.light import Light
from rvlight.illuminationmodel import PhongIlluminationModel
import obj_parser

# Mock world
class DummyWorld:
    def doesIntersect(self, srcobject=None, lightray=None, intpoint=None):
        return False

world = DummyWorld()
mat = PhongIlluminationModel(kAmbient=0.1, kDiffuse=numpy.array([0.1, 0.8, 0.3]), kSpecular=0.8, nS=50.0)

tris = obj_parser.load_obj("models/pyramid.obj")
triangle_objects = []
for (v0, v1, v2) in tris:
    triangle_objects.append(triangle.Triangle(v0=v0, v1=v1, v2=v2, ill_model=mat))

m = mesh.Mesh(triangles=triangle_objects, ill_model=mat, world_intfn=world.doesIntersect)

l1 = Light(orig=numpy.array([15.0, 20.0, 15.0]), color=numpy.array([250, 250, 250]))
l2 = Light(orig=numpy.array([-15.0, 10.0, -10.0]), color=numpy.array([100, 10, 15]))
lightlist = [l1, l2]

# Fire a ray from camera to the pyramid
# Camera is at (10, 15, 10), looking at (0, 2, 0)
cam_origin = numpy.array([10.0, 15.0, 10.0])
# Let's fire a ray exactly at the center of the pyramid face
target = numpy.array([1.0, -1.0, 1.0])
r = ray.Ray(orig=cam_origin, dir=target - cam_origin)

hit = m.findIntersection(dray=r)
print("Hit details:", hit)
if hit[0] is not None:
    intpoint, viewvec, un, t = hit
    print("Normal (un):", un)
    print("Viewvec:", viewvec)
    for i, l in enumerate(lightlist):
        lightvec = l.GetOrigin() - intpoint
        lightvec = lightvec / numpy.linalg.norm(lightvec)
        ldv = numpy.dot(lightvec, un)
        print(f"Light {i} ldv:", ldv)
        print(f"Light {i} lightvec:", lightvec)
    color = m.GetColor(intpoint=intpoint, lightlist=lightlist, viewvec=viewvec, normal=un)
    print("Final color:", color)
