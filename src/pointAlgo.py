import math


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    def magnitude(self):
        return math.sqrt(self.x**2 + self.y**2)

def check_point(point, polygon):
    left_intersections = 0
    for i in range(len(polygon)):
        p0 = polygon[i]
        p1 = polygon[(i + 1) % len(polygon)]
        if p0.y == p1.y:
            # print(1,polygon[0])
            continue
        if p0.y == point.y and p0.x <= point.x:
            left_intersections += 1
            # print(2,polygon[0])
            if ((polygon[(i-1)%len(polygon)]).y - point.y)*(p1.y-point.y) > 0:
                # print(3, polygon[0])
                left_intersections += 1
            continue

        if min(p0.y, p1.y) < point.y < max(p0.y, p1.y):
            x_intersect = ((point.y - p0.y)*(p0.x - p1.x)/(p0.y-p1.y))+p0.x
            if x_intersect < point.x:
                # print(4, polygon[0])
                left_intersections += 1
    return left_intersections


if __name__ == "__main__":
    assert(check_point(Point(2,2), [Point(1,9),Point(8,3),Point(0,-6),Point(-4,2)]))
#Tests

