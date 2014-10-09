import numpy
from rvmath.geometry import ray

class Light():
    def __init__( self,
                  orig = None,
                  color = None ):
        # The vectors can be of any numeric type
        # but for this to make sense, floats should be used.
        self.origin = numpy.copy( orig )
        if color == None:
            color = numpy.array( [1.0, 1.0, 1.0] )
        self.color = numpy.copy( color )

    def GetOrigin( self ):
        return self.origin

    def GetColor( self ):
        return self.color

class DirectedLight( Light ):
    def __init__( self,
                  orig = None,
                  dir = None ):
        Light.__init__( orig )
        self.direction = numpy.copy( dir )

    def GetDirection( self ):
        return self.direction


class ConeLight( DirectedLight ):
    def __init__( self,
                  orig = None,
                  dir = None,
                  angle = None ):
        DirectedLight.__init__( orig, dir )
        self.angle = numpy.copy( angle )

if __name__ == "__main__":
    print "Basic Light class test:"
    mylight = Light( orig = numpy.array( [ 0, 1, 2 ] ) )
    print "Done"
