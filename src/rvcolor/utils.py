#
# Color utility functions
#
__all__ = [ "get_pattern_data" ]

class Color:
    BLACK = (0, 0, 0)
    RED = (1, 0, 0)
    GREEN = (0, 1, 0)
    BLUE = (0, 0, 1)
    WHITE = (1, 1, 1)

def get_pattern_data( width = 0, height = 0 ):
    assert( width > 0 and height > 0 )
    data = []
    num_horizontal_stripes = 5
    jline = 0.8 * height
    iwide = (1.0/num_horizontal_stripes) * width
    for j in xrange( height ):
        for i in xrange( width ):
            if j > jline:
                # output red
                data.append( (255, 0, 0 ) ) # Red
            else:
                if i < iwide:
                    data.append( (0, 0, 0 ) ) # Black
                elif i < 2*iwide:
                    data.append( (0, 255, 0 ) ) # Green
                elif i < 3*iwide:
                    data.append( (0, 0, 255 ) ) # Blue
                elif i < 4*iwide:
                    data.append( (128, 128, 128 ) ) # Grey?
                else:
                    data.append( (205, 205, 255 ) ) # ???
    return data

