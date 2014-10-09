import numpy

class Ray():
    def __init__( self,
                  orig = None,
                  dir = None ):
        # The vectors can be of any numeric type
        # but for this to make sense, floats should be used.
        assert( orig != None and dir != None )
        self.origin = numpy.copy( orig )
        self.direction = numpy.copy( dir )
        self.direction = self.direction / numpy.linalg.norm( dir )
        try:
            uv = dir - orig
        except:
            print dir, type(dir)
            print orig, type(orig)
            assert(0)
        self.unitvector = uv / numpy.linalg.norm( uv )

    def getOrigin( self ):
        return self.origin

    def getDirection( self ):
        return self.direction

    def getUnitVector( self ):
        return self.unitvector

if __name__ == "__main__":
    print "Basic Ray class test:"
    myray = Ray( orig = numpy.array( [ 0, 1, 2 ] ),
                 dir = numpy.array( [ 3, 4, 5 ] ) )
    print "Done"
