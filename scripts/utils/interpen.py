import numpy as np

corners = np.array([[x,y,z] for x in [-1,1] for y in [-1,1] for z in [-1,1]])

def get_R(r):
    from math import radians, cos, sin
    def rx(t): c, s = cos(t), sin(t); return np.array([[1,0,0],[0,c,-s],[0,s,c]])
    def ry(t): c, s = cos(t), sin(t); return np.array([[c,0,s],[0,1,0],[-s,0,c]])
    def rz(t): c, s = cos(t), sin(t); return np.array([[c,-s,0],[s,c,0],[0,0,1]])
    return rz(r[2]) @ rx(r[1]) @ ry(r[0])

R_red = get_R([-65, -25, -10])
R_ivory = get_R([65, -25, 15])

red_corners = (R_red @ corners.T).T + np.array([-1.5, 1.53, 0.0])
ivory_corners = (R_ivory @ corners.T).T + np.array([1.5, 1.54, -0.5])

print("Red X max:", np.max(red_corners[:, 0]))
print("Ivory X min:", np.min(ivory_corners[:, 0]))
