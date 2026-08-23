import os
from PIL import Image
import numpy
import itertools
import multiprocessing
from optparse import OptionParser
import logging

from rvmath.geometry import *
from rvlight.light import *
from rvlight.illuminationmodel import *
from rvcolor.utils import Color
from rvmath.geometry.utils import *
import scene_parser
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

def render_scanline(args):
    vlist, cam_origin, world, maxdepth = args
    scanline_data = []
    for v in vlist:
        tmpray = ray.Ray( orig = cam_origin,
                          dir = numpy.array( v ) - cam_origin )
        mycolor = traceRay_worker( tmpray, 1, world, maxdepth )
        scanline_data.append( tuple(int(c) for c in mycolor) )
    return scanline_data

def traceRay_worker(myray, depth, world, maxdepth, srcobject=None):
    if depth > maxdepth:
        return Color.BLACK
    else:
        color = []
        tmpcolor = None
        (object, hitpoint, tmpcolor) = \
                world.findIntersectionAndColor( myray,
                                                srcobject = srcobject )
        if tmpcolor is None:
            tmpcolor = Color.BLACK
        color.append( tmpcolor )
        srcname = "None"
        if object is not None and object.IsReflector():
            invec = myray.getDirection()
            invec = -1 * invec / numpy.linalg.norm( invec )
            un = object.getUnitNormal( point = hitpoint )
            rvec = CalcReflectionVector( invec = invec,
                                         normalvec = un )
            rvec = (rvec / numpy.linalg.norm( rvec ))
            refray = ray.Ray( hitpoint,
                              dir = rvec )
            color.append( traceRay_worker( refray,
                                           depth + 1,
                                           world,
                                           maxdepth,
                                           srcobject = object ) )
        if object is not None and object.IsRefractor():
            invec = myray.getDirection()
            invec = invec / numpy.linalg.norm( invec )
            un = object.getUnitNormal( point = hitpoint )
            refract_vec = CalcRefractionVector( invec = invec,
                                                normalvec = un,
                                                ior = 1.5 )
            if refract_vec is not None:
                refract_vec = refract_vec / numpy.linalg.norm( refract_vec )
                offset_point = hitpoint + refract_vec * 0.001
                refract_ray = ray.Ray( offset_point, dir = refract_vec )
                color.append( traceRay_worker( refract_ray,
                                               depth + 1,
                                               world,
                                               maxdepth,
                                               srcobject = object ) )

        retcolor = [0.0, 0.0, 0.0]
        for x in color:
            retcolor[0] = retcolor[0] + x[0]
            retcolor[1] = retcolor[1] + x[1]
            retcolor[2] = retcolor[2] + x[2]
        return [ max( 0, min( 255, x ) ) for x in retcolor ]

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
        print("Rendering image:", self.width, "x", self.height)
        pixdata = None
        if self.testflag:
            from rvcolor.utils import get_pattern_data
            print("Rendering testdata.")
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
            assert( pixdata is not None and pixdata != [] )
            print("DBG: imagesize =", image.size, "  length =", len(pixdata))
            image.putdata( pixdata )
            image = image.transpose( Image.FLIP_TOP_BOTTOM )
            image.save( self.output )
        
    def setViewport( self,
                     cam_origin = [0.0, 0.0, 60.0],
                     distance = 30.0,
                     vpwidth = 64.0,
                     vpheight = 48.0,
                     look_at = None,
                     up = [0.0, 1.0, 0.0]):
        assert( vpwidth > 0 and vpheight > 0 )
        self.vpwidth = vpwidth
        self.vpheight = vpheight
        self.distance = distance
        self.cam_origin = numpy.array(cam_origin)
        
        if look_at is None:
            # Backward compatibility: old system placed the viewport center at (0, 0, distance)
            self.look_at = numpy.array([0.0, 0.0, float(distance)])
        else:
            self.look_at = numpy.array(look_at)
            
        self.up = numpy.array(up)
        
        # Calculate camera basis vectors
        # w points from look_at TO cam_origin (backwards)
        w = self.cam_origin - self.look_at
        w_norm = numpy.linalg.norm(w)
        if w_norm == 0:
            # Fallback if camera is exactly at look_at
            self.w = numpy.array([0.0, 0.0, 1.0])
        else:
            self.w = w / w_norm
            
        # u points to the right
        u = numpy.cross(self.up, self.w)
        u_norm = numpy.linalg.norm(u)
        if u_norm == 0:
            self.u = numpy.array([1.0, 0.0, 0.0])
        else:
            self.u = u / u_norm
            
        # v points up (orthogonal to w and u)
        self.v = numpy.cross(self.w, self.u)

    def get_data( self ):
        vlists = list(self.getSimpleVertex((self.vpwidth, self.vpheight),
                                           (self.width, self.height)))
        args = [(vlist, self.cam_origin, self.world, self.maxdepth) for vlist in vlists]
        pool = multiprocessing.Pool()
        results = pool.map(render_scanline, args)
        pool.close()
        pool.join()
        
        data = [pixel for scanline in results for pixel in scanline]
        return data

    def getSimpleVertex( self,
                         vpdim = None,
                         imgdim = None ):
        # The viewport center is exactly at look_at.
        # We trace from -vpwidth/2 to +vpwidth/2, and -vpheight/2 to +vpheight/2.
        xd = vpdim[0] / float( imgdim[0] )
        yd = vpdim[1] / float( imgdim[1] )
        xright = vpdim[0] / 2.0 
        yright = vpdim[1] / 2.0 
        
        y_scalar = -1 * yright
        for ytmp in range( imgdim[1] ):
            x_scalar = -1 * xright
            for xtmp in range( imgdim[0] ):
                x_scalar = x_scalar + xd
                
                # Compute exact point on the viewport plane in world coordinates
                target_point = self.look_at + (x_scalar * self.u) + (y_scalar * self.v)
                yield [ target_point ]
                
            y_scalar = y_scalar + yd



def processSize( size = None ):
    (width, height) = size.lower().split( 'x' )
    return (int( width ), int( height ))

if __name__ == "__main__":
    usage = "Usage: %prog [options]"
    parser = OptionParser( usage = usage )
    parser.set_defaults( size = "640x480",
                         outfile = "output.bmp",
                         scene = None,
                         profileflag = False )
    parser.add_option( "--size",
                       action="store",
                       dest="size",
                       help="Specify size as <width>x<height> like this --size=640x480" )
    parser.add_option( "--outfile",
                       action="store",
                       dest="outfile",
                       help="Specify output filename." )
    parser.add_option( "--scene",
                       action="store",
                       dest="scene",
                       help="Specify path to a JSON scene configuration file." )
    parser.add_option( "--profile",
                       action = "store_true",
                       dest = "profileflag",
                       help = "help for profile" )
    
    (options, args) = parser.parse_args()
    
    if not options.scene:
        print("Error: You must specify a scene file using --scene")
        parser.usage()
        sys.exit(1)
        
    size = options.size
    try:
        size = processSize( size )
    except:
        print("Unable to parse size arguments.")
        parser.usage()
        sys.exit(1)
        
    print(("size : ", size))
    outfile = options.outfile
    
    # Initialize the raytracer
    raytracer = BTracer( output = outfile,
                         size = size, 
                         testflag = False )
                         
    # Load the scene from the JSON file
    scene_parser.load_scene(options.scene, raytracer.world, raytracer)
    
    # Render the scene
    raytracer.render()

