import unittest
import numpy
    
import geometry

class TestPlaneFunctions( unittest.TestCase ):
    def setUp(self):
        # create plane
        pnorm = None
        # self.myplane = geometry.Plane( pnorm, distance )

    def test_line_plane_intersections( self ):
        # case 1 - parallel, no intersection
        # case 2 - perpendicular, intersects
        # case 3 - at an angle, intersects
        pass

class TestSphereFunctions( unittest.TestCase ):
    def setUp(self):
        # create unit radius sphere
        self.usphere = range(10)

    def test_line_sphere_intersections( self ):
        # case 1 - no intersection, far from surface
        # case 2 - intersects the center
        # case 3 - intersects surface
        # case 4 - intersects close to surface
        # case 5 - no intersection, close to surface
        pass

if __name__ == '__main__':
    unittest.main()
