from scipy.optimize import linprog
from tkinter import *

class Thruster:
    def __init__(self, position: tuple[float, float], force_unit: tuple[float, float]) -> None:
        self.level = 0
        self.position = position
        self.force_unit = force_unit

thrusters = [
    Thruster((10.0, 78.0), (0.0, -1.0)),
    Thruster((10.0, 78.0), (1.0, 0.0)),
    Thruster((10.0, 78.0), (-1.0, 0.0)),

    Thruster((10.0, -78.0), (0.0, 1.0)),
    Thruster((10.0, -78.0), (1.0, 0.0)),
    Thruster((10.0, -78.0), (-1.0, 0.0)),

    Thruster((-199.0, 88.0), (0.0, -1.0)),
    Thruster((-199.0, 88.0), (1.0, 0.0)),
    Thruster((-199.0, 88.0), (-1.0, 0.0)),

    Thruster((-199.0, -88.0), (0.0, 1.0)),
    Thruster((-199.0, -88.0), (1.0, 0.0)),
    Thruster((-199.0, -88.0), (-1.0, 0.0)),
]

def angular_acceleration() -> float:
    angular_force = 0
    for thruster in thrusters:
        angular_force += thruster.level * (thruster.force_unit[1] * thruster.position[0] - thruster.force_unit[0] * thruster.position[1])
    return angular_force

def acceleration() -> tuple[float, float]:
    angular_acceleration = [0.0, 0.0]
    for thruster in thrusters:
        angular_acceleration[0] += thruster.level * thruster.force_unit[0]
        angular_acceleration[1] += thruster.level * thruster.force_unit[1]
    return angular_acceleration

objective = []
for thruster in thrusters:
    objective.append(-(thruster.force_unit[1] * thruster.position[0] - thruster.force_unit[0] * thruster.position[1]))

bounds = []
for i in range(0, len(thrusters)):
    bounds.append((0, 1))

equality_x = []
equality_y = []
for thruster in thrusters:
    equality_x.append(thruster.force_unit[0])
    equality_y.append(thruster.force_unit[1])

equalities_lhs = [equality_x, equality_y]
equalities_rhs = [0, 0]

result = linprog(c = objective, A_eq = equalities_lhs, b_eq = equalities_rhs, bounds = bounds, method = "highs")
values = result.get("x")
print("values:", values)
print("angular force:", result.get("fun"))
for i in range(0, len(thrusters)):
    thrusters[i].level = float(values[i])

window = Tk()

c = Canvas(window, bg = "black", width = 500, height = 500)
c.pack()

def from_rgb(rgb):
    return "#%02x%02x%02x" % rgb   

for thruster in thrusters:
    level_for_rgb = round(thruster.level * 255)
    fill = from_rgb((level_for_rgb, level_for_rgb, level_for_rgb))
                    
    x = 250 + thruster.position[0]
    y = 250 + thruster.position[1]

    c.create_oval(x - 5.0, y - 5.0, x + 5.0, y + 5.0, fill = "red")
    c.create_line(x, y, x + 30.0 * thruster.force_unit[0], y + 30.0 * thruster.force_unit[1], fill = fill)

window.mainloop()