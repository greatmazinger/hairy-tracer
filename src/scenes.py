import os
import numpy
import logging

from rvmath.geometry import *
from rvlight.light import *
from rvlight.illuminationmodel import *
from rvcolor.utils import Color

__all__ = [ "renderSampleScene_01",
            "renderShadowScene_01",
            "renderSpheres3Scene_01",
            "renderSpheres3Scene_02",
            "renderCheckerBoard_01",
            "renderMegaReflect_01",
            "renderMegaReflect_02",
            "renderCheckerSpheres_01",
            "renderCheckerSpheresReflect_02",
            "renderCheckerSphereReflect_02",
            "profileRenderSampleScene_01", ]

def renderSampleScene_01( size = None,
                          raytracer = None,
                          world = None,
                          logger = None,
                          outfile = "simple1.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 2.4,
                           vpheight = 4.80 * 2.4 ) # defaults are set in function TODO
    readScene_plane1( world = world,
                      logger = logger )
    raytracer.render()

def renderShadowScene_01( size = None,
                          raytracer = None,
                          world = None,
                          logger = None,
                          outfile = "simple1.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 10.0,
                           vpwidth = 6.40 * 0.9,
                           vpheight = 4.80 * 0.9 ) # defaults are set in function TODO
    readScene_TestShadow( world = world,
                          logger = logger )
    raytracer.render()

def renderSpheres3Scene_01( size = None,
                            raytracer = None,
                            world = None,
                            logger = None,
                            outfile = "simple1.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 10.0,
                           vpwidth = 6.40 * 0.9,
                           vpheight = 4.80 * 0.9 ) # defaults are set in function TODO
    readScene_3spheres( world = world,
                        logger = logger )
    raytracer.render()

def renderSpheres3Scene_02( size = None,
                            raytracer = None,
                            world = None,
                            logger = None,
                            outfile = "simple1.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 10.0,
                           vpwidth = 6.40 * 0.9,
                           vpheight = 4.80 * 0.9 ) # defaults are set in function TODO
    readScene_3spheres_2( world = world,
                          logger = logger )
    raytracer.render()

def renderCheckerBoard_01( size = None,
                           raytracer = None,
                           world = None,
                           logger = None,
                           outfile = "checkers01.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 2.4,
                           vpheight = 4.80 * 2.4 ) # defaults are set in function TODO
    readScene_plane2_checkers( world = world,
                               logger = logger )
    raytracer.render()

def renderCheckerSpheres_01( size = None,
                             raytracer = None,
                             world = None,
                             logger = None,
                             outfile = "checkers01.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 2.4,
                           vpheight = 4.80 * 2.4 ) # defaults are set in function TODO
    readScene_checkers_spheres( world = world,
                                logger = logger )
    raytracer.render()

def renderCheckerSpheresReflect_02( size = None,
                                    raytracer = None,
                                    world = None,
                                    logger = None,
                                    outfile = "checkers02.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 2.4,
                           vpheight = 4.80 * 2.4 ) # defaults are set in function TODO
    readScene_checkers_spheres_reflect_2( world = world,
                                          logger = logger )
    raytracer.render()

def renderMegaReflect_01( size = None,
                          raytracer = None,
                          world = None,
                          logger = None,
                          outfile = "megareflect.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 0.7,
                           vpheight = 4.80 * 0.7 ) # defaults are set in function TODO
    
    readScene_megareflect_01( world = world,
                              logger = logger )
    raytracer.render()

def renderMegaReflect_02( size = None,
                          raytracer = None,
                          world = None,
                          logger = None,
                          outfile = "megareflect.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 0.7,
                           vpheight = 4.80 * 0.7 ) # defaults are set in function TODO
    
    readScene_megareflect_01( world = world,
                              pnormal = numpy.array( [0.0, 0.99785892, 0.06540313] ),
                              logger = logger )
    raytracer.render()

def renderCheckerSphereReflect_02( size = None,
                                   raytracer = None,
                                   world = None,
                                   logger = None,
                                   outfile = "checkersreflect_02.bmp" ):
    raytracer.setViewport( cam_origin = [0.0, 0.0, 20.0],
                           distance = 4.0,
                           vpwidth = 6.40 * 2.4,
                           vpheight = 4.80 * 2.4 ) # defaults are set in function TODO
    readScene_checkers_sphere_reflect( world = world,
                                       logger = logger )
    raytracer.render()

def profileRenderSampleScene_01( size = None,
                                 raytracer = None,
                                 world = None,
                                 logger = None,
                                 outfile = "simple1.bmp" ):
    import cProfile
    cProfile.run( 'renderSampleScene_01( size, outfile )', 'simple1_prof.txt' )

def readScene_3spheres( world = None,
                        logger = None ):
    assert( world != None )
    pnormal = numpy.array( [0.0, 1.0, 0.0] )
    ill_model1 = PhongIlluminationModel( kAmbient = 0.0, # 0.1,
                                         kDiffuse = numpy.array( [0.4, 0.0, 0.0] ),
                                         kSpecular = 0.5,
                                         nS = 15.0 )
    world.addObject( sphere.Sphere( center = numpy.array( [0.0, -13.0, -8.0] ),
                                    radius = 12.0,
                                    ill_model = ill_model1,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
                                         kSpecular = 0.7,
                                         nS = 10.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [1.0, 0.0, -3.0] ),
                                    radius = 1.0,
                                    ill_model = ill_model3,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.3, 0.3, 0.9] ),
                                         kSpecular = 1.0,
                                         nS = 100.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [-1.0, 1.0, -5.0] ),
                                    radius = 2.0,
                                    ill_model = ill_model4,
                                    world_intfn = world.doesIntersect,
                                    logger = None )
                   )
    world.addLight( Light( orig = numpy.array( [5.0, 5.0, 5.0] ),
                           color = numpy.array( [100, 100, 100] ) )
                  )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 0.0] ),
                           color = numpy.array( [255, 10, 15] ) )
                  )

def readScene_3spheres_2( world = None,
                          logger = None ):
    assert( world != None )
    pnormal = numpy.array( [0.0, 1.0, 0.0] )
    ill_model1 = PhongIlluminationModel( kAmbient = 0.0, # 0.1,
                                         kDiffuse = numpy.array( [0.1, 0.0, 0.0] ),
                                         kSpecular = 0.6,
                                         nS = 100.0 )
    world.addObject( sphere.Sphere( center = numpy.array( [0.0, -13.0, -8.0] ),
                                    radius = 12.0,
                                    ill_model = ill_model1,
                                    world_intfn = world.doesIntersect,
                                    is_reflector = True,
                                    logger = logger )
                   )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
                                         kSpecular = 0.7,
                                         nS = 10.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [1.0, 0.0, -3.0] ),
                                    radius = 1.0,
                                    ill_model = ill_model3,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.3, 0.3, 0.9] ),
                                         kSpecular = 1.0,
                                         nS = 100.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [-1.0, 1.0, -5.0] ),
                                    radius = 2.0,
                                    ill_model = ill_model4,
                                    world_intfn = world.doesIntersect,
                                    logger = None )
                   )
    world.addLight( Light( orig = numpy.array( [5.0, 5.0, 40.0] ),
                           color = numpy.array( [100, 100, 100] ) )
                  )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 20.0] ),
                           color = numpy.array( [255, 10, 15] ) )
                  )

def readScene_plane1( world = None,
                      logger = None ):
    assert( world != None )
    ill_model1 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.2, 0.2, 0.2] ),
                                         kSpecular = 0.1,
                                         nS = 1.0 )
    # world.addObject( plane.CheckeredPlane( pnormal = pnormal,
    #                                             distance = 1.0,
    #                                             ill_model =  ill_model1
    #                                           )
    #                     )
    # wallnormal = numpy.array( [0.0, 0.0, 1.0] )
    # pnormal = numpy.array( [0.0, 1.0, 0.0] )
    pnormal = numpy.array( [0.0, 0.96592583, 0.25881905] )
    world.addObject( plane.Plane( pnormal = pnormal,
                                  distance = 0.2,
                                  ill_model =  ill_model1,
                                  world_intfn = world.doesIntersect,
                                  logger = logger
                                )
                   )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
                                         kSpecular = 0.7,
                                         nS = 10.0,
                                         logger = logger )
    world.addObject( sphere.Sphere( center = numpy.array( [1.0, 1.5, 9.0] ),
                                    radius = 1.0,
                                    ill_model = ill_model3,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.3, 0.3, 0.9] ),
                                         kSpecular = 1.0,
                                         nS = 100.0,
                                         logger = logger )
    world.addObject( sphere.Sphere( center = numpy.array( [-1.0, 2.0, 7.0] ),
                                    radius = 2.0,
                                    ill_model = ill_model4,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    world.addLight( Light( orig = numpy.array( [0.0, 5.0, 40.0] ),
                           color = numpy.array( [200, 200, 200] ) )
                  )
    world.addLight( Light( orig = numpy.array( [-20.0, 40.0, 0.0] ),
                           color = numpy.array( [205, 10, 15] ) )
                  )

def readScene_plane2_checkers( world = None,
                               logger = None ):
    assert( world != None )
    # wallnormal = numpy.array( [0.0, 0.0, 1.0] )
    pnormal = numpy.array( [0.0, 0.96592583, 0.25881905] )
    ill_model2a = PhongIlluminationModel( kAmbient = 0.2,
                                          kDiffuse = numpy.array( [0.7, 0.7, 0.0] ),
                                          kSpecular = 0.4,
                                          nS = 1.0 )
    ill_model2b = PhongIlluminationModel( kAmbient = 0.1,
                                          kDiffuse = numpy.array( [0.2, 0.2, 0.2] ),
                                          kSpecular = 0.9,
                                          nS = 100.0 )
    world.addObject( plane.CheckeredPlane( pnormal = pnormal,
                                           distance = -0.2,
                                           ill_model1 =  ill_model2a,
                                           ill_model2 = ill_model2b,
                                           world_intfn = world.doesIntersect,
                                           logger = logger
                                         )
                   )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
                                         kSpecular = 0.7,
                                         nS = 10.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [1.0, 1.5, 9.0] ),
                                    radius = 1.0,
                                    ill_model = ill_model3,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.3, 0.3, 0.9] ),
                                         kSpecular = 1.0,
                                         nS = 100.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [-1.0, 2.0, 7.0] ),
                                    radius = 2.0,
                                    ill_model = ill_model4,
                                    world_intfn = world.doesIntersect,
                                    logger = None )
                   )
    world.addLight( Light( orig = numpy.array( [0.0, 5.0, 40.0] ),
                           color = numpy.array( [250, 250, 250] ) )
                  )
    world.addLight( Light( orig = numpy.array( [-20.0, 40.0, 0.0] ),
                           color = numpy.array( [205, 10, 15] ) )
                  )

def readScene_checkers_spheres( world = None,
                                logger = None ):
    assert( world != None )
    ill_model1 = PhongIlluminationModel( kAmbient = 0.0, # 0.1,
                                         kDiffuse = numpy.array( [0.4, 0.0, 0.0] ),
                                         kSpecular = 0.5,
                                         nS = 15.0 )
    world.addObject( sphere.Sphere( center = numpy.array( [-5.0, 15.5, -4.0] ),
                                    radius = 12.0,
                                    ill_model = ill_model1,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model2a = PhongIlluminationModel( kAmbient = 0.2,
                                          kDiffuse = numpy.array( [0.7, 0.7, 0.0] ),
                                          kSpecular = 0.4,
                                          nS = 1.0 )
    ill_model2b = PhongIlluminationModel( kAmbient = 0.1,
                                          kDiffuse = numpy.array( [0.2, 0.2, 0.2] ),
                                          kSpecular = 0.9,
                                          nS = 100.0 )
    pnormal = numpy.array( [0.0, 0.96592583, 0.25881905] )
    world.addObject( plane.CheckeredPlane( pnormal = pnormal,
                                           distance = -0.2,
                                           ill_model1 =  ill_model2a,
                                           ill_model2 = ill_model2b,
                                           world_intfn = world.doesIntersect,
                                           logger = logger
                                         )
                   )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
                                         kSpecular = 0.7,
                                         nS = 10.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [2.0, 3.2, 9.0] ),
                                    radius = 3.0,
                                    ill_model = ill_model3,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.3, 0.3, 0.9] ),
                                         kSpecular = 1.0,
                                         nS = 100.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [-2.0, 2.0, 4.0] ),
                                    radius = 2.0,
                                    ill_model = ill_model4,
                                    world_intfn = world.doesIntersect,
                                    logger = None )
                   )
    world.addLight( Light( orig = numpy.array( [-5.0, 5.0, 5.0] ),
                           color = numpy.array( [120, 120, 120] ) )
                  )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 0.0] ),
                           color = numpy.array( [255, 10, 15] ) )
                  )
    world.addLight( Light( orig = numpy.array( [5.0, 5.0, 55.0] ),
                           color = numpy.array( [180, 200, 200] ) )
                  )

def readScene_checkers_spheres_reflect_2( world = None,
                                          logger = None ):
    assert( world != None )
    # ill_model1 = PhongIlluminationModel( kAmbient = 0.0, # 0.1,
    #                                      kDiffuse = numpy.array( [0.4, 0.0, 0.0] ),
    #                                      kSpecular = 0.5,
    #                                      nS = 15.0 )
    # world.addObject( sphere.Sphere( center = numpy.array( [-5.0, 15.5, -4.0] ),
    #                                 radius = 12.0,
    #                                 ill_model = ill_model1,
    #                                 world_intfn = world.doesIntersect,
    #                                 logger = logger )
    #                )
    ill_model2a = PhongIlluminationModel( kAmbient = 0.2,
                                          kDiffuse = numpy.array( [0.1, 0.1, 0.6] ),
                                          kSpecular = 0.4,
                                          nS = 10.0 )
    ill_model2b = PhongIlluminationModel( kAmbient = 0.1,
                                          kDiffuse = numpy.array( [0.2, 0.2, 0.2] ),
                                          kSpecular = 0.3,
                                          nS = 1.0 )
    pnormal = numpy.array( [0.0, 0.96592583, 0.25881905] )
    world.addObject( plane.CheckeredPlane( pnormal = pnormal,
                                           distance = -0.2,
                                           ill_model1 =  ill_model2a,
                                           ill_model2 = ill_model2b,
                                           world_intfn = world.doesIntersect,
                                           logger = logger
                                         )
                   )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.1, 0.2, 0.0] ),
                                         kSpecular = 0.3,
                                         nS = 10.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [0.0, 3.2, 0.0] ),
                                    radius = 2.5,
                                    ill_model = ill_model3,
                                    is_reflector = True,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.1, 0.0, 0.9] ),
                                         kSpecular = 0.2,
                                         nS = 1.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [-5.0, 2.0, 3.0] ),
                                    radius = 1.0,
                                    ill_model = ill_model4,
                                    world_intfn = world.doesIntersect,
                                    logger = None )
                   )
    world.addLight( Light( orig = numpy.array( [-5.0, 5.0, 45.0] ),
                           color = numpy.array( [220, 220, 220] ) )
                  )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 0.0] ),
                           color = numpy.array( [255, 10, 15] ) )
                  )
    world.addLight( Light( orig = numpy.array( [5.0, 5.0, 55.0] ),
                           color = numpy.array( [18, 220, 20] ) )
                  )


def readScene_checkers_sphere_reflect( world = None,
                                       logger = None ):
    assert( world != None )
    ill_model2a = PhongIlluminationModel( kAmbient = 0.2,
                                          kDiffuse = numpy.array( [0.5, 0.2, 0.1] ),
                                          kSpecular = 0.2,
                                          nS = 1.0 )
    ill_model2b = PhongIlluminationModel( kAmbient = 0.1,
                                          kDiffuse = numpy.array( [0.2, 0.2, 0.2] ),
                                          kSpecular = 0.9,
                                          nS = 100.0 )
    # pnormal = numpy.array( [0.0, 0.96592583, 0.25881905] )
    pnormal = numpy.array( [0.0, 1.0, 0.0] )
    # world.addObject( plane.Plane( pnormal = pnormal,
    #                                    distance = 0.2,
    #                                    ill_model =  ill_model2a,
    #                                    world_intfn = world.doesIntersect,
    #                                    logger = logger
    #                                  )
    #                    )
    world.addObject( plane.CheckeredPlane( pnormal = pnormal,
                                           distance = 5.0,
                                           ill_model1 =  ill_model2a,
                                           ill_model2 = ill_model2b,
                                           world_intfn = world.doesIntersect,
                                           logger = logger
                                         )
                   )
    # ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
    #                                      kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
    #                                      kSpecular = 0.7,
    #                                      nS = 10.0,
    #                                      logger = None )
    # world.addObject( sphere.Sphere( center = numpy.array( [2.0, 3.2, 9.0] ),
    #                                      radius = 3.0,
    #                                      ill_model = ill_model3,
    #                                      world_intfn = world.doesIntersect,
    #                                      logger = logger )
    #                     )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.0, 0.0, 0.0] ),
                                         kSpecular = 0.6,
                                         nS = 100.0,
                                         logger = logger )
    world.addObject( sphere.Sphere( center = numpy.array( [0.0, 0.8, 2.8] ),
                                    radius = 3.5,
                                    ill_model = ill_model4,
                                    is_reflector = True,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    world.addLight( Light( orig = numpy.array( [2.0, 5.0, 7.0] ),
                           color = numpy.array( [220, 220, 220] ) )
                  )
    world.addLight( Light( orig = numpy.array( [-1.0, 40.0, 0.0] ),
                           color = numpy.array( [255, 10, 15] ) )
                  )

def readScene_TestShadow( world = None,
                          logger = None ):
    assert( world != None )
    pnormal = numpy.array( [0.0, 1.0, 0.0] )
    ill_model1 = PhongIlluminationModel( kAmbient = 0.0, # 0.1,
                                         kDiffuse = numpy.array( [0.9, 0.0, 0.0] ),
                                         kSpecular = 0.5,
                                         nS = 150.0 )
    world.addObject( sphere.Sphere( center = numpy.array( [0.0, -4.0, 0.0] ),
                                    radius = 3.0,
                                    ill_model = ill_model1,
                                    world_intfn = world.doesIntersect,
                                    myname = "Red sphere",
                                    logger = None )
                   )
    # wallnormal = numpy.array( [0.0, 0.0, 1.0] )
    # world.addObject( plane.Plane( pnormal = wallnormal,
    #                                    distance = -10.0,
    #                                    ill_model =  ill_model2
    #                                  )
    #                     )
    ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                         kDiffuse = numpy.array( [0.0, 1.0, 0.0] ),
                                         kSpecular = 0.7,
                                         nS = 10.0,
                                         logger = None )
    world.addObject( sphere.Sphere( center = numpy.array( [0.0, 1.0, 0.0] ),
                                    radius = 1.0,
                                    ill_model = ill_model3,
                                    world_intfn = world.doesIntersect,
                                    myname = "Green sphere",
                                    logger = None )
                   )
    ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                         kDiffuse = numpy.array( [0.1, 0.1, 1.0] ),
                                         kSpecular = 1.0,
                                         nS = 100.0,
                                         logger = None )
    # world.addObject( sphere.Sphere( center = numpy.array( [3.0, 0.0, 0.0] ),
    #                        radius = 1.0,
    #                        ill_model = ill_model4,
    #                        world_intfn = world.doesIntersect,
    #                        logger = None )
    #                     )
    # world.addLight( Light( orig = numpy.array( [0.0, 5.0, 50.0] ),
    #                             color = numpy.array( [235, 235, 235] ) )
    #                    )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 0.0] ),
                           color = numpy.array( [255, 255, 255] ) )
                  )


def readScene_megareflect_01( world = None,
                              pnormal = numpy.array( [0.0, 0.96592583, 0.25881905] ),
                              logger = None ):
    assert( world != None )
    ill_model2a = PhongIlluminationModel( kAmbient = 0.2,
                                          kDiffuse = numpy.array( [0.7, 0.7, 0.0] ),
                                          kSpecular = 1.4,
                                          nS = 1.0 )
    ill_model2b = PhongIlluminationModel( kAmbient = 0.1,
                                          kDiffuse = numpy.array( [0.2, 0.2, 0.2] ),
                                          kSpecular = 0.9,
                                          nS = 100.0 )
    world.addObject( plane.CheckeredPlane( pnormal = pnormal,
                                           distance = 0.0,
                                           ill_model1 =  ill_model2a,
                                           ill_model2 = ill_model2b,
                                           world_intfn = world.doesIntersect,
                                           logger = logger
                                         )
                   )
    ill_model1 = PhongIlluminationModel( kAmbient = 0.1,
                                         kDiffuse = numpy.array( [0.1, 0.8, 0.0] ),
                                         kSpecular = 0.5,
                                         nS = 5.0 )
    world.addObject( sphere.Sphere( center = numpy.array( [-9.0, 21.0, 60.0] ),
                                    radius = 21.0,
                                    ill_model = ill_model1,
                                    world_intfn = world.doesIntersect,
                                    logger = logger )
                   )
    ill_model_ref = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                            kDiffuse = numpy.array( [0.0, 0.0, 0.1] ),
                                            kSpecular = 0.5,
                                            nS = 100.0,
                                            logger = None )
    
    x = -2.6
    y = 0.6
    z = 3.0
    for xindex in xrange(6):
        world.addObject( sphere.Sphere( center = numpy.array( [x, y, z] ),
                                        radius = 0.5,
                                        ill_model = ill_model_ref,
                                        is_reflector = True,
                                        world_intfn = world.doesIntersect,
                                        logger = logger )
                       )
        x = x + 1.0

    world.addLight( Light( orig = numpy.array( [-9.0, 27.0, 60.0] ),
                           color = numpy.array( [190, 190, 190] ) )
                  )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 0.0] ),
                           color = numpy.array( [255, 10, 15] ) )
                  )
    world.addLight( Light( orig = numpy.array( [0.0, 40.0, 55.0] ),
                           color = numpy.array( [8, 255, 5] ) )
                  )

