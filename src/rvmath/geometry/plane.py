import numpy
from numpy import dot

import ray
from rvmath.geometry.utils import *
from rvlight.illuminationmodel import *

# The plane's definition is:
#   A*x + B*y + C*z + D = 0
# where:
#   Pnormal (plane normal) is
#      [ A B C ]
#   distance from origin to the plane is D.
#     - the sign of D determines which side of the 
#       plane the system origin is located.
class Plane():
    def __init__( self,
                  pnormal = None,
                  distance = None,
                  ill_model = None,
                  world_intfn = None,
                  myname = "plane",
                  is_reflector = False,
                  is_refractor = False,
                  logger = None ):
        self.logger = logger
        # The vectors can be of any numeric type
        # but for this to make sense, floats should be used.
        # Distance should be a scalar.
        self.normal = pnormal / numpy.linalg.norm( pnormal )
        self.distance = distance
        self.ill_model = ill_model
        self.wintfn = world_intfn
        self.myname = myname
        self.reflector_flag = False
        self.refractor_flag = False

    def getNormal( self ):
        return self.normal

    def getDistance( self ):
        return self.distance

    def findIntersection( self,
                          dray = None ):
        global logger
        Rd = dray.getDirection()
        Rd = Rd / numpy.linalg.norm( Rd )
        # Rd = -1 * Rd
        vd = dot( self.normal, Rd )
        Np = self.normal
        vd2 = Np[0]*Rd[0] + Np[1]*Rd[1] + Np[2]*Rd[2]
        # assert( fequal( vd, vd2 ) )
        if fequal( vd, 0.0 ):
            return (None, None, None, None)
        Rorig = dray.getOrigin()
        v0 = -1 * (dot( self.normal, Rorig ) + self.distance )
        # v0 = (dot( self.normal, Rorig ) + self.distance )
        t = v0 / vd
        if t < 0:
            intpoint = Rorig + (t * Rd)
            # logger.debug( "Rd: %s  Ro: %s NOINT[ %s ]" %
            #               (str(Rd), str(Rorig), str(intpoint)) )
            return (None, None, None, None)
        else:
            intpoint = Rorig + (t * Rd)
            viewvec = numpy.copy( dray.getUnitVector() )
            # logger.debug( "Rd: %s  Ro: %s int: %s view: %s" %
            #               (str(Rd), str(Rorig), str(intpoint), str(viewvec)) )
            return (intpoint, viewvec, self.normal, t)
        
    def findIntersectionAndColor( self,
                                  dray = None,
                                  lightlist = None ):
        (intpoint, viewvec, pn, t) = self.findIntersection( dray = dray )
        if intpoint == None:
            return (None, None)
        flag = numpy.isnan( intpoint )
        if ( flag[0] or
             flag[1] or
             flag[2] ):
            return (None, None)
        return ( intpoint,
                 self.GetColor( intpoint = intpoint,
                                lightlist = lightlist,
                                viewvec = viewvec ) )

    def GetColor( self,
                  intpoint = None,
                  lightlist = [],
                  viewvec = None ):
        # TODO
        # amblight and light should be parameters to Plane
        # amblight = numpy.array([15, 15, 75])
        amblight = numpy.array([0, 0, 0])
        color = numpy.array( [0.0, 0.0, 0.0] )
        for l in lightlist:
            light = l.GetColor()
            lightvec = l.GetOrigin() - intpoint
            lightvec = lightvec / numpy.linalg.norm( lightvec )
            normalvec = self.normal / numpy.linalg.norm( self.normal )
            lightflag = True
            if self.wintfn( srcobject = self,
                            lightray = lightvec,
                            intpoint = intpoint ):
                # It intersects something.
                # Light does not affect this object.
                lightflag = False
            color = color + self.ill_model.GetColor( amblight = amblight,
                                                     light = light,
                                                     lightvec = lightvec,
                                                     normalvec = normalvec,
                                                     viewvec = viewvec,
                                                     lightflag = lightflag )
        return [ max(0, min( 255, x )) for x in color ]

    def IsReflector( self ):
        return self.reflector_flag

    def IsRefractor( self ):
        return self.refractor_flag

    def getYintercept( self ):
        return -1 * self.distance / self.normal[1]

class CheckeredPlane( Plane ):
    def __init__( self,
                  pnormal = None,
                  distance = None,
                  ill_model1 = None,
                  ill_model2 = None,
                  world_intfn = None,
                  myname = "plane",
                  logger = None
                ):
        Plane.__init__( self,
                        pnormal = pnormal,
                        distance = distance,
                        ill_model = ill_model1,
                        world_intfn = world_intfn,
                        myname = myname,
                        logger = logger )
        self.ill_model2 = ill_model2

    def GetColor( self,
                  intpoint = None,
                  lightlist = [],
                  viewvec = None ):
        # simplified
        # TODO need to actually rotate the plane
        # currently we are assuming that we only need to look at (x,y) to
        # generate the texture
        light = numpy.array( [175, 175, 235] )
        if ( ( (int( round( intpoint[0] * 1.0 ) ) % 2) == 1 and
               (int( round( intpoint[2] * 1.0 ) ) % 2) == 0 ) or
             ( (int( round( intpoint[0] * 1.0 ) ) % 2) == 0 and
               (int( round( intpoint[2] * 1.0 ) ) % 2) == 1 ) ):
            amblight = numpy.array([10, 10, 250])
            im = self.ill_model
        else:
            amblight = numpy.array([150, 10, 10])
            im = self.ill_model2
        color = numpy.array( [0.0, 0.0, 0.0] )
        for l in lightlist:
            lightvec = l.GetOrigin() - intpoint
            lightvec = lightvec / numpy.linalg.norm( lightvec )
            normalvec = self.normal / numpy.linalg.norm( self.normal )
            lightflag = True
            if self.wintfn( srcobject = self,
                            lightray = lightvec,
                            intpoint = intpoint ):
                # It intersects something.
                # Light does not affect this object.
                lightflag = False
            color = color + im.GetColor( amblight = amblight,
                                         light = light,
                                         lightvec = lightvec,
                                         normalvec = normalvec,
                                         lightflag = lightflag,
                                         viewvec = viewvec )
        return [ max(0, min( 255, x )) for x in color ]

if __name__ == "__main__":
    print "Basic Plane class test:"
    myplane = Plane( numpy.array( [ 1., 5., 0. ] ),
                     -3. )
    rOrig = numpy.array([ 1., 1., 0. ])
    rDir = numpy.array([ 0., 1., 0. ])
    xtn = myplane.findIntersection( ray.Ray( rOrig, rDir ) )
    print "Intersection: ", xtn
