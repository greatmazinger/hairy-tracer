#
# Math utility functions
#
import math
import numpy
from numpy import dot

__all__ = [ "fequal", "getMatrixForRotateToZaxis", "CalcReflectionVector",
            "CalcHalfVector" ]

# Floating point equality check
def fequal(a, b):
    return abs(a - b) < 1e-6

# TODO invec is actually pointing out
def CalcReflectionVector( invec = None, # unit in vector
                          normalvec = None, # surface normal vector
                        ):
    N = normalvec
    L = invec
    return ((numpy.dot(N, 2*L)) * N) - L
    # return L - ((numpy.dot(N, 2*L)) * N)

def CalcHalfVector( lightvec = None, # unit light vector
                    viewvec = None, # view vector
                  ):
    lv = (lightvec + viewvec)
    return lv / numpy.linalg.norm( lv )

def getMatrixForRotateToZaxis( myvec ):
    u = myvec[0]
    v = myvec[1]
    w = myvec[2]
    u2v2 = u*u + v*v
    sqrt_u2v2 = math.sqrt( u2v2 )
    u_d_sqrt_u2v2 = u / sqrt_u2v2
    v_d_sqrt_u2v2 = v / sqrt_u2v2
    u2v2w2 = u2v2 + w*w
    sqrt_u2v2w2 = math.sqrt( u2v2w2 )
    w_d_sqrt_u2v2w2 = w / sqrt_u2v2w2
    sqrt_u2v2_d_sqrt_u2v2w2 = sqrt_u2v2 / sqrt_u2v2w2
    # Not yet done. TODO
    # - complete operation. equations on jumbobing
    # - document steps
    translate = \
        numpy.dot( numpy.array( [[u_d_sqrt_u2v2,    v_d_sqrt_u2v2, 0, 0],
                                 [-1*v_d_sqrt_u2v2, u_d_sqrt_u2v2, 0, 0],
                                 [0,                0,             1, 0],
                                 [0,                0,             0, 1]] ),
                   numpy.array( [[w_d_sqrt_u2v2w2,      -1*sqrt_u2v2_d_sqrt_u2v2w2,   0, 0],
                                 [sqrt_u2v2_d_sqrt_u2v2w2, w_d_sqrt_u2v2w2,           0, 0],
                                 [0,                       0,                         1, 0],
                                 [0,                       0,                         0, 1]] )
                   )


