import numpy
from numpy import dot

from . import ray

class Mesh():
    def __init__(self,
                 triangles=[],
                 ill_model=None,
                 world_intfn=None,
                 myname="mesh",
                 is_reflector=False,
                 is_refractor=False):
        self.triangles = triangles
        self.ill_model = ill_model
        self.wintfn = world_intfn
        self.myname = myname
        self.reflector_flag = is_reflector
        self.refractor_flag = is_refractor
        
        # Calculate Axis-Aligned Bounding Box (AABB)
        self.min_bounds = numpy.array([float('inf'), float('inf'), float('inf')])
        self.max_bounds = numpy.array([float('-inf'), float('-inf'), float('-inf')])
        
        for tri in self.triangles:
            for v in [tri.v0, tri.v1, tri.v2]:
                self.min_bounds = numpy.minimum(self.min_bounds, v)
                self.max_bounds = numpy.maximum(self.max_bounds, v)
                
    def _intersect_aabb(self, dray):
        """
        Slab method for ray-AABB intersection.
        Returns True if the ray hits the bounding box, False otherwise.
        """
        orig = dray.getOrigin()
        dir = dray.getDirection()
        
        # Add epsilon to avoid division by zero
        dir = numpy.where(dir == 0, 1e-8, dir)
        
        tmin = (self.min_bounds - orig) / dir
        tmax = (self.max_bounds - orig) / dir
        
        t1 = numpy.minimum(tmin, tmax)
        t2 = numpy.maximum(tmin, tmax)
        
        tnear = numpy.max(t1)
        tfar = numpy.min(t2)
        
        if tnear > tfar or tfar < 0:
            return False
        return True

    def findIntersection(self, dray=None):
        # 1. Fast AABB Check
        if not self._intersect_aabb(dray):
            return (None, None, None, None)
            
        # 2. Detailed Triangle Check
        best_t = float('inf')
        best_result = (None, None, None, None)
        
        for tri in self.triangles:
            (intpoint, viewvec, un, t) = tri.findIntersection(dray)
            if t is not None and t < best_t:
                best_t = t
                best_result = (intpoint, viewvec, un, t)
                
        return best_result

    def findIntersectionAndColor(self, dray=None, lightlist=None):
        (intpoint, viewvec, un, t) = self.findIntersection(dray=dray)
        if intpoint is None:
            return (None, None)
            
        # To get the color, we evaluate the lighting using the specific hit point and normal
        # We can just delegate to one of the triangles' GetColor function for convenience,
        # but the Mesh holds the material so we should compute it here.
        color = self.GetColor(intpoint=intpoint, lightlist=lightlist, viewvec=viewvec, normal=un)
        return (intpoint, color)
        
    def GetColor(self, intpoint=None, lightlist=[], viewvec=None, normal=None):
        amblight = numpy.array([15, 75, 255])
        color = numpy.array([0.0, 0.0, 0.0])
        
        for l in lightlist:
            light = l.GetColor()
            lightvec = l.GetOrigin() - intpoint
            lightvec = lightvec / numpy.linalg.norm(lightvec)
            lightflag = True
            
            # Shadow check
            if self.wintfn(srcobject=self, lightray=lightvec, intpoint=intpoint):
                lightflag = False
                
            color = color + self.ill_model.GetColor(amblight=amblight,
                                                    light=light,
                                                    lightvec=lightvec,
                                                    normalvec=normal,
                                                    viewvec=viewvec,
                                                    lightflag=lightflag)
        return [max(0, min(255, x)) for x in color]

    def IsReflector(self):
        return self.reflector_flag

    def IsRefractor(self):
        return self.refractor_flag
