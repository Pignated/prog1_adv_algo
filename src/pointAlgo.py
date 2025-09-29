
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
def check_point(point, polygon):
    left_intersections = 0
    for i in range(len(polygon)):
        p0 = polygon[i]
        p1 = polygon[(i + 1) % len(polygon)]
        if p0.y == p1.y:
            # print(1,polygon[i])
            continue
        if p0.y == point.y and p0.x <= point.x:
            left_intersections += 1
            # print(2,polygon[i])
            if ((polygon[(i-1)%len(polygon)]).y - point.y)*(p1.y-point.y) >= 0:
                # print(3, polygon[i])
                left_intersections += 1
            continue

        if min(p0.y, p1.y) < point.y  and point.y < max(p0.y, p1.y):
            x_intersect = ((point.y - p0.y)*(p0.x - p1.x)/(p0.y-p1.y))+p0.x
            if x_intersect < point.x:
                # print(4, polygon[i])
                left_intersections += 1

    return left_intersections % 2 == 1


if __name__ == "__main__":
    # Tests
    assert(check_point(Point(2,2), [Point(1,9),Point(8,3),Point(0,-6),Point(-4,2)]))
    assert(not check_point(Point(2,2), [Point(1,9),Point(8,3),Point(0,6),Point(-4,2)]))
    assert(check_point(Point(2,2),[Point(1.5,3),Point(1,9),Point(8,3),Point(0,-6),Point(-4,2),Point(-2,1)]))
    assert(check_point(Point(0,0),[Point(1,0),Point(0,1),Point(-1,0),Point(0,-1)]))

