import os

for module in os.listdir(os.path.dirname(__file__) + "/katas"):
    __import__("katas." + module + ".solution_test", locals(), globals())

