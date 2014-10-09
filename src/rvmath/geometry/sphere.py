import math
import logging
import numpy
from numpy import dot

import ray
from rvmath.geometry.utils import *

# The sphere's definition is:
#   x^2 + y^2 = r^2
class Sphere():
    def __init__( self,
                  center = None,
                  radius = None,
                  ill_model = None,
                  world_intfn = None,
                  myname = "sphere",
                  is_reflector = False,
                  is_refractor = False,
                  logger = None ):
        self.logger = logger
        self.radius = radius
        self.center = numpy.copy( center )
        self.ill_model = ill_model
        self.wintfn = world_intfn
        self.myname = myname
        # Precalculate some terms
        self.r2 = radius * radius
        self.reflector_flag = is_reflector
        self.refractor_flag = is_refractor

    def getRadius( self ):
        return self.radius

    def getUnitNormal( self,
                       point ):
        Ce = self.center
        sr = self.radius
        return numpy.array( [ (point[0] - Ce[0])/sr,
                              (point[1] - Ce[1])/sr,
                              (point[2] - Ce[2])/sr ] )

    def findIntersection( self,
                          dray = None ):
        Rd = dray.getDirection()
        Rd = Rd / numpy.linalg.norm( Rd )
        Og = dray.getOrigin()
        Ce = self.center
        xO_m_xC = Og[0] - Ce[0]
        yO_m_yC = Og[1] - Ce[1]
        zO_m_zC = Og[2] - Ce[2]
        B = 2 * (Rd[0] * xO_m_xC +
                 Rd[1] * yO_m_yC +
                 Rd[2] * zO_m_zC)
        C = (xO_m_xC * xO_m_xC +
             yO_m_yC * yO_m_yC +
             zO_m_zC * zO_m_zC) - self.r2 
        disc = B*B - 4*C
        if disc < 0:
            return (None, None, None, None)
        sq_disc = math.sqrt( disc )
        t = (-1 * B - sq_disc) * 0.5
        if t > 0.01:
            intpoint = Og + (t * Rd)
        else:
            t = (-1 * B + sq_disc) * 0.5
            intpoint = Og + (t * Rd)
        sr = self.radius
        # Calculate unit normal
        un = self.getUnitNormal( point = intpoint )
        # viewvec = -1 * Rd
        # viewvec = viewvec / numpy.linalg.norm( viewvec )
        viewvec = numpy.copy( dray.getUnitVector() )
        viewvec = -1 * viewvec
        # self.logger.debug( "Rd: %s  Ro: %s int: %s view: %s" %
        #              (str(Rd), str(Og), str(intpoint), str(viewvec)) )
        return (intpoint, viewvec, un, t)

    def findIntersectionAndColor( self,
                                  dray = None,
                                  lightlist = None ):
        (intpoint, viewvec, un, t) = self.findIntersection( dray = dray )
        # self.logger.debug( "t = %f intpoint => %s disc: %f" %
        #               (t, str(intpoint), sq_disc) )
        if intpoint == None:
            return (None, None)
        assert( viewvec != None and un != None )
        return ( intpoint, # intersection point
                 self.GetColor( intpoint = intpoint,
                                lightlist = lightlist,
                                viewvec = viewvec,
                                normal = un ) )

    def GetColor( self,
                  intpoint = None,
                  lightlist = [],
                  viewvec = None,
                  normal = None ):
        """Get combined color? Returns RGB in list form."""
        # TODO
        # amblight and light should be parameters to Sphere
        amblight = numpy.array([15, 75, 255])
        color = numpy.array( [0.0, 0.0, 0.0] )
        # We need:
        # 1. source object => self
        # 2. lightvec
        # 3. intersection point
        # -- for shadow testing
        for l in lightlist:
            light = l.GetColor()
            lightvec =  l.GetOrigin() - intpoint
            lightvec = lightvec / numpy.linalg.norm( lightvec )
            lightflag = True
            if self.wintfn( srcobject = self,
                            lightray = lightvec,
                            intpoint = intpoint ):
                # It intersects something.
                # Light does not affect this object.
                lightflag = False
            # lightflag = True
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

if __name__ == "__main__":
    print "Basic Sphere class test:"
    # TODO
    # Test code.
    # create sphere
    # xtn = mysphere.findIntersection( ray.Ray( rOrig, rDir ) )
    # print "Intersection: ", xtn
