import numpy as np

def rot_x(t):
    t = np.radians(t)
    c, s = np.cos(t), np.sin(t)
    return np.array([[1, 0, 0], [0, c, -s], [0, s, c]])

def rot_y(t):
    t = np.radians(t)
    c, s = np.cos(t), np.sin(t)
    return np.array([[c, 0, s], [0, 1, 0], [-s, 0, c]])

def rot_z(t):
    t = np.radians(t)
    c, s = np.cos(t), np.sin(t)
    return np.array([[c, -s, 0], [s, c, 0], [0, 0, 1]])

# red = Z(-10) * X(-25) * Y(-65)
R_red = rot_z(-10) @ rot_x(-25) @ rot_y(-65)
# ivory = Z(15) * X(-25) * Y(65)
R_ivory = rot_z(15) @ rot_x(-25) @ rot_y(65)

corners = np.array([[x,y,z] for x in [-1,1] for y in [-1,1] for z in [-1,1]])

red_corners = (R_red @ corners.T).T
ivory_corners = (R_ivory @ corners.T).T

red_min_y = np.min(red_corners[:, 1])
ivory_min_y = np.min(ivory_corners[:, 1])

print(f"Red min Y relative to center: {red_min_y}")
print(f"Ivory min Y relative to center: {ivory_min_y}")
print(f"Suggested Red Y: {-red_min_y}")
print(f"Suggested Ivory Y: {-ivory_min_y}")
