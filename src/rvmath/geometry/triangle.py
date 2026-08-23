import math
import numpy
from numpy import dot, cross

from . import ray
from rvmath.geometry.utils import *

class Triangle():
    def __init__( self,
                  v0 = None,
                  v1 = None,
                  v2 = None,
                  ill_model = None,
                  world_intfn = None,
                  myname = "triangle",
                  is_reflector = False,
                  is_refractor = False ):
        self.v0 = numpy.copy(v0)
        self.v1 = numpy.copy(v1)
        self.v2 = numpy.copy(v2)
        self.ill_model = ill_model
        self.wintfn = world_intfn
        self.myname = myname
        self.reflector_flag = is_reflector
        self.refractor_flag = is_refractor
        
        # Precompute edges
        self.edge1 = self.v1 - self.v0
        self.edge2 = self.v2 - self.v0
        
        # Precompute normal
        normal = cross(self.edge1, self.edge2)
        normal_len = numpy.linalg.norm(normal)
        if normal_len > 0:
            self.normal = normal / normal_len
        else:
            self.normal = numpy.array([0.0, 1.0, 0.0]) # Fallback for degenerate triangle

    def getUnitNormal( self, point = None ):
        return self.normal

    def findIntersection( self, dray = None ):
        Rd = dray.getDirection()
        Rd = Rd / numpy.linalg.norm( Rd )
        Og = dray.getOrigin()
        
        # Möller–Trumbore intersection algorithm
        epsilon = 1e-6
        h = cross(Rd, self.edge2)
        a = dot(self.edge1, h)
        
        # Ray is parallel to the triangle
        if -epsilon < a < epsilon:
            return (None, None, None, None)
            
        f = 1.0 / a
        s = Og - self.v0
        u = f * dot(s, h)
        
        if u < 0.0 or u > 1.0:
            return (None, None, None, None)
            
        q = cross(s, self.edge1)
        v = f * dot(Rd, q)
        
        if v < 0.0 or u + v > 1.0:
            return (None, None, None, None)
            
        # At this stage we can compute t to find out where the intersection point is on the line.
        t = f * dot(self.edge2, q)
        
        # Ray intersection
        if t > 0.01:
            intpoint = Og + (t * Rd)
            un = self.getUnitNormal()
            
            # If we hit the back of the triangle, flip the normal
            if dot(Rd, un) > 0:
                un = -1 * un
                
            viewvec = numpy.copy( dray.getUnitVector() )
            viewvec = -1 * viewvec
            return (intpoint, viewvec, un, t)
            
        return (None, None, None, None)

    def findIntersectionAndColor( self,
                                  dray = None,
                                  lightlist = None ):
        (intpoint, viewvec, un, t) = self.findIntersection( dray = dray )
        if intpoint is None:
            return (None, None)
        assert( viewvec is not None and un is not None )
        return ( intpoint,
                 self.GetColor( intpoint = intpoint,
                                lightlist = lightlist,
                                viewvec = viewvec,
                                normal = un ) )

    def GetColor( self,
                  intpoint = None,
                  lightlist = [],
                  viewvec = None,
                  normal = None ):
        amblight = numpy.array([15, 75, 255])
        color = numpy.array( [0.0, 0.0, 0.0] )
        
        for l in lightlist:
            light = l.GetColor()
            lightvec =  l.GetOrigin() - intpoint
            lightvec = lightvec / numpy.linalg.norm( lightvec )
            lightflag = True
            
            # Shadow check
            if self.wintfn( srcobject = self,
                            lightray = lightvec,
                            intpoint = intpoint ):
                lightflag = False
                
            color = color + self.ill_model.GetColor( amblight = amblight,
                                                     light = light,
                                                     lightvec = lightvec,
                                                     normalvec = normal,
                                                     viewvec = viewvec,
                                                     lightflag = lightflag )
        return [ max(0, min( 255, x )) for x in color ]

    def IsReflector( self ):
        return self.reflector_flag

    def IsRefractor( self ):
        return self.refractor_flag
