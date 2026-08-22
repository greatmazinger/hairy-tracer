import numpy
import pytest

from rvmath.geometry import sphere, plane, ray
from rvmath.geometry.utils import CalcReflectionVector, CalcRefractionVector, fequal
from rvlight.illuminationmodel import PhongIlluminationModel

class TestPlaneFunctions:
    def setup_method(self):
        # Create a basic plane at z=0, facing +z
        self.pnorm = numpy.array([0.0, 0.0, 1.0])
        self.plane = plane.Plane(pnormal=self.pnorm, distance=0.0)

    def test_line_plane_intersection_perpendicular(self):
        # Ray pointing straight down at the plane
        r = ray.Ray(orig=numpy.array([0.0, 0.0, 5.0]), dir=numpy.array([0.0, 0.0, -1.0]))
        (intpoint, viewvec, un, t) = self.plane.findIntersection(r)
        assert intpoint is not None
        assert t == 5.0
        assert numpy.allclose(intpoint, [0.0, 0.0, 0.0])
        assert numpy.allclose(un, self.pnorm)

    def test_line_plane_intersection_parallel(self):
        # Ray parallel to the plane (moving along x-axis)
        r = ray.Ray(orig=numpy.array([0.0, 0.0, 5.0]), dir=numpy.array([1.0, 0.0, 0.0]))
        (intpoint, viewvec, un, t) = self.plane.findIntersection(r)
        assert intpoint is None


class TestSphereFunctions:
    def setup_method(self):
        # Create a unit sphere at the origin
        self.sphere = sphere.Sphere(center=numpy.array([0.0, 0.0, 0.0]), radius=1.0)

    def test_line_sphere_no_intersection(self):
        # Ray far away from the sphere
        r = ray.Ray(orig=numpy.array([0.0, 5.0, 5.0]), dir=numpy.array([0.0, 0.0, -1.0]))
        (intpoint, viewvec, un, t) = self.sphere.findIntersection(r)
        assert intpoint is None

    def test_line_sphere_intersects_center(self):
        # Ray pointing straight at the center
        r = ray.Ray(orig=numpy.array([0.0, 0.0, 5.0]), dir=numpy.array([0.0, 0.0, -1.0]))
        (intpoint, viewvec, un, t) = self.sphere.findIntersection(r)
        assert intpoint is not None
        assert t == 4.0 # 5.0 down to radius 1.0
        assert numpy.allclose(intpoint, [0.0, 0.0, 1.0])


class TestGeometryUtils:
    def test_reflection(self):
        # CalcReflectionVector expects invec to point OUT from the surface
        invec = numpy.array([-0.70710678, 0.70710678, 0.0]) # Pointing up-left
        normal = numpy.array([0.0, 1.0, 0.0]) # Pointing up
        rvec = CalcReflectionVector(invec=invec, normalvec=normal)
        # Should reflect to pointing up-right
        assert numpy.allclose(rvec, [0.70710678, 0.70710678, 0.0])

    def test_refraction(self):
        # Entering glass (IOR 1.5) from straight above
        invec = numpy.array([0.0, -1.0, 0.0]) 
        normal = numpy.array([0.0, 1.0, 0.0])
        refract_vec = CalcRefractionVector(invec=invec, normalvec=normal, ior=1.5)
        # Should continue straight down but slower
        assert refract_vec is not None
        assert numpy.allclose(refract_vec / numpy.linalg.norm(refract_vec), [0.0, -1.0, 0.0])
