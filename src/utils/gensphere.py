import os
import itertools
import logging

def generate_sphere( center = None,
                     radius = None,
                     kAmbient = None, ):
                     kSpecular = None,
                     nS = None
    print"""
    def %(funcname)( self ):
        global logger_name
        pnormal = numpy.array( [0.0, 1.0, 0.0] )
        ill_model1 = PhongIlluminationModel( kAmbient = 0.0, # 0.1,
                                             kDiffuse = numpy.array( [0.4, 0.0, 0.0] ),
                                             kSpecular = 0.5,
                                             nS = 15.0 )
        self.world.addObject( sphere.Sphere( center = numpy.array( [0.0, -13.0, -8.0] ),
                                             radius = 12.0,
                                             ill_model = ill_model1,
                                             world_intfn = self.world.doesIntersect,
                                             logger_name = logger_name )
                            )
        ill_model3 = PhongIlluminationModel( kAmbient = 0.0, # 0.3,
                                             kDiffuse = numpy.array( [0.1, 0.7, 0.0] ),
                                             kSpecular = 0.7,
                                             nS = 10.0,
                                             logger_name = None )
        self.world.addObject( sphere.Sphere( center = numpy.array( [1.0, 0.0, -3.0] ),
                                             radius = 1.0,
                                             ill_model = ill_model3,
                                             world_intfn = self.world.doesIntersect,
                                             logger_name = logger_name )
                            )
        ill_model4 = PhongIlluminationModel( kAmbient = 0.0,
                                             kDiffuse = numpy.array( [0.3, 0.3, 0.9] ),
                                             kSpecular = 1.0,
                                             nS = 100.0,
                                             logger_name = None )
        self.world.addObject( sphere.Sphere( center = numpy.array( [-1.0, 1.0, -5.0] ),
                                             radius = 2.0,
                                             ill_model = ill_model4,
                                             world_intfn = self.world.doesIntersect,
                                             logger_name = None )
                            )
        self.world.addLight( Light( orig = numpy.array( [5.0, 5.0, 5.0] ),
                                    color = numpy.array( [100, 100, 100] ) )
                           )
        self.world.addLight( Light( orig = numpy.array( [0.0, 40.0, 0.0] ),
                                    color = numpy.array( [255, 10, 15] ) )
                            )
        """
