import numpy
import logging
from numpy import dot

import ray
from utils import *
from rvlight import *

logger = None

class World():
    def __init__( self,
                  objects = [],
                  lights = [],
                  myname = "world",
                  logger_name = None ):
        if logger_name != None:
            self.setup_logger( logger_name = logger_name )
        self.objects = objects
        self.lights = lights
        self.myname = myname

    def addObject( self, obj ):
        self.objects.append( obj )

    def addLight( self, mylight ):
        self.lights.append( mylight )

    # TODO TODO TODO
    # May need to refactor loop code.
    # TODO TODO TODO
    def findIntersectionAndColor( self,
                                  myray = None,
                                  srcobject = None ):
        objhit = None
        currhit = None
        currnorm = None
        currcolor = None
        distance = None
        for item in self.objects:
            if item == srcobject:
                continue
            (hitpoint, hitcolor) = \
                    item.findIntersectionAndColor( dray = myray,
                                                   lightlist = self.lights )
            if hitpoint != None:
                newdistance = numpy.linalg.norm( hitpoint - myray.getOrigin() )
                if ((currhit == None) or
                    (newdistance < distance)):
                    currhit = hitpoint.copy()
                    currnorm = numpy.linalg.norm( hitpoint )
                    currcolor = hitcolor
                    distance = newdistance
                    objhit = item
        return (objhit, currhit, currcolor)

    def doesIntersect( self,
                       srcobject = None,
                       lightray = None,
                       intpoint = None ):
        global logger
        assert( srcobject != None and
                lightray != None and
                intpoint != None )
        for item in self.objects:
            if item == srcobject:
                continue
            # TODO Origin numpy.array constant would be a good idea.
            tmpray = ray.Ray( orig = intpoint,
                              dir = lightray )
            (hitpoint, viewvec, un, t) = \
                    item.findIntersection( dray = tmpray )
            if hitpoint != None and t > 0.0:
                # logger.debug( "%s hits %s" %
                #               (srcobject.myname, item.myname) )
                return True
        return False

    def setup_logger( self, logger_name = None ):
        global logger
        # Set up main logger
        logger = logging.getLogger( logger_name )


if __name__ == "__main__":
    print "Basic World class test:"
    print "Done."
