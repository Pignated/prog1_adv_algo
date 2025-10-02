import json
import os.path

import pandas as pd
from src.pointAlgo import Point, check_point


class Test:
    def __init__(self, point, polygon, inside):
        self.point = point
        self.polygon = polygon
        self.inside = inside
    def to_dict(self):
        point_count = len(self.polygon)
        test_success = check_point(self.point, self.polygon)==self.inside
        return {"point":self.point, "polygon":self.polygon, "inside":self.inside, "point_count":point_count, "test_success":test_success   }
def check_tests(tests):
    for test in tests:
        assert(check_point(test.point, test.polygon) == test.inside)

def read_tests(filename):
    with open(filename) as f:
        file = json.load(f)
    tests = []
    for test in file:
        pre_polygon = test["polygon"]
        polygon = []
        for point in pre_polygon:
            point = point["coord"]
            polygon.append(Point(point[0], point[1]))
        pre_point = test["point"]["coord"]
        point = Point(pre_point[0], pre_point[1])
        inside = test["inside"]
        test = Test(point, polygon, inside)
        tests.append(test)
    return tests
def create_test_table(tests):
    test_dicts = []
    for test in tests:
        test_dicts.append(test.to_dict())
    test_df = pd.DataFrame(test_dicts)
    test_df["inside"]=test_df["inside"].astype(int)
    test_df["test_success"]=test_df["test_success"].astype(int)
    return test_df

if __name__ == "__main__":
    tests = read_tests("testfile.json")
    check_tests(tests)
    print(create_test_table(tests))

