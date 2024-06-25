import os


def main():
    for module in os.listdir(os.path.dirname(__file__) + "/katas"):
        __import__("katas." + module + ".solution_test", locals(), globals())

if __name__ == "__main__":
    main()
