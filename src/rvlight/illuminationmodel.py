import numpy
import logging
from math import pow
from rvmath.geometry import ray
from rvmath.geometry.utils import CalcReflectionVector, CalcHalfVector


class LocalIlluminationModel():
    def __init__( self,
                  kAmbient = 0.2,
                  kDiffuse = numpy.array( [0.0, 0.0, 0.7] ),
                  kSpecular = 0.6,
                  logger = None ):
        self._logger = logger
        self.kAmbient = kAmbient
        self.kDiffuse = numpy.copy( kDiffuse )
        self.kSpecular = kSpecular

    def GetAmbient( self ):
        pass

    def GetDiffuse( self ):
        pass

    def GetSpecular( self ):
        pass


class PhongIlluminationModel( LocalIlluminationModel ):
    def __init__( self,
                  kAmbient = 0.2,
                  kDiffuse = numpy.array( [0.0, 0.0, 0.7] ),
                  kSpecular = 0.6,
                  nS = 1.0,
                  useH = True,
                  logger = None ):
        # nS = specular reflection parameter
        #      1 = dull, 100 = shiny, infinite = perfect reflector
        LocalIlluminationModel.__init__( self,
                                         kAmbient,
                                         kDiffuse,
                                         kSpecular )
        self.nS = nS
        self.useH = useH
        self.logger = logger

    # TODO
    # Add the recursion depth parameter to check for recursion termination.
    # TODO
    def GetColor( self,
                  amblight = None, # ambient light, usually in RGB vector form
                  light = None, # light intensity, usually in RGB vector form
                  lightvec = None, # unit normal pointing to light
                  normalvec = None, # unit surface normal
                  viewvec = None, # unit view vector
                  lightflag = True, # whether to calculate diffuse + specular
                ):
        """Returns RGB triple in list form."""
        global logger
        ambient = self.GetAmbient( amblight )
        # Check if light source contributes
        if lightflag:
            diffuse = self.GetDiffuse( light,
                                       lightvec,
                                       normalvec )
            diffuse = [ max( 0, min( 255, x ) ) for x in diffuse ]
            specular = self.GetSpecular( light,
                                         lightvec,
                                         normalvec,
                                         viewvec )
            specular = [ max( 0, min( 255, x ) ) for x in specular ]
        else:
            diffuse = [ 0.0, 0.0, 0.0 ]
            specular = [ 0.0, 0.0, 0.0 ]
        retval = [ max( 0, min( 255, (ambient[x] + diffuse[x] + specular[x]) ) )
                   for x in xrange( len(ambient) ) ]
        return retval

    def GetAmbient( self,
                    light = None ):
        # light is the light intensity, usually in RGB vector form
        # print type(self.kAmbient), type(light)
        return self.kAmbient * light

    def GetDiffuse( self,
                    light = None, # light intensity, usually in RGB vector form
                    lightvec = None, # unit normal pointing to light
                    normalvec = None, # unit surface normal
                  ):
        ldv = (numpy.dot( lightvec, normalvec ))
        color = []
        for ind in xrange( len(self.kDiffuse) ):
            color.append( self.kDiffuse[ind] * ldv * light[ind] )
        return color

    def GetSpecular( self,
                     light = None, # light intensity, usually in RGB vector form
                     lightvec = None, # unit normal pointing to light
                     normalvec = None, # unit surface normal
                     viewvec = None, # unit view vector
                   ):
        global logger
        if numpy.dot( normalvec, viewvec ) < 0.0:
            return numpy.array( [0.0, 0.0, 0.0] )
        # logger.debug( "Phong: GetSpecular" )
        V = viewvec
        # R = CalcReflectionVector( lightvec = lightvec,
        #                           normalvec = normalvec )
        H = CalcHalfVector( lightvec = lightvec,
                            viewvec = viewvec )
        # R = R / numpy.linalg.norm( R )
        H = H / numpy.linalg.norm( H )
        # logger.debug( "k: %f  nS: %f  R: %s  V: %s" %
        #              (self.kSpecular, self.nS, str(R), str(V)) )
        # return self.kSpecular * pow( numpy.dot( V, R ), self.nS ) * light
        return self.kSpecular * pow( numpy.dot( normalvec, H ), self.nS ) * light


class PhongReflectionModel( PhongIlluminationModel ):
    def __init__( self,
                  kAmbient = 0.2,
                  kDiffuse = numpy.array( [0.0, 0.0, 0.7] ),
                  kSpecular = 0.6,
                  nS = 1.0,
                  useH = True,
                  logger = None ):
        # nS = specular reflection parameter
        #      1 = dull, 100 = shiny, infinite = perfect reflector
        LocalIlluminationModel.__init__( self,
                                         kAmbient,
                                         kDiffuse,
                                         kSpecular )
        self.nS = nS
        self.useH = useH
        self.logger = logger

    def GetColor( self,
                  amblight = None, # ambient light, usually in RGB vector form
                  light = None, # light intensity, usually in RGB vector form
                  lightvec = None, # unit normal pointing to light
                  normalvec = None, # unit surface normal
                  viewvec = None, # unit view vector
                  lightflag = True, # whether to calculate diffuse + specular
                ):
        """Returns RGB triple in list form."""
        global logger
        ambient = self.GetAmbient( amblight )
        if lightflag:
            diffuse = self.GetDiffuse( light,
                                       lightvec,
                                       normalvec )
            diffuse = [ max( 0, min( 255, x ) ) for x in diffuse ]
            specular = self.GetSpecular( light,
                                         lightvec,
                                         normalvec,
                                         viewvec )
            specular = [ max( 0, min( 255, x ) ) for x in specular ]
        else:
            diffuse = [ 0.0, 0.0, 0.0 ]
            specular = [ 0.0, 0.0, 0.0 ]
        retval = [ max( 0, min( 255, (ambient[x] + diffuse[x] + specular[x]) ) )
                   for x in xrange( len(ambient) ) ]
        return retval

    def GetAmbient( self,
                    light = None ):
        # light is the light intensity, usually in RGB vector form
        # print type(self.kAmbient), type(light)
        return self.kAmbient * light

    def GetDiffuse( self,
                    light = None, # light intensity, usually in RGB vector form
                    lightvec = None, # unit normal pointing to light
                    normalvec = None, # unit surface normal
                  ):
        ldv = (numpy.dot( lightvec, normalvec ))
        color = []
        for ind in xrange( len(self.kDiffuse) ):
            color.append( self.kDiffuse[ind] * ldv * light[ind] )
        return color

    def GetSpecular( self,
                     light = None, # light intensity, usually in RGB vector form
                     lightvec = None, # unit normal pointing to light
                     normalvec = None, # unit surface normal
                     viewvec = None, # unit view vector
                   ):
        global logger
        if numpy.dot( normalvec, viewvec ) < 0.0:
            return numpy.array( [0.0, 0.0, 0.0] )
        # logger.debug( "Phong: GetSpecular" )
        V = viewvec
        # R = CalcReflectionVector( lightvec = lightvec,
        #                           normalvec = normalvec )
        H = CalcHalfVector( lightvec = lightvec,
                            viewvec = viewvec )
        # R = R / numpy.linalg.norm( R )
        H = H / numpy.linalg.norm( H )
        # logger.debug( "k: %f  nS: %f  R: %s  V: %s" %
        #              (self.kSpecular, self.nS, str(R), str(V)) )
        # return self.kSpecular * pow( numpy.dot( V, R ), self.nS ) * light
        return self.kSpecular * pow( numpy.dot( normalvec, H ), self.nS ) * light


if __name__ == "__main__":
    print "Basic IlluminationModel class test:"
    mylight = IlluminationModel()
    print "Done"
