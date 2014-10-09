import os
import Image
import numpy
import itertools
from optparse import OptionParser
import logging

from rvmath.geometry import *
from rvlight.light import *
from rvlight.illuminationmodel import *
from rvcolor.utils import Color
from rvmath.geometry.utils import *
from scenes import *
import utils.timing 


# Setup logging
logger = None
logger_name = 'btrace'
def setup_logger( targetdir = ".",
                  filename = "btrace.log",
                  debugflag = 0 ):
    global logger
    global logger_name
    # Set up main logger
    logger = logging.getLogger( logger_name )
    formatter = logging.Formatter( '[%(module)s] %(funcName)s : %(message)s' )
    logger.setLevel( logging.DEBUG )
    filehandler = logging.FileHandler( os.path.join( targetdir, filename ) , 'w' )
    filehandler.setLevel( logging.DEBUG )
    filehandler.setFormatter( formatter )
    logger.addHandler( filehandler )
    if debugflag:
        chandler = logging.StreamHandler()
        chandler.setLevel( logging.DEBUG )
        chandler.setFormatter( formatter )
        logger.addHandler( chandler )

setup_logger()

class BTracer():
    def __init__( self,
                  output = "image.bmp",
                  size = (640, 480),
                  type = "BMP",
                  maxdepth = 2,
                  testflag = True ):
        global logger_name
        self.width = size[0]
        self.height = size[1]
        self.size = size
        self.output = output
        self.type = type
        self.maxdepth = maxdepth
        self.testflag = testflag
        self.world = world.World( logger_name = logger_name )
        self.vpwidth = None
        self.vpheight = None
        self.distance = None

    def render( self ):
        print "Rendering image:", self.width, "x", self.height
        pixdata = None
        if self.testflag:
            from rvcolor.utils import get_pattern_data
            print "Rendering testdata."
            image = Image.new( "RGB",
                               self.size,
                               (0, 0, 255) )
            pixdata = get_pattern_data( self.width, self.height )
            image.putdata( pixdata )
            image.save( self.output )
        else:
            image = Image.new( "RGB",
                               self.size,
                               (0, 0, 255) )
            pixdata = self.get_data()
            assert( pixdata != None and pixdata != [] )
            print "DBG: imagesize =", image.size, "  length =", len(pixdata)
            image.putdata( pixdata )
            image = image.transpose( Image.FLIP_TOP_BOTTOM )
            image.save( self.output )
        
    def setViewport( self,
                     cam_origin = [0.0, 0.0, 60.0],
                     distance = 30.0,
                     vpwidth = 64.0,
                     vpheight = 48.0):
        # TODO: check the parameters
        # TODO: Assume the viewport is parallel to XY plane
        #       and perpendicular to the XZ plane.
        # TODO: Need to convert camera and objects to this
        #       space
        # Given A = upleft, B = upright, C = lowright,
        # the camera is on the side of the plane where
        # BA x BC.
        # Therefore the viewing direction is in the opposite
        # direction
        assert( vpwidth > 0 and vpheight > 0 )
        self.vpwidth = vpwidth
        self.vpheight = vpheight
        self.distance = distance
        self.cam_origin = cam_origin

    def get_data( self ):
        origin = numpy.array( [0, 0, 0] )
        data = []
        for vlist in self.getSimpleVertex( (self.vpwidth, self.vpheight),
                                           (self.width, self.height) ):
            for v in vlist:
                tmpray = ray.Ray( orig = self.cam_origin,
                                  dir = numpy.array( v ) - self.cam_origin )
                mycolor = self.traceRay( myray = tmpray,
                                         depth = 1 )
                data.append( tuple(mycolor) )
        return data

    def getSimpleVertex( self,
                         vpdim = None,
                         imgdim = None ):
        # returns lists of vertices
        # Get the corresponding deltas in object space
        xd = vpdim[0] / float( imgdim[0] )
        yd = vpdim[1] / float( imgdim[1] )
        xright = vpdim[0] / 2.0 
        yright = vpdim[1] / 2.0 
        x = -1 * xright
        y = -1 * yright
        for ytmp in xrange( imgdim[1] ):
            x = -1 * xright
            for xtmp in xrange( imgdim[0] ):
                x = x + xd
                yield [ (x, y, self.distance) ]
                # we can use distance here because
                # the viewport is parallel to the XY plane and
                # on the positive Z axis.
            y = y + yd

    def traceRay( self,
                  myray = None,
                  depth = None,
                  srcobject = None ):
        global logger
        if depth > self.maxdepth:
            return Color.BLACK
        else:
            color = []
            tmpcolor = None
            (object, hitpoint, tmpcolor) = \
                    self.world.findIntersectionAndColor( myray,
                                                         srcobject = srcobject )
            if tmpcolor == None:
                tmpcolor = Color.BLACK
            color.append( tmpcolor )
            srcname = "None"
            if object != None and object.IsReflector():
                invec = myray.getDirection()
                invec = -1 * invec / numpy.linalg.norm( invec )
                un = object.getUnitNormal( point = hitpoint )
                rvec = CalcReflectionVector( invec = invec,
                                             normalvec = un )
                rvec = (rvec / numpy.linalg.norm( rvec ))
                refray = ray.Ray( hitpoint, # numpy.array([0.,0.,0.]),
                                  dir = rvec )
                color.append( self.traceRay( myray = refray,
                                             depth = depth + 1,
                                             srcobject = object ) )
                # logger.debug( "refcolor: %s" % str(color[-1]) )
                logger.debug( "reflect : %s" % str(rvec) )
                logger.debug( "invec: %s" % str(invec) )
                if srcobject != None:
                    srcname = srcobject.myname
            if depth > 1:
                if object == None:
                    tmpobjname = "None"
                else:
                    tmpobjname = object.myname
                logger.debug( "depth: %d  intpt: %s obj: %s  src: %s" %
                              (depth, str(hitpoint), tmpobjname, srcname) )
            if object != None and object.IsRefractor():
                pass
            retcolor = [0.0, 0.0, 0.0]
            for x in color:
                retcolor[0] = retcolor[0] + x[0]
                retcolor[1] = retcolor[1] + x[1]
                retcolor[2] = retcolor[2] + x[2]
            return [ max( 0, min( 255, x ) ) for x in retcolor ]

def processSize( size = None ):
    (width, height) = size.lower().split( 'x' )
    return (int( width ), int( height ))

if __name__ == "__main__":
    usage = "Usage: %prog [options]"
    parser = OptionParser( usage = usage )
    parser.set_defaults( size = "640x480",
                         outfile = "simple1.bmp",
                         profileflag = False,
                         checkers_flag = False,
                         checkers_spheres_flag = False,
                         checkers_spheres_reflect_flag = False,
                         megareflect_flag = False,
                         megareflect2_flag = False,
                         sample01_flag = False,
                         shadow01_flag = False,
                         spheres3_flag = False,
                         spheres3_reflect_flag = False,
                         spheres_reflect_flag = False, )
    parser.add_option( "--size",
                       action="store",
                       dest="size",
                       help="Specify size as <width>x<height> like this --size=640x480" )
    parser.add_option( "--outfile",
                       action="store",
                       dest="outfile",
                       help="Specify output filename." )
    parser.add_option( "--profile",
                       action = "store_true",
                       dest = "profileflag",
                       help = "help for profile" )
    parser.add_option( "--sample01",
                       action = "store_true",
                       dest = "sample01_flag",
                       help = "Render sample scene 01." )
    parser.add_option( "--shadow01",
                       action = "store_true",
                       dest = "shadow01_flag",
                       help = "Render shadow test scene 01." )
    parser.add_option( "--spheres3",
                       action = "store_true",
                       dest = "spheres3_flag",
                       help = "Render 3 spheres scene." )
    parser.add_option( "--spheres3-reflect",
                       action = "store_true",
                       dest = "spheres3_reflect_flag",
                       help = "Render 3 spheres scene with reflective large sphere." )
    parser.add_option( "--checkers",
                       action = "store_true",
                       dest = "checkers_flag",
                       help = "Render checkerboard scene." )
    parser.add_option( "--checkers-spheres",
                       action = "store_true",
                       dest = "checkers_spheres_flag",
                       help = "Render checkerboard scene with spheres." )
    parser.add_option( "--checkers-spheres-reflect",
                       action = "store_true",
                       dest = "checkers_spheres_reflect_flag",
                       help = "Render checkerboard scene with spheres (one is reflective)." )
    parser.add_option( "--megareflect",
                       action = "store_true",
                       dest = "megareflect_flag",
                       help = "Render lots of reflective spheres on checkerboard." )
    parser.add_option( "--megareflect2",
                       action = "store_true",
                       dest = "megareflect2_flag",
                       help = "Render lots of reflective spheres on checkerboard." )
    parser.add_option( "--spheres-reflect",
                       action = "store_true",
                       dest = "spheres_reflect_flag",
                       help = "Render large reflective sphere on checkerboard pattern." )
    (options, args) = parser.parse_args()
    size = options.size
    try:
        size = processSize( size )
    except:
        print "Unable to parse size arguments."
        parser.usage()
    print "size : ", size
    profileflag = options.profileflag
    sample01_flag = options.sample01_flag
    shadow01_flag = options.shadow01_flag
    spheres3_flag = options.spheres3_flag
    spheres3_reflect_flag = options.spheres3_reflect_flag
    checkers_flag = options.checkers_flag
    checkers_spheres_flag = options.checkers_spheres_flag
    checkers_spheres_reflect_flag = options.checkers_spheres_reflect_flag
    megareflect_flag = options.megareflect_flag
    megareflect2_flag = options.megareflect2_flag
    spheres_reflect_flag = options.spheres_reflect_flag
    outfile = options.outfile
    # TODO
    # Check to see if its a valid filename
    if sample01_flag:
        if profileflag:
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False )
            renderSampleScene_01( size = size,
                                  world = raytracer.world,
                                  raytracer = raytracer,
                                  outfile = outfile )
    elif shadow01_flag:
        if profileflag:
            print "TODO! This part is under construction. Try --sample01"
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False )
            renderShadowScene_01( size = size,
                                  world = raytracer.world,
                                  raytracer = raytracer,
                                  outfile = outfile )
    elif spheres3_flag:
        if profileflag:
            print "TODO! This part is under construction. Try --sample01"
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False )
            renderSpheres3Scene_01( size = size,
                                    world = raytracer.world,
                                    raytracer = raytracer,
                                    outfile = outfile )
    elif spheres3_reflect_flag:
        if profileflag:
            print "TODO! This part is under construction. Try --sample01"
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False )
            renderSpheres3Scene_02( size = size,
                                    world = raytracer.world,
                                    raytracer = raytracer,
                                    outfile = outfile )
    elif checkers_flag:
        if profileflag:
            print "TODO! This part is under construction. No profiling in checkers."
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False )
            renderCheckerBoard_01( size = size,
                                   world = raytracer.world,
                                   raytracer = raytracer,
                                   outfile = outfile )
    elif checkers_spheres_flag:
        if profileflag:
            print "TODO! This part is under construction. No profiling in checkers."
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False,
                                 maxdepth = 2 )
            renderCheckerSpheres_01( size = size,
                                     world = raytracer.world,
                                     raytracer = raytracer,
                                     outfile = outfile )
    elif checkers_spheres_reflect_flag:
        if profileflag:
            print "TODO! This part is under construction. No profiling in checkers."
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False,
                                 maxdepth = 2 )
            renderCheckerSpheresReflect_02( size = size,
                                            world = raytracer.world,
                                            raytracer = raytracer,
                                            outfile = outfile )
    elif megareflect_flag:
        if profileflag:
            print "TODO! This part is under construction. No profiling in checkers."
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False,
                                 maxdepth = 6 )
            renderMegaReflect_01( size = size,
                                  world = raytracer.world,
                                  raytracer = raytracer,
                                  outfile = outfile )
    elif megareflect2_flag:
        if profileflag:
            print "TODO! This part is under construction. No profiling in checkers."
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False,
                                 maxdepth = 6 )
            renderMegaReflect_02( size = size,
                                  world = raytracer.world,
                                  raytracer = raytracer,
                                  outfile = outfile )
    elif spheres_reflect_flag:
        if profileflag:
            print "TODO! This part is under construction. No profiling in checkers."
            assert(0)
            profileRenderSampleScene_01( size = size,
                                         world = raytracer.world,
                                         raytracer = raytracer,
                                         outfile = outfile )
        else:
            raytracer = BTracer( output = outfile,
                                 size = size, 
                                 testflag = False,
                                 maxdepth = 2 )
            renderCheckerSphereReflect_02( size = size,
                                           world = raytracer.world,
                                           raytracer = raytracer,
                                           outfile = outfile )
    else:
        print "TODO! This part is under construction. Try --sample01"
